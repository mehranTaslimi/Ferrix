use std::{
    sync::{atomic::Ordering, Arc},
    time::Instant,
};

use log::debug;
use serde::Serialize;

use crate::{
    dispatch, emitter::Emitter, file::File, queue_spawn, repository::download::DownloadRepository,
};

use super::super::Registry;

#[derive(Debug, Serialize)]
pub struct Task {
    pub name: String,
    pub status: TaskStatus,
    #[serde(skip)]
    pub start_time: Instant,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Completed,
    Running,
}

pub trait SystemActions {
    async fn check_available_permit() -> anyhow::Result<()>;
    async fn close_request() -> anyhow::Result<()>;
    async fn add_task(task_id: u64, task_name: impl Into<String>) -> anyhow::Result<()>;
    async fn change_task_status(task_id: u64, status: TaskStatus) -> anyhow::Result<()>;
}

impl SystemActions for Registry {
    async fn check_available_permit() -> anyhow::Result<()> {
        queue_spawn!("check_available_permit_action", {
            let state = Arc::clone(&Self::get_state());
            let pending_queue = Arc::clone(&state.pending_queue);

            let download_id = {
                let pending_queue = pending_queue.lock().await;
                pending_queue.front().cloned()
            };

            if let Some(download_id) = download_id {
                let download_result = DownloadRepository::find(download_id).await;

                match download_result {
                    Ok(download) => {
                        let chunk_count = download.chunk_count as usize;

                        let available_permits =
                            Self::get_state().available_permits.load(Ordering::SeqCst);
                        /*
                            The app reserves 10 permits for other operations.
                            Before starting a download, we check if available permits
                            are sufficient: the required `chunk_count` plus 5 extra permits as buffer.
                            If enough permits are available, the ID is popped from the queue
                            and the download is dispatched.
                        */

                        let remaining_bytes = download
                            .total_bytes
                            .saturating_sub(download.downloaded_bytes)
                            as u64;

                        let has_disk_space =
                            match File::check_disk_space(&download.file_path, remaining_bytes) {
                                Ok(is_available) => is_available,
                                Err(e) => {
                                    Emitter::emit_error(e);
                                    false
                                }
                            };

                        let permit_available = has_disk_space
                            && (available_permits >= 10 && available_permits - 5 >= chunk_count);

                        if permit_available {
                            pending_queue.lock().await.pop_front();
                            dispatch!(registry, PrepareDownloadData, (download_id));
                        }
                    }
                    Err(_) => {
                        /*
                            This section handles the case where a download is removed or cancelled from the queue.
                            The ID is removed from the queued list, and in the next loop iteration,
                            if the queue is empty, the loop will break and the listener will stop.
                        */
                        pending_queue.lock().await.pop_front();
                    }
                }
            }
        });

        Ok(())
    }

    async fn close_request() -> anyhow::Result<()> {
        Self::get_state().spawn_cancellation_token.cancel();
        let app_handle = Arc::clone(&Self::get_state().app_handle);
        app_handle.exit(0);
        Ok(())
    }

    async fn add_task(task_id: u64, task_name: impl Into<String>) -> anyhow::Result<()> {
        let tasks = Arc::clone(&Self::get_state().tasks);
        let now = Instant::now();
        tasks.insert(
            task_id,
            Task {
                name: task_name.into(),
                status: TaskStatus::Running,
                start_time: now,
            },
        );

        for task in tasks.iter() {
            debug!("{}. {}", task.key(), task.name)
        }
        debug!("--- --- --- ---");

        Ok(())
    }

    async fn change_task_status(task_id: u64, status: TaskStatus) -> anyhow::Result<()> {
        let tasks = Arc::clone(&Self::get_state().tasks);

        match status {
            TaskStatus::Completed => {
                tasks.remove(&task_id);
            }
            _ => {}
        };

        for task in tasks.iter() {
            debug!("{}. {}", task.key(), task.name)
        }
        debug!("--- --- --- ---");

        Ok(())
    }
}
