use crate::error::Result;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// Compute fast partial hash of a file
/// Uses: file size + first 1MB + middle 64KB + last 64KB + full path
/// This is much faster than full file hash while still being unique enough
pub fn compute_file_hash(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    // Hash the full path and file size
    hasher.update(path.to_string_lossy().as_bytes());
    hasher.update(file_size.to_le_bytes());

    // Read and hash first 1MB (or entire file if smaller)
    let first_chunk_size = std::cmp::min(1024 * 1024, file_size); // 1MB or less
    let mut buffer = vec![0u8; first_chunk_size as usize];
    reader.read_exact(&mut buffer)?;
    hasher.update(&buffer);

    // If file is large enough, also hash middle and end chunks
    if file_size > 2 * 1024 * 1024 {
        // Hash middle 64KB
        let middle_pos = file_size / 2 - 32 * 1024;
        reader.seek(SeekFrom::Start(middle_pos))?;
        let mut middle_buffer = vec![0u8; 64 * 1024];
        reader.read_exact(&mut middle_buffer)?;
        hasher.update(&middle_buffer);

        // Hash last 64KB
        let end_pos = file_size - 64 * 1024;
        reader.seek(SeekFrom::Start(end_pos))?;
        let mut end_buffer = vec![0u8; 64 * 1024];
        reader.read_exact(&mut end_buffer)?;
        hasher.update(&end_buffer);
    }

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

/// Compute SHA256 hash of a string
pub fn compute_string_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hash() {
        let hash1 = compute_string_hash("test");
        let hash2 = compute_string_hash("test");
        let hash3 = compute_string_hash("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA256 produces 64 hex characters
    }
}
