//! apw-kernel — deterministischer Event-Sourcing Kernel für apw-rs.
//!
//! ## Governance
//!
//! - **Time-Policy:** `std::time::SystemTime` ist **verboten**.
//!   Nur `Tick` aus `apw-protocol` als autoritative Zeit.
//! - **Determinismus-Policy:** Nur `BTreeMap`/`BTreeSet` in Replay-State.
//!   Keine `HashMap`/`HashSet`, keine `f32`/`f64` in kanonischen Pfaden.
//! - **Capability-Policy:** Jede sensitive Operation ist durch `Capability`-Gate geschützt.
//!   `AuthorityMap` wird nur hier mutiert.
//! - **Panic-Policy:** Kernel gibt `Err` zurück; nur `apw-server` darf crashen.
//! - **Unsafe:** `#![forbid(unsafe_code)]`

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::struct_field_names,
    clippy::too_many_arguments,
)]

pub mod algebra;
pub mod clock;
pub mod event_store;
pub mod replay;
pub mod snapshot;
pub mod trust;

pub fn name() -> &'static str {
    "apw-kernel"
}