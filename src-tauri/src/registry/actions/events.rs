use std::time::Duration;

use tokio::time::sleep;

use crate::registry::actions::{ActionKind, EventName};

use super::super::*;

pub trait EventsActions {
    async fn register_event(event: EventName, id: String) -> anyhow::Result<()>;
    async fn unregister_event(event: EventName, id: String) -> anyhow::Result<()>;
    async fn run_event_job(
        event: EventName,
        action_kind: ActionKind,
        action: Box<RegistryAction>,
    ) -> anyhow::Result<()>;
    async fn event_job_completed(action_kind: ActionKind, id: String) -> anyhow::Result<()>;
}

impl EventsActions for Registry {
    async fn register_event(event: EventName, id: String) -> anyhow::Result<()> {
        let registered_events = Arc::clone(&Self::get_state().registered_events);
        registered_events.entry(event).or_default().insert(id);
        Ok(())
    }

    async fn unregister_event(action: EventName, id: String) -> anyhow::Result<()> {
        let registered_events = Arc::clone(&Self::get_state().registered_events);

        if let Some(mut registered_event) = registered_events.get_mut(&action) {
            registered_event.remove(&id);

            if registered_event.is_empty() {
                registered_events.remove(&action);
            };
        };

        Ok(())
    }

    async fn run_event_job(
        event: EventName,
        action_kind: ActionKind,
        action: Box<RegistryAction>,
    ) -> anyhow::Result<()> {
        let registered_events = Arc::clone(&Self::get_state().registered_events);
        let running_events = Arc::clone(&Self::get_state().running_events);

        let event_ids = registered_events
            .get(&event)
            .ok_or(anyhow::anyhow!("cannot find registered event: {:?}", event))?
            .clone();

        {
            running_events.insert(action_kind, event_ids.clone());
        }

        spawn!("run_event_job", {
            for id in event_ids.iter() {
                Emitter::emit_event(id, action_kind);

                loop {
                    if let Some(ev) = running_events.get(&action_kind) {
                        if !ev.contains(id) {
                            break;
                        }
                    } else {
                        break;
                    }
                    sleep(Duration::from_millis(10)).await;
                }
            }

            let completed_events = Arc::clone(&Self::get_state().completed_events);
            {
                let mut completed_events = completed_events.write().await;
                completed_events.insert(action_kind);
            }

            Self::dispatch(*action)
        });

        Ok(())
    }

    async fn event_job_completed(action_kind: ActionKind, id: String) -> anyhow::Result<()> {
        let running_events = Arc::clone(&Self::get_state().running_events);

        if let Some(mut running_event) = running_events.get_mut(&action_kind) {
            running_event.remove(&id);

            if running_event.is_empty() {
                drop(running_event);
                running_events.remove(&action_kind);
            }
        }

        Ok(())
    }
}
