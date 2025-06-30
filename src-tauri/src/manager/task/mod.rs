use std::{future::Future, sync::Arc};

use tokio::{
    sync::{Mutex, Semaphore},
    task::JoinHandle,
};

#[derive(Clone, Debug)]
pub struct TaskManager {
    current_tasks: Arc<Semaphore>,
    pub available_permits: Arc<Mutex<usize>>,
}

impl TaskManager {
    pub fn new() -> Arc<Self> {
        let max_concurrent_tasks = Self::detect_max_concurrent_tasks();

        Arc::new(Self {
            current_tasks: Arc::new(Semaphore::new(max_concurrent_tasks)),
            available_permits: Arc::new(Mutex::new(max_concurrent_tasks)),
        })
    }

    fn detect_max_concurrent_tasks() -> usize {
        num_cpus::get() * 10
    }

    pub fn spawn<F, R>(&self, fut: F) -> JoinHandle<R>
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let permit = Arc::clone(&self.current_tasks);
        let available_permits: Arc<Mutex<usize>> = Arc::clone(&self.available_permits);
        tokio::spawn(async move {
            let acquired = permit
                .acquire()
                .await
                .expect("failed to acquire semaphore permit");

            *available_permits.lock().await = permit.available_permits();
            println!("{}", available_permits.lock().await);

            let join_handle_response = fut.await;

            drop(acquired);

            *available_permits.lock().await = permit.available_permits();
            println!("{}", available_permits.lock().await);

            join_handle_response
        })
    }
}
