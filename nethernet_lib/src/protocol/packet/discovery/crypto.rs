




use crate::error::{NethernetError, Result};
use aes::Aes256;
use aes::cipher::{Block, BlockDecrypt, BlockEncrypt, KeyInit};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::sync::LazyLock;




static ENCRYPTION_KEY: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let mut hasher = Sha256::new();
    hasher.update(0xdeadbeef_u64.to_le_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
});


static CIPHER: LazyLock<Aes256> = LazyLock::new(|| Aes256::new(ENCRYPTION_KEY.as_slice().into()));


static HMAC_STATE: LazyLock<Hmac<Sha256>> = LazyLock::new(|| {
    <Hmac<Sha256> as Mac>::new_from_slice(ENCRYPTION_KEY.as_slice())
        .expect("HMAC can take key of any size")
});




pub(crate) fn encrypt(buf: &mut Vec<u8>) -> Result<()> {
    
    let block_size = 16;
    let data_len = buf.len();
    let padding_len = block_size - (data_len % block_size);
    buf.resize(data_len + padding_len, padding_len as u8);

    
    
    let blocks = unsafe {
        std::slice::from_raw_parts_mut(
            buf.as_mut_ptr() as *mut Block<Aes256>,
            buf.len() / block_size,
        )
    };
    CIPHER.encrypt_blocks(blocks);

    Ok(())
}




pub(crate) fn decrypt(buf: &mut Vec<u8>) -> Result<()> {
    if buf.is_empty() || buf.len() % 16 != 0 {
        return Err(NethernetError::Other(
            "Invalid encrypted data length".to_string(),
        ));
    }

    
    
    let blocks = unsafe {
        std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut Block<Aes256>, buf.len() / 16)
    };
    CIPHER.decrypt_blocks(blocks);

    
    if let Some(&padding_len) = buf.last() {
        if padding_len > 0 && padding_len <= 16 {
            let data_len = buf.len();
            if data_len >= padding_len as usize {
                
                let padding_start = data_len - padding_len as usize;
                let mut mismatched: u8 = 0;
                for &byte in &buf[padding_start..] {
                    mismatched |= byte ^ padding_len;
                }
                if mismatched == 0 {
                    buf.truncate(padding_start);
                    return Ok(());
                }
            }
        }
    }

    Err(NethernetError::Other("Invalid padding".to_string()))
}




pub(crate) fn compute_checksum(data: &[u8]) -> [u8; 32] {
    let mut mac = HMAC_STATE.clone();
    mac.update(data);
    let result = mac.finalize();
    result.into_bytes().into()
}






pub(crate) fn verify_checksum(data: &[u8], expected: &[u8; 32]) -> bool {
    let mut mac = HMAC_STATE.clone();
    mac.update(data);
    mac.verify_slice(expected).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = b"Hello, NetherNet!";
        let mut buf = data.to_vec();
        encrypt(&mut buf).unwrap();
        decrypt(&mut buf).unwrap();
        assert_eq!(data.as_slice(), buf.as_slice());
    }

    #[test]
    fn test_checksum() {
        let data = b"Test data for checksum";
        let checksum = compute_checksum(data);
        assert!(verify_checksum(data, &checksum));

        let wrong_checksum = [0u8; 32];
        assert!(!verify_checksum(data, &wrong_checksum));
    }
}
