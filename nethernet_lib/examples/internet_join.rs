use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use nethernet::protocol::Signal;
use nethernet::{NethernetStream, Signaling};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize)] struct AuthReq { username: String, password: String }
#[derive(Deserialize)] struct AuthRes { token: Option<String> }
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignalMessage { #[serde(rename = "type")] msg_type: String, sender_id: String, payload: Value }

struct WsSignaling {
    network_id: String,
    tx_ws: mpsc::Sender<Message>,
    rx_signals: broadcast::Sender<Signal>,
}

impl Signaling for WsSignaling {
    async fn signal(&self, signal: Signal) -> nethernet::error::Result<()> {
        let msg = SignalMessage {
            msg_type: "webrtc_signal".to_string(), sender_id: self.network_id.clone(),
            payload: serde_json::json!({ "raw": signal.to_string() }),
        };
        let _ = self.tx_ws.send(Message::Text(serde_json::to_string(&msg).unwrap())).await;
        Ok(())
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let room_id = "global_room_7551";
    let base_url = "http:

    println!("🚀 [JOIN] Подключение к сигнальному серверу...");

    let client = Client::builder().no_proxy().build()?;
    let username = format!("join_{}", rand::random::<u16>());
    let _ = client.post(format!("{}/api/register", base_url)).json(&AuthReq { username: username.clone(), password: "123".into() }).send().await;
    let res: AuthRes = client.post(format!("{}/api/login", base_url)).json(&AuthReq { username: username.clone(), password: "123".into() }).send().await?.json().await?;
    let token = res.token.unwrap();

    let ws_url = format!("ws:
    let (ws_stream, _) = connect_async(&ws_url).await?;
    let (mut ws_write, mut ws_read) = ws_stream.split();

    let (tx_ws, mut rx_ws_internal) = mpsc::channel(100);
    let (tx_signals, _) = broadcast::channel(100);
    
    let signaling = Arc::new(WsSignaling {
        network_id: username.clone(), tx_ws: tx_ws.clone(), rx_signals: tx_signals.clone(),
    });

    tokio::spawn(async move { while let Some(msg) = rx_ws_internal.recv().await { let _ = ws_write.send(msg).await; } });

    let tx_signals_clone = tx_signals.clone();
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(txt))) = ws_read.next().await {
            if let Ok(msg) = serde_json::from_str::<SignalMessage>(&txt) {
                if msg.msg_type == "webrtc_signal" {
                    if let Some(raw) = msg.payload.get("raw").and_then(|v| v.as_str()) {
                        if let Ok(signal) = Signal::from_string(raw, msg.sender_id) {
                            let _ = tx_signals_clone.send(signal);
                        }
                    }
                }
            }
        }
    });

    
    let _ = tx_ws.send(Message::Text(serde_json::to_string(&SignalMessage {
        msg_type: "get_motd".into(), sender_id: username.clone(), payload: serde_json::json!({})
    }).unwrap())).await;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    
    let stream = Arc::new(NethernetStream::connect(signaling.clone(), "ANY".into(), "0.0.0.0:0".parse()?).await?);
    println!("✅ [OK] Туннель 7551 к хосту пробит!");

    
    let proxy_socket = Arc::new(UdpSocket::bind("0.0.0.0:19132").await?);
    let last_client_addr = Arc::new(Mutex::new(None::<SocketAddr>));

    println!("🎮 ВСЁ ГОТОВО! Заходи в Minecraft -> Друзья.");

    let s1 = stream.clone(); let g1 = proxy_socket.clone(); let cache = last_client_addr.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        loop {
            if let Ok((n, addr)) = g1.recv_from(&mut buf).await {
                *cache.lock().await = Some(addr);
                let _ = s1.send(Bytes::copy_from_slice(&buf[..n])).await;
            }
        }
    });

    let s2 = stream.clone(); let g2 = proxy_socket.clone(); let cache2 = last_client_addr.clone();
    loop {
        if let Ok(Some(data)) = s2.recv().await {
            if let Some(addr) = *cache2.lock().await { let _ = g2.send_to(&data, addr).await; }
        }
    }
}
