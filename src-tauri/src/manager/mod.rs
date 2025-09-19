use log::debug;
use std::sync::Arc;
use tokio::sync::mpsc;

mod actions;
mod bandwidth;
mod chunk;
mod event;
mod monitor;
mod reports;

pub use event::ManagerAction;

use crate::{emitter::Emitter, spawn};

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

        spawn!("manager_mpsc", {
            while let Some(action) = rx.recv().await {
                if let Err(err) = self_clone.reducer(action).await {
                    debug!("{}", err.to_string());
                    Emitter::emit_error(err.to_string());
                }
            }
        });
    }
}
