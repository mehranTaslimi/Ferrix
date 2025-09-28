use crate::client::Client;
use crate::torrent::Torrent;

use super::super::Registry;
use super::*;

pub trait TorrentActions {
    async fn decode_torrent_file(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()>;
}

impl TorrentActions for Registry {
    async fn decode_torrent_file(
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

        let t = Torrent::from_bytes(&bytes)?;

        t.http_announce().await?;

        Ok(())
    }
}
