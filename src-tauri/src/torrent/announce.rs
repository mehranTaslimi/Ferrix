use std::sync::Arc;

use crate::registry::Registry;
use rand::{rngs::OsRng, TryRngCore};
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

    async fn generate_peer_id() -> anyhow::Result<[u8; 20]> {
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

    pub async fn http_announce(&self) -> anyhow::Result<()> {
        let peer_id = Self::generate_peer_id().await?;

        let encoded_info_hash = encode_binary(&self.info_hash);
        let encoded_peer_id = encode_binary(&peer_id);

        println!("{:?}", encoded_info_hash);
        println!("{:?}", encoded_peer_id);

        Ok(())
    }
}
