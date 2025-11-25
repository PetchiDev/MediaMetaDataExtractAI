// Hash utilities for deduplication
// I-FR-02: No duplication of media assets in storage

use sha2::{Digest, Sha256};
use std::io::Read;

pub fn calculate_file_hash<R: Read>(mut reader: R) -> anyhow::Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];
    
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

pub async fn calculate_file_hash_async(
    data: &[u8],
) -> anyhow::Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    Ok(format!("{:x}", hasher.finalize()))
}
