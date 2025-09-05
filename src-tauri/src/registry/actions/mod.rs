mod download;
mod events;
mod report;
mod system;

pub use download::DownloadActions;
pub use events::EventsActions;
pub use report::ReportActions;
use serde::{Deserialize, Serialize};
pub use system::{SystemActions, Task, TaskStatus};

#[derive(Debug, Clone)]
pub enum RegistryAction {
    NewDownload(i64),
    CheckAvailablePermit,
    UpdateNetworkReport(i64, u64),
    UpdateDiskReport(i64, i64, u64),
    CleanDownloadedItemData(i64),
    PauseDownload(i64),
    ResumeDownload(i64),
    RecoverDownloads,
    RemoveDownload(i64, bool),
    CloseRequested,
    PrepareDownloadData(i64),
    UpdateChunkBufferReport(i64, i64, Vec<u8>),
    AddTask(u64, String),
    ChangeTaskStatus(u64, TaskStatus),
    RegisterEvent(EventName, String),
    UnRegisterEvent(EventName, String),
    EventJobCompleted(ActionKind, String),
    RunEventJob(EventName, ActionKind, Box<RegistryAction>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum EventName {
    NewDownload,
    CheckAvailablePermit,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "kebab-case")]
pub enum ActionKind {
    NewDownload(i64),
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

impl From<&RegistryAction> for ActionKind {
    fn from(action: &RegistryAction) -> Self {
        use RegistryAction::*;
        match action {
            NewDownload(id) => ActionKind::NewDownload(*id),
            CheckAvailablePermit => ActionKind::CheckAvailablePermit,
            UpdateNetworkReport(id, _) => ActionKind::UpdateNetworkReport(*id),
            UpdateDiskReport(id, _, _) => ActionKind::UpdateDiskReport(*id),
            CleanDownloadedItemData(id) => ActionKind::CleanDownloadedItemData(*id),
            PauseDownload(id) => ActionKind::PauseDownload(*id),
            ResumeDownload(id) => ActionKind::ResumeDownload(*id),
            RecoverDownloads => ActionKind::RecoverDownloads,
            RemoveDownload(id, _) => ActionKind::RemoveDownload(*id),
            CloseRequested => ActionKind::CloseRequested,
            PrepareDownloadData(id) => ActionKind::PrepareDownloadData(*id),
            UpdateChunkBufferReport(id, _, _) => ActionKind::UpdateChunkBufferReport(*id),
            AddTask(_, _) => ActionKind::AddTask,
            ChangeTaskStatus(_, _) => ActionKind::ChangeTaskStatus,
            RegisterEvent(_, _) => ActionKind::RegisterEvent,
            UnRegisterEvent(_, _) => ActionKind::UnRegisterEvent,
            EventJobCompleted(_, _) => ActionKind::EventJobCompleted,
            RunEventJob(_, _, _) => ActionKind::RunEventJob,
        }
    }
}

impl From<&RegistryAction> for EventName {
    fn from(value: &RegistryAction) -> Self {
        match value {
            RegistryAction::NewDownload(_) => EventName::NewDownload,
            RegistryAction::CheckAvailablePermit => EventName::CheckAvailablePermit,
            RegistryAction::UpdateNetworkReport(_, _) => EventName::UpdateNetworkReport,
            RegistryAction::UpdateDiskReport(_, _, _) => EventName::UpdateDiskReport,
            RegistryAction::CleanDownloadedItemData(_) => EventName::CleanDownloadedItemData,
            RegistryAction::PauseDownload(_) => EventName::PauseDownload,
            RegistryAction::ResumeDownload(_) => EventName::ResumeDownload,
            RegistryAction::RecoverDownloads => EventName::RecoverDownloads,
            RegistryAction::RemoveDownload(_, _) => EventName::RemoveDownload,
            RegistryAction::CloseRequested => EventName::CloseRequested,
            RegistryAction::PrepareDownloadData(_) => EventName::PrepareDownloadData,
            RegistryAction::UpdateChunkBufferReport(_, _, _) => EventName::UpdateChunkBufferReport,
            RegistryAction::AddTask(_, _) => EventName::AddTask,
            RegistryAction::ChangeTaskStatus(_, _) => EventName::ChangeTaskStatus,
            RegistryAction::RegisterEvent(_, _) => EventName::RegisterEvent,
            RegistryAction::UnRegisterEvent(_, _) => EventName::UnRegisterEvent,
            RegistryAction::EventJobCompleted(_, _) => EventName::EventJobCompleted,
            RegistryAction::RunEventJob(_, _, _) => EventName::RunEventJob,
        }
    }
}
