




use crate::error::{NethernetError, Result};
use crate::protocol::types::{
    read_bytes_u8, read_i32_le, read_u8, write_bytes_u8, write_i32_le, write_u8,
};
use std::io::Cursor;


const VERSION: u8 = 4;


#[derive(Debug, Clone)]
pub struct ServerData {
    
    pub server_name: String,
    
    pub level_name: String,
    
    pub game_type: u8,
    
    pub player_count: i32,
    
    pub max_player_count: i32,
    
    pub editor_world: bool,
    
    pub hardcore: bool,
    
    pub transport_layer: u8,
    
    pub connection_type: u8,
}

impl ServerData {
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn new(server_name: String, level_name: String) -> Self {
        Self {
            server_name,
            level_name,
            game_type: 0,
            player_count: 1,
            max_player_count: 8,
            editor_world: false,
            hardcore: false,
            transport_layer: 2, 
            connection_type: 4, 
        }
    }

    
    
    
    
    
    pub fn marshal(&self) -> Result<Vec<u8>> {
        
        if self.game_type >= 128 {
            return Err(NethernetError::Other(format!(
                "game_type must be less than 128 to avoid overflow, got {}",
                self.game_type
            )));
        }
        if self.transport_layer >= 128 {
            return Err(NethernetError::Other(format!(
                "transport_layer must be less than 128 to avoid overflow, got {}",
                self.transport_layer
            )));
        }
        if self.connection_type >= 128 {
            return Err(NethernetError::Other(format!(
                "connection_type must be less than 128 to avoid overflow, got {}",
                self.connection_type
            )));
        }

        let mut buf = Vec::new();

        
        write_u8(&mut buf, VERSION)?;

        
        write_bytes_u8(&mut buf, self.server_name.as_bytes())?;

        
        write_bytes_u8(&mut buf, self.level_name.as_bytes())?;

        
        write_u8(&mut buf, self.game_type << 1)?;

        
        write_i32_le(&mut buf, self.player_count)?;
        write_i32_le(&mut buf, self.max_player_count)?;

        
        write_u8(&mut buf, if self.editor_world { 1 } else { 0 })?;
        write_u8(&mut buf, if self.hardcore { 1 } else { 0 })?;

        
        write_u8(&mut buf, self.transport_layer << 1)?;
        write_u8(&mut buf, self.connection_type << 1)?;

        Ok(buf)
    }

    
    
    
    
    
    
    
    pub fn unmarshal(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        
        let version = read_u8(&mut cursor)?;
        if version != VERSION {
            return Err(NethernetError::Other(format!(
                "version mismatch: got {}, want {}",
                version, VERSION
            )));
        }

        
        let server_name_bytes = read_bytes_u8(&mut cursor)?;
        let server_name = String::from_utf8(server_name_bytes)
            .map_err(|e| NethernetError::Other(format!("invalid server name UTF-8: {}", e)))?;

        
        let level_name_bytes = read_bytes_u8(&mut cursor)?;
        let level_name = String::from_utf8(level_name_bytes)
            .map_err(|e| NethernetError::Other(format!("invalid level name UTF-8: {}", e)))?;

        
        let game_type = read_u8(&mut cursor)? >> 1;

        
        let player_count = read_i32_le(&mut cursor)?;
        let max_player_count = read_i32_le(&mut cursor)?;

        
        let editor_world = read_u8(&mut cursor)? != 0;
        let hardcore = read_u8(&mut cursor)? != 0;

        
        let transport_layer = read_u8(&mut cursor)? >> 1;
        let connection_type = read_u8(&mut cursor)? >> 1;

        
        let remaining = data.len() - cursor.position() as usize;
        if remaining != 0 {
            return Err(NethernetError::Other(format!("unread {} bytes", remaining)));
        }

        Ok(Self {
            server_name,
            level_name,
            game_type,
            player_count,
            max_player_count,
            editor_world,
            hardcore,
            transport_layer,
            connection_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_data_roundtrip() {
        let original = ServerData {
            server_name: "Test Server".to_string(),
            level_name: "My World".to_string(),
            game_type: 0,
            player_count: 3,
            max_player_count: 10,
            editor_world: false,
            hardcore: false,
            transport_layer: 2,
            connection_type: 4,
        };

        let encoded = original.marshal().unwrap();
        let decoded = ServerData::unmarshal(&encoded).unwrap();

        assert_eq!(original.server_name, decoded.server_name);
        assert_eq!(original.level_name, decoded.level_name);
        assert_eq!(original.game_type, decoded.game_type);
        assert_eq!(original.player_count, decoded.player_count);
        assert_eq!(original.max_player_count, decoded.max_player_count);
        assert_eq!(original.editor_world, decoded.editor_world);
        assert_eq!(original.hardcore, decoded.hardcore);
        assert_eq!(original.transport_layer, decoded.transport_layer);
        assert_eq!(original.connection_type, decoded.connection_type);
    }

    #[test]
    fn test_version_mismatch() {
        let data = vec![5]; 
        let result = ServerData::unmarshal(&data);
        assert!(result.is_err());
    }
}
