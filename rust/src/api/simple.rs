
use crate::logic;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn run_host(room_id: String) -> String {
    
    logic::host_task(room_id).await
}

pub async fn run_join(room_id: String) -> String {
    format!("Joiner для комнаты {} в пути...", room_id)
}
