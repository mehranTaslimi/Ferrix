use crate::dispatch;

use super::actions::{
    ActionKind, DownloadActions, EventName, EventsActions, RegistryAction, ReportActions,
    SystemActions,
};

use anyhow::Context;
use std::sync::Arc;

impl super::Registry {
    pub fn dispatch(action: RegistryAction) -> anyhow::Result<()> {
        let mpsc_sender = Arc::clone(&Self::get_state().mpsc_sender);
        mpsc_sender
            .send(action)
            .context("failed to dispatch registry action: receiver might be closed")
    }

    pub(super) async fn reducer(action: RegistryAction) -> anyhow::Result<()> {
        use RegistryAction::*;

        let event_name = EventName::from(&action);

        let registered_events = Arc::clone(&Self::get_state().registered_events);
        let is_event_registered = registered_events.contains_key(&event_name);

        if is_event_registered {
            let completed_events = Arc::clone(&Self::get_state().completed_events);
            let running_events = Arc::clone(&Self::get_state().running_events);

            let action_kind = ActionKind::from(&action);

            let running = running_events.contains_key(&action_kind);
            let completed = completed_events.read().await.contains(&action_kind);

            match (running, completed) {
                (false, true) => {
                    completed_events.write().await.remove(&action_kind);
                }
                (false, false) => {
                    dispatch!(
                        registry,
                        RunEventJob,
                        (event_name, action_kind, Box::new(action.clone()))
                    )?;
                    return Ok(());
                }
                (true, false) => {
                    return Err(anyhow::anyhow!("the event is already running"));
                }
                _ => {}
            }
        }

        match action {
            // Download
            PauseDownload(download_id) => Self::pause_download(download_id).await,
            ResumeDownload(download_id) => Self::resume_download(download_id).await,
            RemoveDownload(download_id, remove_file) => {
                Self::remove_download(download_id, remove_file).await
            }
            NewDownload(download_id) => Self::new_download(download_id).await,
            RecoverDownloads => Self::recover_downloads().await,
            PrepareDownloadData(download_id) => Self::prepare_download_data(download_id).await,
            CleanDownloadedItemData(download_id) => Self::clean_download_data(download_id).await,

            // Report
            UpdateNetworkReport(download_id, bytes_len) => {
                Self::update_network_report(download_id, bytes_len).await
            }
            UpdateDiskReport(download_id, chunk_index, bytes_len) => {
                Self::update_disk_report(download_id, chunk_index, bytes_len).await
            }
            UpdateChunkBufferReport(download_id, chunk_index, bytes) => {
                Self::update_chunk_buffer_report(download_id, chunk_index, bytes).await
            }

            // System
            CheckAvailablePermit => Self::check_available_permit().await,
            CloseRequested => Self::close_request().await,
            AddTask(task_id, task_name) => Self::add_task(task_id, task_name).await,
            ChangeTaskStatus(task_id, status) => Self::change_task_status(task_id, status).await,

            // Event
            RegisterEvent(event_name, id) => Self::register_event(event_name, id).await,
            UnRegisterEvent(event_name, id) => Self::unregister_event(event_name, id).await,
            RunEventJob(event_name, action_kind, action) => {
                Self::run_event_job(event_name, action_kind, action).await
            }
            EventJobCompleted(action_kind, id) => Self::event_job_completed(action_kind, id).await,
        }
    }
}
