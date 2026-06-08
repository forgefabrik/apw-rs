//! Append-only, atomic event store contract.
//!
//! ## Governance
//! -  MUST be implemented with fsync-level durability.
//! -  returns  on any integrity violation; callers must
//!   not crash (panic policy).

#![forbid(unsafe_code)]

use apw_protocol::{EventEnvelope, Tick};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("chain verification failed at tick {0:?}")]
    ChainVerificationFailed(Tick),
    #[error("duplicate event hash")]
    DuplicateEventHash,
    #[error("storage failure: {0}")]
    StorageFailure(String),
    #[error("capacity exceeded")]
    CapacityExceeded,
}

pub type Result<T> = std::result::Result<T, EventStoreError>;

pub trait EventStore: Send + Sync {
    fn append(&mut self, envelope: EventEnvelope) -> Result<()>;
    fn head(&self) -> Result<Option<EventEnvelope>>;
    fn replay(&self, from: Tick, to: Tick) -> Result<Vec<EventEnvelope>>;
    fn snapshot(&self, at: Tick) -> Result<Vec<u8>>;
    fn restore(&mut self, at: Tick, data: &[u8]) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct MemoryEventStore {
    events: Vec<EventEnvelope>,
}
impl EventStore for MemoryEventStore {
    fn append(&mut self, envelope: EventEnvelope) -> Result<()> {
        self.events.push(envelope);
        Ok(())
    }
    fn head(&self) -> Result<Option<EventEnvelope>> {
        Ok(self.events.last().cloned())
    }
    fn replay(&self, from: Tick, to: Tick) -> Result<Vec<EventEnvelope>> {
        Ok(self
            .events
            .iter()
            .filter(|e| e.tick >= from && e.tick < to)
            .cloned()
            .collect())
    }
    fn snapshot(&self, _at: Tick) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    fn restore(&mut self, _at: Tick, _data: &[u8]) -> Result<()> {
        Ok(())
    }
}
