use std::{
    future::Future,
    sync::{atomic::Ordering, Arc},
};

use log::info;
use tokio::task::JoinHandle;

impl super::Registry {
    pub(super) fn detect_max_concurrent_tasks() -> usize {
        num_cpus::get() * 10
    }

    pub fn spawn<F, R>(fut: F) -> JoinHandle<Option<R>>
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let permit = Arc::clone(&Self::get_state().current_tasks);
        let available_permits = Arc::clone(&Self::get_state().available_permits);
        let spawn_cancellation_token = Arc::clone(&Self::get_state().spawn_cancellation_token);

        tokio::spawn(async move {
            let acquired = permit
                .acquire()
                .await
                .expect("failed to acquire semaphore permit");

            available_permits.store(permit.available_permits(), Ordering::SeqCst);
            println!("{}", available_permits.load(Ordering::SeqCst));

            let result = tokio::select! {
                join_handle_response = fut => {
                    Some(join_handle_response)
                }

                _ = spawn_cancellation_token.cancelled() => {
                    None
                }
            };

            drop(acquired);
            available_permits.store(permit.available_permits(), Ordering::SeqCst);
            println!("{}", available_permits.load(Ordering::SeqCst));
            result
        })
    }
}
