//! apw-store mod
pub mod memory;
pub use memory::MemoryEventStore;
pub fn name() -> &str { "apw-store" }
