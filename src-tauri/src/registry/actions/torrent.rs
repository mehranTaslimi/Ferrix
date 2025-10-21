use crate::client::Client;
use crate::dispatch;
use crate::torrent::Torrent;

use super::super::Registry;
use super::*;

pub trait TorrentActions {
    async fn decode_torrent(opt_id: String, bytes: Vec<u8>) -> anyhow::Result<()>;
    async fn load_torrent(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()>;
    async fn send_announce_request(torrent_id: i64) -> anyhow::Result<()>;
}

impl TorrentActions for Registry {
    async fn decode_torrent(opt_id: String, bytes: Vec<u8>) -> anyhow::Result<()> {
        let t = Torrent::from_bytes(&bytes)?;

        t.get_tracker().await?;

        // println!("{:?}", t);

        Ok(())
    }

    async fn send_announce_request(torrent_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn load_torrent(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()> {
        let client = Client::new(
            &url,
            &options.auth,
            &options.proxy,
            &options.headers,
            &options.cookies,
        )?;

        let bytes = client.fetch_bytes().await?;

        dispatch!(
            registry,
            DecodeTorrent {
                opt_id,
                bytes: bytes.to_vec()
            }
        )?;

        Ok(())
    }
}
