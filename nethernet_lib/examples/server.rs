






use nethernet::signaling::lan::LanSignaling;
use nethernet::{NethernetListener, ServerData, Signaling};
use std::net::SocketAddr;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let fmt_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);

    let filter_layer = filter::LevelFilter::from_level(Level::TRACE);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();

    
    let server_data = ServerData::new(
        "My NetherNet Server".to_string(),
        "Example World".to_string(),
    );

    
    
    let network_id = rand::random::<u64>();
    let bind_addr: SocketAddr = "0.0.0.0:7551".parse()?;

    let signaling = LanSignaling::new(network_id, bind_addr).await?;

    
    signaling.set_pong_data(server_data.marshal()?);

    tracing::info!("NetherNet server starting");
    tracing::info!("   Network ID: {}", network_id);
    tracing::info!("   Listening on: {}", bind_addr);
    tracing::info!("   Broadcasting discovery responses...");

    
    let mut listener = NethernetListener::bind(signaling, bind_addr).await?;
    tracing::info!("✅ Server ready and responding to LAN discovery");

    
    loop {
        match listener.accept().await {
            Ok(session) => {
                tracing::info!("🔗 New client connected");

                
                tokio::spawn(async move {
                    let mut packet_count = 0;

                    loop {
                        match session.recv().await {
                            Ok(Some(data)) => {
                                packet_count += 1;
                                
                                if let Err(e) = session.send(data).await {
                                    tracing::error!("Failed to send packet: {}", e);
                                    break;
                                }
                            }
                            Ok(None) => {
                                tracing::info!("Client disconnected gracefully");
                                break;
                            }
                            Err(e) => {
                                tracing::error!("Error receiving packet: {}", e);
                                break;
                            }
                        }
                    }

                    tracing::info!("Client session ended ({} packets received)", packet_count);
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}
