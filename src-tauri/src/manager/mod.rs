use std::{sync::Arc, time::Duration};
use tokio::{spawn, sync::mpsc};

mod actions;
mod bandwidth;
mod chunk;
mod event;
mod monitor;

pub use actions::DownloadOptions;
pub use event::ManagerAction;

use crate::registry::Registry;

#[derive(Debug)]
pub struct DownloadsManager {
    mpsc_sender: Arc<mpsc::UnboundedSender<ManagerAction>>,
}

impl DownloadsManager {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = mpsc::unbounded_channel::<ManagerAction>();
        let manager = Arc::new(Self {
            mpsc_sender: Arc::new(tx),
        });

        Self::initialize_mpsc_action(manager.clone(), rx);
        manager
    }

    fn initialize_mpsc_action(self: Arc<Self>, mut rx: mpsc::UnboundedReceiver<ManagerAction>) {
        let self_clone = Arc::clone(&self);

        Registry::spawn(async move {
            while let Some(action) = rx.recv().await {
                self_clone.reducer(action).await;
            }
        });
    }
}
