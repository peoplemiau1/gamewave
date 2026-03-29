use nethernet::NethernetStream;
use nethernet::signaling::lan::LanSignaling;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use bytes::Bytes;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let network_id = rand::random::<u64>();
    
    let signaling = Arc::new(LanSignaling::new(network_id, "0.0.0.0:0".parse()?).await?);

    tracing::info!("🔎 Ищем хоста через порт 7551...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let servers = signaling.discover().await;
    let (server_id, _) = servers.iter().next().ok_or("Хост не найден! Убедись, что mc_host запущен.")?;
    let server_addr = signaling.get_address(*server_id).await.unwrap();

    let stream = Arc::new(NethernetStream::connect(signaling.clone(), server_id.to_string(), server_addr).await?);
    tracing::info!("✅ Туннель к хосту пробит!");

    
    let proxy_socket = Arc::new(UdpSocket::bind("0.0.0.0:19132").await?);
    let last_client_addr = Arc::new(Mutex::new(None::<SocketAddr>));

    tracing::info!("🎮 Теперь заходи в Майнкрафт -> Друзья. Там появится сервер!");

    
    let s1 = stream.clone();
    let g1 = proxy_socket.clone();
    let addr_cache = last_client_addr.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        loop {
            if let Ok((n, addr)) = g1.recv_from(&mut buf).await {
                *addr_cache.lock().await = Some(addr);
                let _ = s1.send(Bytes::copy_from_slice(&buf[..n])).await;
            }
        }
    });

    
    let s2 = stream.clone();
    let g2 = proxy_socket.clone();
    let addr_cache2 = last_client_addr.clone();
    loop {
        if let Ok(Some(data)) = s2.recv().await {
            if let Some(addr) = *addr_cache2.lock().await {
                let _ = g2.send_to(&data, addr).await;
            }
        }
    }
}
