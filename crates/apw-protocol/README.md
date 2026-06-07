#apw-protocol

Wire-Types für das apw-rs-System. Pure `serde`-Types -- kein I/O, keine Runtime.

**Status M0:** Skelett mit allen Types aus dem Design-Spec.
**M1+:** Event-Core, kanonische Serialisierung, Canonical-Trait.

## Governance

- **Determinismus:** Nur `BTreeMap`/`BTreeSet` in replay-authoritativem State
- **Serialisierung:** Keine Floats in kanonischen Pfaden -- pur Integer - **Zeit:** `Tick(u64)` als semantischer Newtype -- kein `SystemTime`
