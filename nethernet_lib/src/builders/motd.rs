use crate::error::Result;
use crate::protocol::packet::discovery::ServerData;




#[derive(Debug, Clone)]
pub struct NethernetMotd {
    server_data: ServerData,
}

impl NethernetMotd {
    
    
    
    
    
    
    
    
    
    
    pub fn new(server_name: impl Into<String>, level_name: impl Into<String>) -> Self {
        Self {
            server_data: ServerData::new(server_name.into(), level_name.into()),
        }
    }

    
    pub fn server_name(mut self, server_name: impl Into<String>) -> Self {
        self.server_data.server_name = server_name.into();
        self
    }

    
    pub fn level_name(mut self, level_name: impl Into<String>) -> Self {
        self.server_data.level_name = level_name.into();
        self
    }

    
    pub fn game_type(mut self, game_type: u8) -> Self {
        self.server_data.game_type = game_type;
        self
    }

    
    pub fn player_count(mut self, player_count: i32) -> Self {
        self.server_data.player_count = player_count;
        self
    }

    
    pub fn max_player_count(mut self, max_player_count: i32) -> Self {
        self.server_data.max_player_count = max_player_count;
        self
    }

    
    pub fn editor_world(mut self, editor_world: bool) -> Self {
        self.server_data.editor_world = editor_world;
        self
    }

    
    pub fn hardcore(mut self, hardcore: bool) -> Self {
        self.server_data.hardcore = hardcore;
        self
    }

    
    pub fn transport_layer(mut self, transport_layer: u8) -> Self {
        self.server_data.transport_layer = transport_layer;
        self
    }

    
    pub fn connection_type(mut self, connection_type: u8) -> Self {
        self.server_data.connection_type = connection_type;
        self
    }

    
    pub fn build(self) -> ServerData {
        self.server_data
    }

    
    pub fn marshal(self) -> Result<Vec<u8>> {
        self.server_data.marshal()
    }
}
