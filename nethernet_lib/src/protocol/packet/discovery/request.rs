




use super::packet::Packet;
use crate::error::Result;
use crate::protocol::constants::ID_REQUEST_PACKET;
use std::io::{Read, Write};



#[derive(Debug, Clone, Default)]
pub struct RequestPacket;

impl Packet for RequestPacket {
    
    fn id(&self) -> u16 {
        ID_REQUEST_PACKET
    }

    
    fn read(&mut self, _r: &mut dyn Read) -> Result<()> {
        
        Ok(())
    }

    
    fn write(&self, _w: &mut dyn Write) -> Result<()> {
        
        Ok(())
    }

    
    
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
