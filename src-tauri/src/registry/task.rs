use std::{
    future::Future,
    sync::{atomic::Ordering, Arc},
};

use tokio::task::JoinHandle;

impl super::Registry {
    pub(super) fn detect_max_concurrent_tasks() -> usize {
        num_cpus::get() * 10
    }

    pub fn spawn<F, R>(task_name: &str, fut: F) -> JoinHandle<R>
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let permit = Arc::clone(&Self::get_state().current_tasks);
        let available_permits = Arc::clone(&Self::get_state().available_permits);
        let task_name = task_name.to_string();

        tokio::spawn(async move {
            let acquired = permit
                .acquire()
                .await
                .expect("failed to acquire semaphore permit");

            available_permits.store(permit.available_permits(), Ordering::SeqCst);
            println!(
                "[CREATED] available_permits: {}, task_name: {}",
                available_permits.load(Ordering::SeqCst),
                task_name
            );

            let join_handle_response = fut.await;

            drop(acquired);

            available_permits.store(permit.available_permits(), Ordering::SeqCst);
            println!(
                "[DROPPED] available_permits: {}, task_name: {}",
                available_permits.load(Ordering::SeqCst),
                task_name
            );

            join_handle_response
        })
    }
}
