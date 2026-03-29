use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use nethernet::protocol::Signal;
use nethernet::{NethernetListener, Signaling};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize)] struct AuthReq { username: String, password: String }
#[derive(Deserialize)] struct AuthRes { token: Option<String> }
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignalMessage {
    #[serde(rename = "type")] msg_type: String,
    sender_id: String,
    pub payload: Value,
}

struct WsSignaling {
    network_id: String,
    tx_ws: mpsc::Sender<Message>,
    rx_signals: broadcast::Sender<Signal>,
}

impl Signaling for WsSignaling {
    async fn signal(&self, signal: Signal) -> nethernet::error::Result<()> {
        let msg = SignalMessage {
            msg_type: "webrtc_signal".to_string(),
            sender_id: self.network_id.clone(),
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
    tracing_subscriber::fmt().init();
    
    // МАЙНКРАФТ ВСЕГДА НА 7551
    let mc_addr: SocketAddr = "127.0.0.1:7551".parse()?;
    let room_id = "global_room_7551"; 
    let base_url = "http://e1.aurorix.net:20833";

    println!("🚀 [HOST] Запуск туннеля NetherNet (Порт 7551)");

    let client = Client::builder().no_proxy().build()?;
    let username = format!("host_{}", rand::random::<u16>());
    
    // Регистрация и Логин
    let _ = client.post(format!("{}/api/register", base_url))
        .json(&AuthReq { username: username.clone(), password: "123".into() }).send().await;
    let res: AuthRes = client.post(format!("{}/api/login", base_url))
        .json(&AuthReq { username: username.clone(), password: "123".into() }).send().await?.json().await?;
    let token = res.token.ok_or("❌ Ошибка авторизации")?;

    // WebSocket
    let ws_url = format!("ws://e1.aurorix.net:20833/ws/{}?token={}", room_id, token);
    let (ws_stream, _) = connect_async(&ws_url).await?;
    let (mut ws_write, mut ws_read) = ws_stream.split();

    let (tx_ws, mut rx_ws_internal) = mpsc::channel(100);
    let (tx_signals, _) = broadcast::channel(100);
    
    let tx_signals_ws = tx_signals.clone();
    let host_name = username.clone();
    let tx_ws_internal = tx_ws.clone();

    let signaling = WsSignaling {
        network_id: username.clone(), tx_ws: tx_ws.clone(), rx_signals: tx_signals.clone(),
    };

    // Фоновая отправка в WS
    tokio::spawn(async move {
        while let Some(msg) = rx_ws_internal.recv().await { let _ = ws_write.send(msg).await; }
    });

    // Фоновый прием из WS
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(txt))) = ws_read.next().await {
            if let Ok(msg) = serde_json::from_str::<SignalMessage>(&txt) {
                if msg.msg_type == "webrtc_signal" {
                    if let Some(raw) = msg.payload.get("raw").and_then(|v| v.as_str()) {
                        if let Ok(signal) = Signal::from_string(raw, msg.sender_id) {
                            let _ = tx_signals_ws.send(signal);
                        }
                    }
                } else if msg.msg_type == "get_motd" {
                    let motd_msg = SignalMessage {
                        msg_type: "motd".into(), sender_id: host_name.clone(),
                        payload: serde_json::json!({"name": "§l§eNetherNet §bGlobal", "players": 1}),
                    };
                    let _ = tx_ws_internal.send(Message::Text(serde_json::to_string(&motd_msg).unwrap())).await;
                }
            }
        }
    });

    // Слушаем WebRTC (на любом свободном порту, сигналы идут через WS)
    let mut listener = NethernetListener::bind(signaling, "0.0.0.0:0".parse()?).await?;
    println!("📡 [READY] Ждем игрока через интернет...");

    while let Ok(session) = listener.accept().await {
        println!("🔥 [CONN] Игрок подключился!");
        let game_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
        let s1 = session.clone(); let g1 = game_socket.clone();
        tokio::spawn(async move {
            while let Ok(Some(data)) = s1.recv().await { let _ = g1.send_to(&data, mc_addr).await; }
        });
        let s2 = session.clone(); let g2 = game_socket.clone();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 4096];
            loop {
                if let Ok((n, _)) = g2.recv_from(&mut buf).await { let _ = s2.send(Bytes::copy_from_slice(&buf[..n])).await; }
            }
        });
    }
    Ok(())
}
