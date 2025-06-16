use md5::{Digest, Md5};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

pub async fn compute_partial_hash(
    file_path: &str,
    start_byte: u64,
    downloaded_bytes: u64,
) -> Result<String, String> {
    let mut file = File::open(file_path).await.map_err(|e| e.to_string())?;
    file.seek(SeekFrom::Start(start_byte))
        .await
        .map_err(|e| e.to_string())?;

    let mut hasher = Md5::new();
    let mut remaining = downloaded_bytes as usize;
    let mut buffer = vec![0; 8192];

    while remaining > 0 {
        let read_size = std::cmp::min(buffer.len(), remaining);
        let n = file
            .read(&mut buffer[..read_size])
            .await
            .map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
        remaining -= n;
    }

    Ok(format!("{:x}", hasher.finalize()))
}
