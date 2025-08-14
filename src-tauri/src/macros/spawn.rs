#[macro_export]
macro_rules! spawn {
    ($name:expr, $body:block) => {{
        let state = $crate::registry::Registry::get_state();

        let permit = ::std::sync::Arc::clone(&state.current_tasks);
        let task_id = ::std::sync::Arc::clone(&state.task_id);
        let available_permits = ::std::sync::Arc::clone(&state.available_permits);
        let spawn_cancellation_token = ::std::sync::Arc::clone(&state.spawn_cancellation_token);

        tokio::spawn(async move {
            let acquired = permit.acquire().await.unwrap();

            let task_id = task_id.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);

            $crate::dispatch!(registry, AddTask, (task_id, $name.into()));

            available_permits.swap(
                permit.available_permits(),
                ::std::sync::atomic::Ordering::SeqCst,
            );

            tokio::select! {
                _ = async move $body => {}
                _ = spawn_cancellation_token.cancelled() => {}
            }

            drop(acquired);

            available_permits.swap(
                permit.available_permits(),
                ::std::sync::atomic::Ordering::SeqCst,
            );
            $crate::dispatch!(
                registry,
                ChangeTaskStatus,
                (task_id, $crate::registry::TaskStatus::Completed)
            );
        });
    }};
}

#[macro_export]
macro_rules! loop_spawn {
    ($name:literal, $break_logic:expr, $duration:expr, $body:block) => {{
        let mut interval = tokio::time::interval($duration);

        $crate::spawn!($name, {
            loop {
                interval.tick().await;

                async move $body.await;

                if $break_logic().await {
                    break;
                }
            }
        });
    }};
}

#[macro_export]
macro_rules! monitors_spawn {
    ($(($name:literal, $duration:expr, $body:block)),* $(,)?) => {{
        let state = $crate::registry::Registry::get_state();

        let should_create = {
            let reports = ::std::sync::Arc::clone(&state.reports);
            move || {
                if !reports.is_empty()
                    && !state
                        .monitor_running
                        .load(::std::sync::atomic::Ordering::SeqCst)
                {
                    state
                        .monitor_running
                        .store(true, ::std::sync::atomic::Ordering::SeqCst);
                    return true;
                }
                false
            }
        };

        if should_create() {
            $(
                let should_break = async move || {
                    let reports = ::std::sync::Arc::clone(&state.reports);
                    if reports.is_empty() {
                        state
                            .monitor_running
                            .store(false, ::std::sync::atomic::Ordering::SeqCst);
                        return true;
                    }
                    false
                };

                $crate::loop_spawn!($name, should_break, $duration, $body);
            )*
        }
    }};
}

#[macro_export]
macro_rules! queue_spawn {
    ($name:literal, $body:block) => {{
        let state = $crate::registry::Registry::get_state();

        let should_create = async move || {
            let pending_queue = ::std::sync::Arc::clone(&state.pending_queue);
            if !pending_queue.lock().await.is_empty()
                && !state
                    .queue_listener_running
                    .load(::std::sync::atomic::Ordering::SeqCst)
            {
                state
                    .queue_listener_running
                    .store(true, ::std::sync::atomic::Ordering::SeqCst);
                return true;
            }
            false
        };

        if should_create().await {
            let duration = ::std::time::Duration::from_secs(1);
            let should_break = async move || {
                let pending_queue = ::std::sync::Arc::clone(&state.pending_queue);
                if pending_queue.lock().await.is_empty() {
                    state
                        .queue_listener_running
                        .store(false, ::std::sync::atomic::Ordering::SeqCst);
                    return true;
                }
                false
            };
            $crate::loop_spawn!($name, should_break, duration, $body);
        };
    }};
}
