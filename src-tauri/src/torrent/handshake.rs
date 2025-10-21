use std::time::Duration;

use anyhow::anyhow;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};

use super::*;

impl Torrent {
    pub async fn handshake<T>(&self, addr: T, ih: &[u8; 20], pid: &[u8; 20]) -> anyhow::Result<()>
    where
        T: ToSocketAddrs,
    {
        let mut stream = TcpStream::connect(addr).await?;

        let mut bytes = [0u8; 68];

        bytes[0] = 19;

        bytes[1..20].copy_from_slice(b"BitTorrent protocol");

        bytes[28..48].copy_from_slice(ih);

        bytes[48..68].copy_from_slice(pid);

        stream.write_all(&bytes).await?;

        let mut buf = [0u8; 68];

        timeout(Duration::from_secs(5), stream.read_exact(&mut buf)).await??;

        if buf[0] != 19 {
            return Err(anyhow!("invalid protocol length"));
        }

        if &buf[1..20] != b"BitTorrent protocol" {
            return Err(anyhow!("invalid protocol pstr"));
        }

        if &buf[28..48] != ih {
            return Err(anyhow!("invalid info hash"));
        }

        Ok(())
    }
}
