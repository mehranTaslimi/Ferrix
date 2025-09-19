mod download;
mod events;
mod report;
mod system;

pub use download::{DownloadActions, DownloadOptions};
pub use events::EventsActions;
pub use report::ReportActions;
use serde::{Deserialize, Serialize};
pub use system::{SystemActions, Task, TaskStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload", rename_all = "kebab-case")]
pub enum RegistryAction {
    NewDownload {
        opt_id: String,
        url: String,
        options: DownloadOptions,
    },
    ProbeDownload {
        opt_id: String,
        url: String,
        options: DownloadOptions,
    },
    AddDownloadToQueue {
        download_id: i64,
    },
    CheckAvailablePermit,
    UpdateNetworkReport {
        download_id: i64,
        bytes_len: u64,
    },
    UpdateDiskReport {
        download_id: i64,
        chunk_index: i64,
        bytes_len: u64,
    },
    CleanDownloadedItemData {
        download_id: i64,
    },
    PauseDownload {
        download_id: i64,
    },
    ResumeDownload {
        download_id: i64,
    },
    RecoverDownloads,
    RemoveDownload {
        download_id: i64,
        remove_file: bool,
    },
    CloseRequested,
    PrepareDownloadData {
        download_id: i64,
    },
    UpdateChunkBufferReport {
        download_id: i64,
        chunk_index: i64,
        bytes: Vec<u8>,
    },
    AddTask {
        task_id: u64,
        task_name: String,
    },
    ChangeTaskStatus {
        task_id: u64,
        task_status: TaskStatus,
    },
    RegisterEvent {
        event_name: EventName,
        event_id: String,
    },
    UnRegisterEvent {
        event_name: EventName,
        event_id: String,
    },
    EventJobCompleted {
        action_key: ActionKey,
        event_id: String,
        muted_action: Option<Box<RegistryAction>>,
    },
    RunEventJob {
        event_name: EventName,
        action_key: ActionKey,
        internal_action: Box<RegistryAction>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum EventName {
    NewDownload,
    CheckAvailablePermit,
    AddDownloadToQueue,
    ProbeDownload,
    UpdateNetworkReport,
    UpdateDiskReport,
    CleanDownloadedItemData,
    PauseDownload,
    ResumeDownload,
    RecoverDownloads,
    RemoveDownload,
    CloseRequested,
    PrepareDownloadData,
    UpdateChunkBufferReport,
    AddTask,
    ChangeTaskStatus,
    RegisterEvent,
    UnRegisterEvent,
    EventJobCompleted,
    RunEventJob,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "event", content = "id", rename_all = "kebab-case")]
pub enum ActionKey {
    NewDownload(String),
    AddDownloadToQueue(i64),
    ProbeDownload(String),
    CheckAvailablePermit,
    UpdateNetworkReport(i64),
    UpdateDiskReport(i64),
    CleanDownloadedItemData(i64),
    PauseDownload(i64),
    ResumeDownload(i64),
    RecoverDownloads,
    RemoveDownload(i64),
    CloseRequested,
    PrepareDownloadData(i64),
    UpdateChunkBufferReport(i64),
    AddTask,
    ChangeTaskStatus,
    RegisterEvent,
    UnRegisterEvent,
    EventJobCompleted,
    RunEventJob,
}

impl From<&RegistryAction> for ActionKey {
    fn from(action: &RegistryAction) -> Self {
        use RegistryAction::*;
        match action {
            NewDownload { opt_id, .. } => ActionKey::NewDownload(opt_id.clone()),
            ProbeDownload { opt_id, .. } => ActionKey::ProbeDownload(opt_id.clone()),
            AddDownloadToQueue { download_id } => ActionKey::AddDownloadToQueue(*download_id),
            CheckAvailablePermit => ActionKey::CheckAvailablePermit,
            UpdateNetworkReport { download_id, .. } => ActionKey::UpdateNetworkReport(*download_id),
            UpdateDiskReport { download_id, .. } => ActionKey::UpdateDiskReport(*download_id),
            CleanDownloadedItemData { download_id } => {
                ActionKey::CleanDownloadedItemData(*download_id)
            }
            PauseDownload { download_id } => ActionKey::PauseDownload(*download_id),
            ResumeDownload { download_id } => ActionKey::ResumeDownload(*download_id),
            RecoverDownloads => ActionKey::RecoverDownloads,
            RemoveDownload { download_id, .. } => ActionKey::RemoveDownload(*download_id),
            CloseRequested => ActionKey::CloseRequested,
            PrepareDownloadData { download_id } => ActionKey::PrepareDownloadData(*download_id),
            UpdateChunkBufferReport { download_id, .. } => {
                ActionKey::UpdateChunkBufferReport(*download_id)
            }
            AddTask { .. } => ActionKey::AddTask,
            ChangeTaskStatus { .. } => ActionKey::ChangeTaskStatus,
            RegisterEvent { .. } => ActionKey::RegisterEvent,
            UnRegisterEvent { .. } => ActionKey::UnRegisterEvent,
            EventJobCompleted { .. } => ActionKey::EventJobCompleted,
            RunEventJob { .. } => ActionKey::RunEventJob,
        }
    }
}

impl From<&RegistryAction> for EventName {
    fn from(value: &RegistryAction) -> Self {
        match value {
            RegistryAction::NewDownload { .. } => EventName::NewDownload,
            RegistryAction::CheckAvailablePermit => EventName::CheckAvailablePermit,
            RegistryAction::UpdateNetworkReport { .. } => EventName::UpdateNetworkReport,
            RegistryAction::UpdateDiskReport { .. } => EventName::UpdateDiskReport,
            RegistryAction::CleanDownloadedItemData { .. } => EventName::CleanDownloadedItemData,
            RegistryAction::PauseDownload { .. } => EventName::PauseDownload,
            RegistryAction::ResumeDownload { .. } => EventName::ResumeDownload,
            RegistryAction::RecoverDownloads => EventName::RecoverDownloads,
            RegistryAction::RemoveDownload { .. } => EventName::RemoveDownload,
            RegistryAction::CloseRequested => EventName::CloseRequested,
            RegistryAction::PrepareDownloadData { .. } => EventName::PrepareDownloadData,
            RegistryAction::UpdateChunkBufferReport { .. } => EventName::UpdateChunkBufferReport,
            RegistryAction::AddTask { .. } => EventName::AddTask,
            RegistryAction::ChangeTaskStatus { .. } => EventName::ChangeTaskStatus,
            RegistryAction::RegisterEvent { .. } => EventName::RegisterEvent,
            RegistryAction::UnRegisterEvent { .. } => EventName::UnRegisterEvent,
            RegistryAction::EventJobCompleted { .. } => EventName::EventJobCompleted,
            RegistryAction::RunEventJob { .. } => EventName::RunEventJob,
            RegistryAction::AddDownloadToQueue { .. } => EventName::AddDownloadToQueue,
            RegistryAction::ProbeDownload { .. } => EventName::ProbeDownload,
        }
    }
}
