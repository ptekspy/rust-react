use std::cell::Cell;

pub struct Event {
    event_type: String,
    default_prevented: Cell<bool>,
    propagation_stopped: Cell<bool>,
    immediate_propagation_stopped: Cell<bool>,
}

impl Event {
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            default_prevented: Cell::new(false),
            propagation_stopped: Cell::new(false),
            immediate_propagation_stopped: Cell::new(false),
        }
    }

    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    pub fn default_prevented(&self) -> bool {
        self.default_prevented.get()
    }

    pub fn prevent_default(&self) {
        self.default_prevented.set(true);
    }

    pub fn propagation_stopped(&self) -> bool {
        self.propagation_stopped.get()
    }

    pub fn stop_propagation(&self) {
        self.propagation_stopped.set(true);
    }

    pub fn immediate_propagation_stopped(&self) -> bool {
        self.immediate_propagation_stopped.get()
    }

    pub fn stop_immediate_propagation(&self) {
        self.immediate_propagation_stopped.set(true);
        self.propagation_stopped.set(true);
    }
}