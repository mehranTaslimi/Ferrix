use magnet_url::Magnet;
use url::Url;

mod kind;
pub struct Link;
pub use kind::LinkKind;

impl Link {
    pub fn detect_url_lind(url: &str) -> anyhow::Result<LinkKind> {
        let u = Url::parse(&url)?;

        match u.scheme() {
            "http" | "https" => {
                if u.path().ends_with(".torrent") {
                    Ok(LinkKind::Torrent)
                } else {
                    Ok(LinkKind::Http)
                }
            }
            "ftp" => Ok(LinkKind::Ftp),
            "magnet" => match Magnet::new(url) {
                Ok(magnet) => Ok(LinkKind::Magnet),
                Err(e) => return Err(anyhow::anyhow!("invalid magnet link: {}: {}", url, e)),
            },
            _ => {
                return Err(anyhow::anyhow!("unsupported link scheme: {}", u.scheme()));
            }
        }
    }
}
