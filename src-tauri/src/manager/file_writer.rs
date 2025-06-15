use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::{fs::OpenOptions, spawn, sync::mpsc};

pub type WriteMessage = (u64, Vec<u8>);

pub async fn file_writer(
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
