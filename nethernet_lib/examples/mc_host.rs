use nethernet::signaling::lan::LanSignaling;
use nethernet::{NethernetListener, ServerData, Signaling};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    tracing_subscriber::fmt::init();

    
    
    
    let mc_world_port = 19132; 
    let mc_addr: SocketAddr = format!("127.0.0.1:{}", mc_world_port).parse()?;
    

    
    let server_data = ServerData::new(
        "§l§6NetherNet §bHost".to_string(),
        format!("Порт игры: {}", mc_world_port),
    );

    let network_id = rand::random::<u64>();
    
    
    let bind_addr: SocketAddr = "0.0.0.0:7552".parse()?;
    let signaling = LanSignaling::new(network_id, bind_addr).await?;
    
    
    signaling.set_pong_data(server_data.marshal()?);

    
    let mut listener = NethernetListener::bind(signaling, bind_addr).await?;
    tracing::info!("🚀 ХОСТ: Запущен на 7552. Ждем игрока...");
    tracing::info!("👉 Убедись, что мир в Minecraft открыт на порту {}", mc_world_port);

    loop {
        match listener.accept().await {
            Ok(session) => {
                tracing::info!("🔗 Соединение установлено! Начинаем проброс пакетов...");
                
                let session = Arc::new(session);
                
                let game_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);

                
                let s1 = session.clone();
                let g1 = game_socket.clone();
                tokio::spawn(async move {
                    while let Ok(Some(data)) = s1.recv().await {
                        let _ = g1.send_to(&data, mc_addr).await;
                    }
                });

                
                let s2 = session.clone();
                let g2 = game_socket.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 4096];
                    loop {
                        if let Ok((n, _)) = g2.recv_from(&mut buf).await {
                            let _ = s2.send(Bytes::copy_from_slice(&buf[..n])).await;
                        }
                    }
                });
            }
            Err(e) => tracing::error!("Ошибка принятия соединения: {}", e),
        }
    }
}
