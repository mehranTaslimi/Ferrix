use crate::registry::Registry;

use anyhow::anyhow;
use rand::{rngs::OsRng, TryRngCore};
use std::sync::Arc;
use tauri_plugin_http::reqwest::Client;
use tokio_util::bytes::Bytes;
use urlencoding::encode_binary;

use super::*;

impl Torrent {
    fn azureus_yyyy_version() -> String {
        let app_handle = Arc::clone(&Registry::get_state().app_handle);
        let version = app_handle.package_info().version.to_string();

        let parts = version
            .split(".")
            .map(|f| f.parse::<u64>().unwrap_or(0))
            .collect::<Vec<_>>();

        let major = parts[0].min(9);
        let minor = parts[1].min(99);
        let patch = parts[2].min(9);

        format!("{major}{:02}{patch}", minor)
    }

    pub(super) async fn generate_peer_id(&self) -> anyhow::Result<[u8; 20]> {
        let peer_id_arc = Arc::clone(&Registry::get_state().peer_id);
        let mut maybe_peer_id_lock = peer_id_arc.lock().await;

        match *maybe_peer_id_lock {
            Some(id) => Ok(id),
            None => {
                let prefix = format!("-FX{}-", Self::azureus_yyyy_version());
                let prefix_bytes = prefix.as_bytes();
                let alph = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

                let mut id = [0u8; 20];
                id[..8].copy_from_slice(prefix_bytes);

                for i in 8..20 {
                    let idx = OsRng.try_next_u32()? as usize % alph.len();
                    id[i] = alph[idx];
                }

                *maybe_peer_id_lock = Some(id);

                Ok(id)
            }
        }
    }

    pub async fn http_announce(
        self: &Arc<Self>,
        announce_url: &str,
        length: i64,
    ) -> anyhow::Result<Bytes> {
        let peer_id = self.generate_peer_id().await?;

        let encoded_info_hash = encode_binary(&self.info_hash);
        let encoded_peer_id = encode_binary(&peer_id);

        let client = Client::new();

        let sep = if announce_url.contains('?') { '&' } else { '?' };

        let url = format!("{base}{sep}info_hash={ih}&peer_id={pid}&port=6881&uploaded=0&downloaded=0&left={left}&compact=1&event=started",
                base = announce_url,
                sep = sep,
                ih = encoded_info_hash,
                pid = encoded_peer_id,
                left = length
            );

        let response = client.get(url).send().await?;
        let status = response.status();
        let body = response.bytes().await?;

        if !status.is_success() {
            return Err(anyhow!("tracker http error: {}", status));
        }

        Ok(body)
    }
}
