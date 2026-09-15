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
        let Some(listeners) = self.listeners.get(event.event_type()) else {
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
                .get(event.event_type())
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

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::EventTarget;
    use crate::dom::Event;

    #[test]
    fn adds_listener() {
        let mut target = EventTarget::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        target.add_event_listener("click", move |_| {
            called_clone.set(true);
        });

        let event = Event::new("click");

        target.dispatch_event(&event);

        assert!(called.get());
    }

    #[test]
    fn returns_unique_listener_ids() {
        let mut target = EventTarget::new();

        let first = target.add_event_listener("click", |_| {});
        let second = target.add_event_listener("click", |_| {});

        assert_ne!(first, second);
    }

    #[test]
    fn removes_listener() {
        let mut target = EventTarget::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let listener = target.add_event_listener("click", move |_| {
            called_clone.set(true);
        });

        assert!(target.remove_event_listener(listener));

        let event = Event::new("click");

        target.dispatch_event(&event);

        assert!(!called.get());
    }

    #[test]
    fn removing_unknown_listener_returns_false() {
        let mut target = EventTarget::new();

        let listener = target.add_event_listener("click", |_| {});

        assert!(target.remove_event_listener(listener));
        assert!(!target.remove_event_listener(listener));
    }

    #[test]
    fn only_dispatches_matching_event_type() {
        let mut target = EventTarget::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        target.add_event_listener("click", move |_| {
            called_clone.set(true);
        });

        let event = Event::new("input");

        target.dispatch_event(&event);

        assert!(!called.get());
    }

    #[test]
    fn dispatches_listeners_in_registration_order() {
        let mut target = EventTarget::new();
        let calls = Rc::new(Cell::new(0));

        let first_calls = Rc::clone(&calls);
        target.add_event_listener("click", move |_| {
            assert_eq!(first_calls.get(), 0);
            first_calls.set(1);
        });

        let second_calls = Rc::clone(&calls);
        target.add_event_listener("click", move |_| {
            assert_eq!(second_calls.get(), 1);
            second_calls.set(2);
        });

        let event = Event::new("click");

        target.dispatch_event(&event);

        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn returns_false_when_default_is_prevented() {
        let mut target = EventTarget::new();

        target.add_event_listener("click", |event| {
            event.prevent_default();
        });

        let event = Event::new("click");

        assert!(!target.dispatch_event(&event));
        assert!(event.default_prevented());
    }

    #[test]
    fn returns_true_when_default_is_not_prevented() {
        let mut target = EventTarget::new();

        target.add_event_listener("click", |_| {});

        let event = Event::new("click");

        assert!(target.dispatch_event(&event));
    }

    #[test]
    fn stops_immediate_propagation() {
        let mut target = EventTarget::new();
        let calls = Rc::new(Cell::new(0));

        let first_calls = Rc::clone(&calls);
        target.add_event_listener("click", move |event| {
            first_calls.set(first_calls.get() + 1);
            event.stop_immediate_propagation();
        });

        let second_calls = Rc::clone(&calls);
        target.add_event_listener("click", move |_| {
            second_calls.set(second_calls.get() + 1);
        });

        let event = Event::new("click");

        target.dispatch_event(&event);

        assert_eq!(calls.get(), 1);
    }
}