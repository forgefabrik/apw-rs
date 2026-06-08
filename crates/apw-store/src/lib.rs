//! apw-store mod
pub mod memory;
pub use memory::MemoryEventStore;
pub fn name() -> &'static str {
    "apw-store"
}
