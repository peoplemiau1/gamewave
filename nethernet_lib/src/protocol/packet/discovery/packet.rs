

use super::crypto::{compute_checksum, decrypt, encrypt, verify_checksum};
use super::{MessagePacket, RequestPacket, ResponsePacket};
use crate::error::{NethernetError, Result};
use crate::protocol::constants::{ID_MESSAGE_PACKET, ID_REQUEST_PACKET, ID_RESPONSE_PACKET};
use crate::protocol::types::{U16LE, U64LE};
use std::io::{Cursor, Read, Write};


pub trait Packet: Send + Sync {
    
    fn id(&self) -> u16;

    
    fn read(&mut self, r: &mut dyn Read) -> Result<()>;

    
    fn write(&self, w: &mut dyn Write) -> Result<()>;

    
    fn as_any(&self) -> &dyn std::any::Any;
}


#[derive(Debug, Clone)]
pub struct Header {
    
    pub packet_id: u16,
    
    pub sender_id: u64,
}

impl Header {
    
    
    
    
    pub fn read(r: &mut dyn Read) -> Result<Self> {
        let packet_id = U16LE::read(r)?.0;
        let sender_id = U64LE::read(r)?.0;

        
        let mut padding = [0u8; 8];
        r.read_exact(&mut padding)?;

        Ok(Self {
            packet_id,
            sender_id,
        })
    }

    
    pub fn write(&self, w: &mut dyn Write) -> Result<()> {
        U16LE(self.packet_id).write(w)?;
        U64LE(self.sender_id).write(w)?;
        
        w.write_all(&[0u8; 8])?;
        Ok(())
    }
}











pub fn marshal(packet: &dyn Packet, sender_id: u64) -> Result<Vec<u8>> {
    
    
    let mut buf = Vec::with_capacity(32 + 2 + 18 + 64 + 16);

    
    buf.extend_from_slice(&[0u8; 32]);

    
    buf.extend_from_slice(&[0u8; 2]);

    
    let header = Header {
        packet_id: packet.id(),
        sender_id,
    };
    header.write(&mut buf)?;

    
    packet.write(&mut buf)?;

    
    let total_len = buf.len();
    if total_len - 32 > u16::MAX as usize {
        return Err(NethernetError::MessageTooLarge(total_len - 32));
    }

    let data_len = (total_len - 32 - 2) as u16;
    buf[32..34].copy_from_slice(&data_len.to_le_bytes());

    
    let checksum = compute_checksum(&buf[32..]);
    buf[..32].copy_from_slice(&checksum);

    
    
    
    

    
    
    
    

    let mut payload = buf.split_off(32);
    encrypt(&mut payload)?;

    buf.extend_from_slice(&payload);

    Ok(buf)
}












pub fn unmarshal(data: &[u8]) -> Result<(Box<dyn Packet>, u64)> {
    if data.len() < 32 {
        return Err(NethernetError::Other("packet too short".to_string()));
    }

    
    let checksum: [u8; 32] = data[..32].try_into().unwrap();

    
    let mut payload = data[32..].to_vec();
    decrypt(&mut payload)?;

    
    if !verify_checksum(&payload, &checksum) {
        return Err(NethernetError::Other("checksum mismatch".to_string()));
    }

    let mut cursor = Cursor::new(payload);

    
    let _length = U16LE::read(&mut cursor)?;

    
    let header = Header::read(&mut cursor)?;

    
    let mut packet: Box<dyn Packet> = match header.packet_id {
        ID_REQUEST_PACKET => Box::new(RequestPacket),
        ID_RESPONSE_PACKET => Box::new(ResponsePacket::default()),
        ID_MESSAGE_PACKET => Box::new(MessagePacket::default()),
        _ => {
            return Err(NethernetError::Other(format!(
                "unknown packet ID: {}",
                header.packet_id
            )));
        }
    };

    
    packet.read(&mut cursor)?;

    
    let cursor_position = cursor.position() as usize;
    let payload_len = cursor.get_ref().len();
    if cursor_position < payload_len {
        let remaining = payload_len - cursor_position;
        return Err(NethernetError::Other(format!(
            "trailing data in packet: {} remaining bytes out of {} total payload bytes",
            remaining, payload_len
        )));
    }

    Ok((packet, header.sender_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_roundtrip() {
        let header = Header {
            packet_id: 0x01,
            sender_id: 0x1234567890abcdef,
        };

        let mut buf = Vec::new();
        header.write(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = Header::read(&mut cursor).unwrap();

        assert_eq!(header.packet_id, decoded.packet_id);
        assert_eq!(header.sender_id, decoded.sender_id);
    }
}
