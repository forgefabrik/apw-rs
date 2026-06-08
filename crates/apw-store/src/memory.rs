use apw_protocol::{EventEnvelope, Tick};

#[derive(Debug, Default)]
pub struct MemoryEventStore {
    events: Vec<EventEnvelope>,
}

impl MemoryEventStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, envelope: EventEnvelope) {
        self.events.push(envelope);
    }

    #[must_use]
    pub fn head(&self) -> Option<EventEnvelope> {
        self.events.last().cloned()
    }

    #[must_use]
    pub fn replay(&self, from: Tick, to: Tick) -> Vec<EventEnvelope> {
        self.events
            .iter()
            .filter(|event| event.tick >= from && event.tick < to)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }
}
