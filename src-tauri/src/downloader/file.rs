use md5::{Digest, Md5};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::{fs::OpenOptions, spawn, sync::mpsc};

pub type WriteMessage = (u64, Vec<u8>);

pub async fn setup_file_write(
    file_path: &str,
    total_bytes: u64,
) -> Result<mpsc::Sender<WriteMessage>, String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .await
        .map_err(|e| e.to_string())?;

    file.set_len(total_bytes as u64)
        .await
        .map_err(|e| e.to_string())?;

    let (tx, mut rx) = mpsc::channel::<WriteMessage>(100);

    spawn(async move {
        while let Some((start, bytes)) = rx.recv().await {
            if let Err(e) = file.seek(SeekFrom::Start(start)).await {
                eprintln!("file seek error: {}", e);
                break;
            };

            if let Err(e) = file.write_all(&bytes).await {
                eprintln!("file write error: {}", e);
                break;
            }
        }
    });

    Ok(tx)
}

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
