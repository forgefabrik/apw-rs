# apw-kernel

Deterministischer Event-Sourcing Kernel — Event-Core, Algebra, Replay, Snapshot, Trust.

**Status M0:** Modul-Skelett mit Governance-Deklarationen.
**M1:** Event-Core, Algebra-Engine, Freezer, Trust-Report, Replay, Snapshot.

## Governance

- **Unsafe:** `#![forbid(unsafe_code)]`
- **Determinismus:** Nur BTreeMap/BTreeSet in Replay-State, keine Floats
- **Capability:** Sensitive Operations durch Capability-Enum geschützt
- **Panic:** Kernel returned Err, kein Crash
