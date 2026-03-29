



use super::packet::Packet;
use crate::error::Result;
use crate::protocol::constants::ID_RESPONSE_PACKET;
use crate::protocol::types::{U32LE, read_bytes_u32};
use std::io::{Read, Write};



#[derive(Debug, Clone, Default)]
pub struct ResponsePacket {
    
    pub application_data: Vec<u8>,
}

impl ResponsePacket {
    
    pub fn new(application_data: Vec<u8>) -> Self {
        Self { application_data }
    }
}

impl Packet for ResponsePacket {
    
    fn id(&self) -> u16 {
        ID_RESPONSE_PACKET
    }

    
    fn read(&mut self, r: &mut dyn Read) -> Result<()> {
        
        let hex_data = read_bytes_u32(r)?;

        
        self.application_data = hex::decode(&hex_data)
            .map_err(|e| crate::error::NethernetError::Other(format!("hex decode error: {}", e)))?;

        Ok(())
    }

    
    fn write(&self, w: &mut dyn Write) -> Result<()> {
        
        let len = self.application_data.len();
        let hex_len = len * 2;

        
        
        U32LE(hex_len as u32).write(w)?;

        
        let mut buf = [0u8; 2048]; 
        for chunk in self.application_data.chunks(512) {
            let encoded_len = chunk.len() * 2;
            hex::encode_to_slice(chunk, &mut buf[..encoded_len]).map_err(|e| {
                crate::error::NethernetError::Other(format!("hex encode error: {}", e))
            })?;
            w.write_all(&buf[..encoded_len])?;
        }
        Ok(())
    }

    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
