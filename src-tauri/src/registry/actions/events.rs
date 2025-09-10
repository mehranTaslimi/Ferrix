use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::registry::actions::{ActionKey, EventName};

use super::super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct EventData {
    pub key: ActionKey,
    pub action: RegistryAction,
}

pub trait EventsActions {
    async fn register_event(event: EventName, id: String) -> anyhow::Result<()>;
    async fn unregister_event(event: EventName, id: String) -> anyhow::Result<()>;
    async fn run_event_job(
        event: EventName,
        action_key: ActionKey,
        action: Box<RegistryAction>,
    ) -> anyhow::Result<()>;
    async fn event_job_completed(
        action_key: ActionKey,
        muted_action: Option<Box<RegistryAction>>,
        id: String,
    ) -> anyhow::Result<()>;
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
                drop(registered_event);
                registered_events.remove(&action);
            };
        };

        Ok(())
    }

    async fn run_event_job(
        event: EventName,
        action_key: ActionKey,
        action: Box<RegistryAction>,
    ) -> anyhow::Result<()> {
        let registered_events = Arc::clone(&Self::get_state().registered_events);
        let running_events = Arc::clone(&Self::get_state().running_events);

        let event_ids = registered_events
            .get(&event)
            .ok_or(anyhow::anyhow!("cannot find registered event: {:?}", event))?
            .clone();

        running_events.insert(action_key.clone(), event_ids.clone());

        spawn!(format!("event_job: {:?}", event), {
            for id in event_ids.iter() {
                Emitter::emit_event(
                    id,
                    EventData {
                        action: *action.clone(),
                        key: action_key.clone(),
                    },
                );

                loop {
                    if let Some(ev) = running_events.get(&action_key) {
                        if !ev.contains(id) {
                            break;
                        }
                    } else {
                        break;
                    }
                    sleep(Duration::from_millis(50)).await;
                }
            }

            let completed_events = Arc::clone(&Self::get_state().completed_events);
            {
                let mut completed_events = completed_events.write().await;
                completed_events.insert(action_key.clone());
            }

            let event_results = Arc::clone(&Registry::get_state().event_results);

            let to_dispatch: RegistryAction =
                if let Some(mut er) = event_results.get_mut(&action_key) {
                    if let Some(muted) = er.muted_action.take() {
                        *muted
                    } else {
                        *action
                    }
                } else {
                    *action
                };

            dispatch!(registry, ::to_dispatch);
        });

        Ok(())
    }

    async fn event_job_completed(
        action_key: ActionKey,
        muted_action: Option<Box<RegistryAction>>,
        id: String,
    ) -> anyhow::Result<()> {
        let running_events = Arc::clone(&Self::get_state().running_events);
        let event_results = Arc::clone(&Registry::get_state().event_results);

        if let Some(mut running_event) = running_events.get_mut(&action_key) {
            running_event.remove(&id);

            event_results.insert(action_key.clone(), EventResult { muted_action });

            if running_event.is_empty() {
                drop(running_event);
                running_events.remove(&action_key);
            }
        }

        Ok(())
    }
}

// Running -> prevent to run again and check can emit next event
// Completed -> prevent to emit and run same event again
