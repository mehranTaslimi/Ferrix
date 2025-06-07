use tokio::sync::broadcast;

pub struct AppState {
    pub tx: broadcast::Sender<AppEvent>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    CancelDownload(String),
    PauseDownload(String),
    ResumeDownload(String),
}
