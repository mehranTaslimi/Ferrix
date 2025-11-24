#[macro_export]
macro_rules! spawn {
    ($name:expr, $body:block) => {{
        let state = $crate::registry::Registry::get_state();

        tokio::spawn(async move {
            let acquired = state.current_tasks.acquire().await.unwrap();

            let task_id = state
                .task_id
                .fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);

            $crate::dispatch!(registry, AddTask, (task_id, $name.into()));

            state.available_permits.swap(
                state.current_tasks.available_permits(),
                ::std::sync::atomic::Ordering::SeqCst,
            );

            tokio::select! {
                _ = async move $body => {}
                _ = state.spawn_cancellation_token.cancelled() => {}
            }

            drop(acquired);

            state.available_permits.swap(
                state.current_tasks.available_permits(),
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
            || {
                if !state.reports.is_empty()
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
                let should_break = {
                    let state = ::std::sync::Arc::clone(&state);
                    async move || {
                        if state.reports.is_empty() {
                            state
                                .monitor_running
                                .store(false, ::std::sync::atomic::Ordering::SeqCst);
                            return true;
                        }
                        false
                    }
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

        let should_create = async || {
            if !state.pending_queue.lock().await.is_empty()
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
                if state.pending_queue.lock().await.is_empty() {
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
