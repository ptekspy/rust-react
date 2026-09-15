use std::collections::HashMap;

use super::Event;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ListenerId(u64);

struct EventListener {
    id: ListenerId,
    callback: Box<dyn Fn(&Event)>,
}

pub struct EventTarget {
    listeners: HashMap<String, Vec<EventListener>>,
    next_listener_id: u64,
}

impl EventTarget {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            next_listener_id: 0,
        }
    }

    pub fn add_event_listener<F>(
        &mut self,
        event_type: impl Into<String>,
        callback: F,
    ) -> ListenerId
    where
        F: Fn(&Event) + 'static,
    {
        let listener_id = ListenerId(self.next_listener_id);

        self.next_listener_id += 1;

        self.listeners
            .entry(event_type.into())
            .or_default()
            .push(EventListener {
                id: listener_id,
                callback: Box::new(callback),
            });

        listener_id
    }

    pub fn remove_event_listener(&mut self, listener_id: ListenerId) -> bool {
        for listeners in self.listeners.values_mut() {
            if let Some(index) = listeners
                .iter()
                .position(|listener| listener.id == listener_id)
            {
                listeners.remove(index);
                return true;
            }
        }

        false
    }

    pub fn dispatch_event(&mut self, event: &Event) -> bool {
        let Some(listeners) = self.listeners.get(&event.event_type()) else {
            return !event.default_prevented();
        };

        let listener_ids: Vec<ListenerId> =
            listeners.iter().map(|listener| listener.id).collect();

        for listener_id in listener_ids {
            if event.immediate_propagation_stopped() {
                break;
            }

            let Some(listener) = self
                .listeners
                .get(&event.event_type())
                .and_then(|listeners| {
                    listeners
                        .iter()
                        .find(|listener| listener.id == listener_id)
                })
            else {
                continue;
            };

            (listener.callback)(event);
        }

        !event.default_prevented()
    }
}

impl Default for EventTarget {
    fn default() -> Self {
        Self::new()
    }
}