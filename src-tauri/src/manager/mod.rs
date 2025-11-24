use log::debug;
use tokio::sync::mpsc;

mod actions;
mod bandwidth;
mod chunk;
mod event;
mod monitor;
mod reports;

pub use actions::DownloadOptions;
pub use event::ManagerAction;

use crate::{emitter::Emitter, spawn};

#[derive(Debug, Clone)]
pub struct DownloadsManager {
    mpsc_sender: mpsc::UnboundedSender<ManagerAction>,
}

impl DownloadsManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<ManagerAction>();
        let manager = Self { mpsc_sender: tx };

        Self::initialize_mpsc_action(&manager, rx);
        manager
    }

    fn initialize_mpsc_action(&self, mut rx: mpsc::UnboundedReceiver<ManagerAction>) {
        spawn!("manager_mpsc", {
            while let Some(action) = rx.recv().await {
                if let Err(err) = Self::reducer(action).await {
                    debug!("{}", err.to_string());
                    Emitter::emit_error(err.to_string());
                }
            }
        });
    }
}
