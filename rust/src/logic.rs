use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use nethernet::protocol::Signal;
use nethernet::{NethernetListener, Signaling};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)] struct AuthReq { username: String, password: String }
#[derive(Deserialize, Clone)] struct AuthRes { token: Option<String> }

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignalMessage { 
    #[serde(rename = "type")] msg_type: String, 
    sender_id: String, 
    payload: String, 
}

struct WsSignaling {
    network_id: String,
    tx_ws: mpsc::Sender<Message>,
    rx_signals: broadcast::Sender<Signal>,
}

impl Signaling for WsSignaling {
    fn signal(&self, signal: Signal) -> impl std::future::Future<Output = nethernet::error::Result<()>> + Send {
        let payload_obj = serde_json::json!({ "raw": signal.to_string() });
        let msg = SignalMessage {
            msg_type: "webrtc_signal".to_string(),
            sender_id: self.network_id.clone(),
            payload: payload_obj.to_string(),
        };
        let tx = self.tx_ws.clone();
        async move {
            let _ = tx.send(Message::Text(serde_json::to_string(&msg).unwrap())).await;
            Ok(())
        }
    }
    fn signals(&self) -> Pin<Box<dyn futures_util::Stream<Item = Signal> + Send>> {
        let rx = self.rx_signals.subscribe();
        Box::pin(futures_util::stream::unfold(rx, |mut rx| async move {
            match rx.recv().await { Ok(sig) => Some((sig, rx)), Err(_) => None }
        }))
    }
    fn network_id(&self) -> String { self.network_id.clone() }
    fn set_pong_data(&self, _data: Vec<u8>) {}
}

pub async fn host_task(room_id: String) -> String {
    let mc_addr: SocketAddr = "127.0.0.1:7551".parse().unwrap();
    let base_url = "http://e1.aurorix.net:20833";
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    let username = format!("host_{}", rand::random::<u16>());
    let _ = client.post(format!("{}/api/register", base_url)).json(&AuthReq { username: username.clone(), password: "123".into() }).send().await;
    let res: AuthRes = client.post(format!("{}/api/login", base_url)).json(&AuthReq { username: username.clone(), password: "123".into() }).send().await.unwrap().json().await.unwrap();
    let token = res.token.unwrap();
    let ws_url = format!("ws://e1.aurorix.net:20833/ws/{}?token={}", room_id, token);
    let (ws_stream, _) = connect_async(&ws_url).await.unwrap();
    let (mut ws_write, mut ws_read) = ws_stream.split();
    let (tx_ws, mut rx_ws_internal) = mpsc::channel(100);
    let (tx_signals, _) = broadcast::channel(100);
    let tx_signals_ws = tx_signals.clone();
    let signaling = WsSignaling { network_id: username.clone(), tx_ws: tx_ws.clone(), rx_signals: tx_signals.clone() };
    tokio::spawn(async move { while let Some(msg) = rx_ws_internal.recv().await { let _ = ws_write.send(msg).await; } });
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(txt))) = ws_read.next().await {
            if let Ok(msg) = serde_json::from_str::<SignalMessage>(&txt) {
                if msg.msg_type == "webrtc_signal" {
                    if let Ok(p_json) = serde_json::from_str::<serde_json::Value>(&msg.payload) {
                        if let Some(raw) = p_json.get("raw").and_then(|v| v.as_str()) {
                            if let Ok(signal) = Signal::from_string(raw, msg.sender_id) { let _ = tx_signals_ws.send(signal); }
                        }
                    }
                }
            }
        }
    });
    let mut listener = NethernetListener::bind(signaling, "0.0.0.0:0".parse().unwrap()).await.unwrap();
    tokio::spawn(async move {
        while let Ok(session) = listener.accept().await {
            let game_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());
            let s1 = session.clone(); let g1 = game_socket.clone();
            tokio::spawn(async move { while let Ok(Some(data)) = s1.recv().await { let _ = g1.send_to(&data, mc_addr).await; } });
            let s2 = session.clone(); let g2 = game_socket.clone();
            tokio::spawn(async move {
                let mut buf = vec![0u8; 4096];
                loop { if let Ok((n, _)) = g2.recv_from(&mut buf).await { let _ = s2.send(Bytes::copy_from_slice(&buf[..n])).await; } }
            });
        }
    });
    format!("✅ Хост запущен в комнате {}", room_id)
}
