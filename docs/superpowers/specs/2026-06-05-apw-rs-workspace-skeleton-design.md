# apw-rs Workspace Skeleton — Design

**Date:** 2026-06-05
**Status:** Approved (brainstorming)
**Target repo:** `github.com/forgefabrik/apw-rs` (new repo, not yet created)

## Goal

A pure-Rust Cargo workspace skeleton for the ForgeFabrik Rust rewrite, covering 4 products (agent-os, opencode-manager-style manager, agent-bigbrother, pixtuoid-style office) in 8 crates. The skeleton compiles, passes tests, and enforces an explicit server/client architecture. It does **not** port any existing logic — that lands in M1+.

## Non-Goals (explicit)

- No actual porting of `kernel/`, `engine/`, or any Node code.
- No real HTTP server, no DB, no LLM calls.
- No Aseprite integration; no animated sprite rendering to a TUI.
- No replacement of the Node webui.
- No publish to crates.io.

## Minimum Supported Rust Version (MSRV)

- **`rust-toolchain.toml` pins 1.82.x exactly.** No `1.82+` ambiguity, no "latest stable" drift, no local-vs-CI toolchain mismatches.
- Bumping the MSRV is a **breaking change** and requires an ADR (see §ADR policy below). Contributors on a newer toolchain build fine; contributors on an older one see a clear error from `rustup`.
- The MSRV is the **only** toolchain policy. We do not pin patch versions of `rustfmt` or `clippy` independently — they ship with the pinned `cargo`.

## ADR Policy

To keep the architecture governance aligned with the "mechanical, not social" spirit, the following changes **require an Architecture Decision Record** committed under `docs/adr/`:

1. **Adding a new crate** to the workspace. The ADR justifies the new boundary and names which other crates it depends on.
2. **Changing a boundary** — e.g. allowing a client crate to depend on `apw-server`, or lifting the `apw-protocol` runtime-purity rule. The ADR states the new rule, the reason, and the rollback plan.
3. **Protocol-breaking changes** to `apw-protocol` — removing a field, renaming an `Event` variant, changing the `serde` tag scheme. The ADR includes a migration plan for the Node webui and any other consumers.
4. **Promoting a new dependency to a "required" one** (e.g. moving from optional `tokio` to required `tokio`). The ADR explains the cost of the dep and why it's worth it.

ADRs are short (1–2 pages), use a standard template (`docs/adr/0000-template.md`), and are merged via PR. A change is **not** considered "done" until its ADR is merged.

## Workspace Philosophy (enforceable rules)

1. **Crates exist only at architectural boundaries.** A crate is introduced only when it creates an enforceable dependency rule (server vs client, schema vs logic, etc.) — not for conceptual grouping or to reduce diff size.
2. **No catch-all shared crate.** `apw-common`, `apw-utils`, `apw-shared`, `apw-core` (used as a kitchen sink), or any equivalent catch-all crate is **forbidden without an ADR**. Catch-all crates erase dependency direction over time and are the single most common cause of unmaintainable Rust workspaces. If shared helpers are needed, they go in `apw-protocol` (if they're types) or in a small purpose-named crate (e.g. `apw-hash` for hash utilities) with its own enforced boundary.
3. **Server and client are physically separated.** A client crate MUST NOT depend on a server crate. A server crate MUST NOT depend on a client crate. Enforced by a compile-time test in every crate (see §Boundary tests).
4. **`apw-protocol` is the only shared crate.** It contains pure `serde` types, no `std::fs`, no `reqwest`, no `tokio`. Any crate that needs wire types depends on this and only this for shared types.
5. **Boundary enforcement is mechanical, not social.** A violation must fail `cargo test`, not a code review comment.

> **Known limitations of the M0 grep-based tests (deliberate, not naive):**
> - `grep Cargo.toml` cannot detect **transitive deps** (a forbidden crate sneaks in via a permitted one)
> - It cannot detect **workspace dependency aliases** that point at a forbidden crate
> - It cannot detect **feature-induced deps** (`dep = { ..., optional = true }` resolved by a feature flag)
> - It cannot detect **path indirection** (a permitted crate re-exports a forbidden crate's types)
>
> The grep approach is the smallest thing that enforces the M0 boundaries; the limitations are tracked so a future milestone can replace it with a proper graph check.
>
> **Planned upgrade (tracked in `docs/ROADMAP.md`, future Mx):** run `cargo metadata --format-version 1` from the workspace root, parse the resolved dependency graph, and assert that no crate has a forbidden edge in its transitive closure. This catches all four failure modes above and fails `cargo test` on violation, with no manual list maintenance. The M0 grep tests ship first; the upgrade is a self-contained `tools/check-boundaries/` binary that replaces the per-crate `tests/boundary.rs` files.

## Determinism Policy

The system combines hash-chained events, trust replay, the freezer, **and** an LLM-driven admin layer (M3). Without an explicit split, the LLM will eventually produce output that "replays" differently — and trust will be silently invalidated. The policy:

1. **Kernel-replay is deterministic.** `apw-kernel` (event-core, algebra, freezer, trust-report, replay) is the source of truth and must produce identical state on replay given the same event chain. No time-of-day, no randomness, no I/O in the replay path.
2. **LLM output is never replay-authoritative.** The LLM (`apw-engine/llm` `TowerAdmin` trait, M3) can produce *suggestions* — bid recommendations, room layouts, sprite proposals — but every authoritative state mutation goes through the deterministic kernel via a contract, a lease, or an explicit rule. The LLM never writes to the event chain directly.
3. **LLM produces intents, not events.** The LLM emits `Intent` types (serializable, stored, replayable). A separate deterministic reducer in the kernel turns intents into events. The reducer's rules are part of the replay contract.
4. **Authority rules are static.** Which subsystems an LLM is allowed to influence is defined in a static config file (loaded at boot, validated against the schema). The config is part of the trust chain — a replay must produce the same authority map.
5. **Randomness, if needed, is seeded.** The only acceptable source of randomness in the kernel is a seeded PRNG whose seed is part of the event payload. Replay reproduces the same sequence.

This split is the line between "the system is auditable" and "the system is a chatbot with logging." M0 ships no LLM, but the policy is locked now so M3 has a place to land.

## Time Policy

The Determinism Policy requires that the kernel be replay-authoritative. That is impossible if the kernel reaches for the host clock. Time is therefore promoted from "a primitive the standard library gives you" to a first-class architectural concept with three distinct domains and one trait.

### Three time domains

| Domain | Lives in | Authoritative? | Replayable? | Used for |
|---|---|---|---|---|
| **Replay time** (`Tick`) | The event log | **Yes** | **Yes** | Event ordering, lease windows, scheduler ticks, trust replay, LLM intent timestamps, every kernel-internal timestamp |
| **Wall clock** (`std::time::SystemTime`) | Process boundary only | No | No | HTTP access logs, telemetry, UI timestamps, "last seen by human" displays |
| **Simulation time** (`Tick` driven by a `ClockSource`) | The simulation layer | No (configurable) | Yes (deterministic) | Fast-forward, pause, freezer snapshots, accelerated replay during tests |

Replay time and simulation time are the same type (`Tick`) — a simulation clock is just a `ClockSource` implementation that emits ticks on demand. Wall clock is the odd one out: it never crosses into the kernel.

### The rule

1. **`std::time::SystemTime` is forbidden in `apw-kernel`.** No `SystemTime::now()`, no `Instant::now()`, no `chrono::Utc::now()`, no `tokio::time::Instant`. The only acceptable "now" inside the kernel is whatever the injected `ClockSource` returns. Enforced by `cargo test` (a grep-style check on `apw-kernel/src/` for the forbidden tokens; M1+ swaps this for a proper lint).
2. **Authoritative time is a `Tick`.** Every event carries its tick in its metadata. Every lease, scheduler decision, trust check, and LLM intent operates on ticks. The signature is `tick: u64`, not `timestamp: SystemTime`.
3. **Wall clock exists only at the process boundary.** `apw-server` is the only place that may call `SystemTime::now()`. It does so exclusively to format the wall-clock timestamp for an HTTP response header, a log line, or a UI display field. The result never enters a kernel data structure.
4. **Replay uses recorded ticks, never current host time.** When the kernel replays an event, it reads the tick from the event itself, not from the clock. A `ReplayClock` returns the recorded tick of the event currently being processed; a `LiveClock` returns the next tick from the live feed.
5. **Schedulers, leases, and the freezer operate on ticks.** "Lease expires in 30 seconds" is `lease_expires_at_tick: u64`. "Scheduler tick every 5s" is `interval_ticks: u64`. The mapping from "5 seconds" to `interval_ticks` happens once, at boot, from a config value, and is part of the trust chain (a replay must use the same mapping).
6. **No hidden time in the kernel.** No `sleep(Duration)`, no `std::thread::sleep`, no `tokio::time::sleep`, no implicit timeout. A function that "waits" returns a `Future` and lets the caller decide the executor (see Async Policy). A function that "times out" takes a `Tick` deadline, not a `Duration`.
7. **Timeouts live at the server boundary, not in the kernel.** HTTP request timeouts, SSE keepalive, LLM API call timeouts — all in `apw-server` or in a client crate. The kernel does not know "now + 30s" — it knows "tick 100,000" and lets the caller decide what that means in wall-clock terms.

### The `ClockSource` trait (skeleton shape)

```rust
// in apw-protocol
pub struct Tick(pub u64);

/// Read-only clock: "what tick is it right now?"
/// Implemented by every clock. Pure query, no side effects.
pub trait ClockSource: Send + Sync {
    fn tick(&self) -> Tick;
}

/// Mutable clock: drives a simulation or test scenario forward.
/// **Not** implemented by clocks that walk the event log or by clocks
/// driven by external events — those must be read-only to keep the
/// trust chain honest (a `ReplayClock` that can `advance` is a bug).
pub trait SimulationClock: ClockSource {
    fn advance(&mut self, n: u64);
}
```

The split is intentional. A `ClockSource` you can `advance` invites trait-semantic bugs: a `ReplayClock` with `advance` would let a buggy caller mutate the cursor and invalidate the replay; a `LiveClock` with `advance` would let a caller rewind the live feed. The two traits force the type system to encode "this clock is a record-keeper, that clock is a scenario-driver."

M1+ implementations, classified by trait:

| Clock | Implements `ClockSource` | Implements `SimulationClock` |
|---|---|---|
| `LiveClock` — driven by an external tick emitter | ✓ | ✗ |
| `ReplayClock` — reads ticks from an event log during replay | ✓ | ✗ |
| `FrozenClock` — emits a fixed tick (for snapshot tests) | ✓ | ✗ |
| `SimulationClock::Accelerated` — emits N ticks per real-time second (for fast-forward, load tests) | ✓ | ✓ |
| `SimulationClock::Manual` — caller drives ticks explicitly (for property-based tests) | ✓ | ✓ |

The skeleton (M0) does not implement any of these. The trait signatures and the policy are locked now; the implementations land when the kernel actually needs a clock in M1.

### Why this matters for LLMs

LLM-driven admin (M3) is the strongest argument for this policy. If the LLM is given "current office activity at 14:32" (wall clock) and asked to suggest a promotion, the next replay at 15:00 produces a different prompt, a different state, a different suggestion — and trust is invalidated. With intent-based admin and a kernel that only knows `Tick`, the LLM is given a tick-based snapshot and produces a tick-stamped intent. Replay reads the same snapshot, the intent has the same effect, trust stays valid.

This is not theoretical. It is the difference between "the system is auditable" and "the audit log is a suggestion."

## Deterministic Serialization Rule

The event chain is hashed. The hash is part of the trust contract. A different hash on a different platform, on a different Rust version, or after a `serde_json` upgrade is a silent trust violation — the chain "verifies" locally but is invalid for every other consumer. The rule locks the serialization contract now, while the chain is still empty.

1. **The kernel does not hash over `serde_json` directly.** `serde_json`'s default output is **not canonical**: map key order is unspecified, whitespace varies, number formatting varies, and the `serde_json` crate reserves the right to change all of the above between versions. The kernel uses a deterministic serializer — M1+ picks one (likely `serde_json` with a canonical-feature flag, or a dedicated `apw-canonical` crate using `BTreeMap` + sorted keys) — and the choice is locked by an ADR. Furthermore, `apw-kernel` and `apw-protocol` do not reference `serde_json::Value`, `serde_json::Map`, or `serde_value::Value` *at all*: any untyped JSON tree at the kernel/protocol boundary is a path to non-canonical bytes. Dynamic-JSON use cases (subsystem payloads, plugin bridges) flow through a typed `BTreeMap<String, KvValue>` or a future boundary-conversion crate (see Boundary Tests §1.5); they never instantiate `Value` inside the kernel or the protocol types.
2. **Canonical ordering is mandatory.** All ordered collections in the canonical form are `BTreeMap`/`BTreeSet` (sorted) or `Vec` (positional). The first rule of canonical serialization is "there is only one valid byte sequence for a given value." Anything that introduces non-determinism — `HashMap`, `HashSet`, floating-point output that depends on platform rounding, timezones — is forbidden in the canonical path.
3. **UTF-8 normalization is required.** All string fields are normalized to NFC before serialization. A user types "café" (composed) and the system stores "café" (decomposed) — the hashes must be the same. The M0 skeleton does not implement this; M1+ picks the normalization function (likely `unicode-normalization` crate) and the choice is locked by an ADR.
4. **Floats are forbidden in canonical paths.** Floating-point numbers are non-deterministic across platforms (x87 vs SSE, fast-math flags, fused-multiply-add). Every numeric field in a canonical-serialized type is an integer (`u64`, `i64`), a `Decimal` (fixed-point), or a string-encoded rational. The M0 protocol sketch reflects this: `AgentState::reputation_milli: u32` (0..=1000) instead of `f32`, and the `SubsystemPayload` enum's `Economy` variant carries `multiplier_milli: u32` instead of a float multiplier. The `Cargo.toml` lint in `apw-kernel` (per the Deterministic Iteration Policy §5) rejects `f32`/`f64` in the kernel crate.
5. **Hash function choice is locked.** The event hash uses one specific hash function (M1+ decides: BLAKE3 is the tentative default — fast, deterministic, no platform variation). Changing the hash function is a breaking change to the trust chain and requires an ADR. M0 picks the placeholder; M1 confirms or replaces it.
6. **The canonical form is snapshot-tested.** A `cargo test` in the relevant crate (likely `apw-kernel` in M1+) takes every public type, serializes it canonically, hashes it, and asserts the hash matches a checked-in fixture. Any change to the byte sequence fails the test, forcing the contributor to either fix a regression or explicitly bump the fixture (which is itself a visible diff in code review).
7. **No `to_string()` in the canonical path.** `Display` impls are not stable. `Debug` impls are explicitly not stable. The canonical serializer is the **only** path that produces hashable bytes. Code that wants to hash anything goes through it; code that does not, does not.

This rule is the difference between "the chain verifies on my laptop" and "the chain verifies on every laptop, in CI, in production, in 5 years, on a future Rust version." The implementation cost is real but bounded (one crate, one test fixture, one ADR). The cost of *not* having it shows up years later as "the chain is fine here, but the partner's chain is invalid" — and the only fix is a rehash of history.

## Deterministic Iteration Policy

The Deterministic Serialization Rule says "no `HashMap` in the canonical path." That is the serialization-side enforcement. This policy is the **type-side** enforcement: it stops the wrong collection from ever being instantiated in replay-authoritative state in the first place.

1. **`HashMap` and `HashSet` are forbidden in replay-authoritative state.** Any struct that lives in `apw-kernel`, in the event chain, in a snapshot, or in a wire type that is hashed or replayed must use only:
   - `BTreeMap<K, V>` — sorted by key, iteration order is the sorted order
   - `BTreeSet<T>` — sorted by value
   - `Vec<T>` — positional, iteration order is the push order
   - `IndexMap<K, V>` — insertion-ordered, allowed only if the canonical serializer uses the **stable serializer** (sorted-keys mode) and the choice is locked by an ADR
   The "replay-authoritative" predicate is: *will the result of iterating this collection ever feed into a hash, a state transition, or a replayed comparison?* If yes, the collection must be ordered.
2. **Iteration order is part of the contract.** A function that returns `BTreeMap<K, V>` is *promising* its caller that two calls with equal inputs produce equal iteration orders. A test in M1+ asserts this for every public collection-returning function in the kernel.
3. **No `HashMap` "because it's faster."** Performance is not a justification for non-determinism in replay-authoritative code. If a hot path needs a `HashMap`, the surrounding code is *not* replay-authoritative (e.g. a transient cache in `apw-server` for HTTP routing) — and that fact is documented at the type.
4. **`IndexMap` is the only escape hatch for insertion order.** It is allowed only when the canonical serializer sorts its output before hashing. Using `IndexMap` to *avoid* the sort is forbidden — that just re-introduces a different non-determinism (insertion order depends on the call site, not the data).
5. **The lint is mechanical.** A `cargo test` in every replay-authoritative crate (`apw-kernel`, `apw-protocol`, and any future crate that holds chain/snapshot/wire state) greps its `src/` for the tokens `HashMap` and `HashSet` and asserts they appear nowhere except in explicitly-allowed locations (a small `[allow]` list, reviewed in ADRs as the crate grows). M1+ replaces this with a proper Clippy lint (`disallowed_types`) at the workspace level.
6. **External `HashMap` types are wrapped, not stored.** If the kernel has to consume a `HashMap` from an external API (a third-party crate, a config parser), it converts to `BTreeMap` at the boundary. The boundary conversion is one explicit function; nothing in the kernel ever holds a `HashMap`.

The most common late-stage failure this prevents:

```rust
// year 2, someone adds this to KernelState:
pub struct KernelState {
    pub agents: HashMap<AgentId, AgentState>,  // ← looks innocent
    // ...
}

// year 2, the trust report iterates `agents` to compute a digest:
fn trust_digest(s: &KernelState) -> Hash {
    let mut h = Hasher::new();
    for (id, agent) in &s.agents {  // ← order is randomized per HashMap
        h.update(id);
        h.update(agent.canonical_bytes());
    }
    h.finalize()
}

// year 2, replay produces a different digest because HashMap iteration
// order is a function of the random hasher seed, which is per-process.
```

This rule is the difference between a system whose replay produces the same digest forever and a system that "mostly works" until it doesn't. The cost is zero at runtime (`BTreeMap` is fine for the data sizes the kernel handles) and the rule is one grep-style test.

## Configuration Determinism Policy

The Determinism Policy, the Time Policy, and the Iteration Policy cover *code* and *time*. The fourth determinism leak is *configuration*: a replay that boots with a different feature flag, a different default timeout, or a different env var silently produces a different state. The policy closes the gap now, before the first feature flag lands.

1. **Replay-authoritative config is canonicalized and hashed.** Any config the kernel reads at boot — authority map, scheduler intervals, lease TTLs, LLM intent thresholds — is parsed into a typed, ordered structure, canonical-serialized (per the Deterministic Serialization Rule), and the hash is recorded in the **chain genesis event**. A replay that boots with a different config produces a different genesis hash and is rejected as "config mismatch" — not as a silent state divergence.
2. **The boot config is part of the trust chain.** A `ConfigSnapshot { schema_version, content_hash, parsed: ConfigRoot }` is itself a replay-authoritative data structure. The kernel may read from it freely; the kernel may not bypass it. There is no `std::env::var("LEASE_TTL_MS")` anywhere in the kernel crate (forbidden-import test in Layer 2 of the boundary tests).
3. **Environment variables are boundary-only.** `apw-server` is the only process that may read `std::env`. It does so to compute the boot `ConfigRoot` (e.g. resolve a config file path, discover an LLM endpoint URL), but the resolved `ConfigRoot` is what the kernel sees. The kernel never reads `std::env` directly. (This is the same shape as the Time Policy: the host primitive is allowed at the process boundary, forbidden in the kernel.)
4. **No ambient config reads in the kernel.** No `lazy_static` config singletons, no global `OnceCell<Config>`, no `thread_local!` config lookups, no "convenience" `cfg!(...)` feature flags inside kernel logic. All config flows in as a constructor argument or a method parameter. A test can boot the kernel with a stub `ConfigRoot`; the kernel does not need a real environment to run.
5. **Feature flags alter config, not code paths.** If a capability can be turned off, the turn-off is a field in `ConfigRoot`, not a `#[cfg(feature = "...")]` attribute on the kernel. The set of feature flags is part of the build, not the config; the kernel code path is the same regardless of which features are enabled at runtime. (Cargo features remain the way to omit code from the *binary*, but the kernel itself does not branch on them at runtime.)
6. **Config errors fail the boot, not the operation.** A malformed config does not produce a `Result<_, ConfigError>` from a kernel method; it produces a `ConfigRoot` that is `valid: false` and the boot fails. The kernel has no path that takes a config it does not understand and "does its best."
7. **Config changes are versioned and migratable.** A `ConfigRoot` carries `schema_version`. Changing the config schema is a breaking change and bumps the major version of `apw-config` (a future crate, M2+) or `apw-protocol` (if the config type is part of the wire schema). The migration path is recorded in the same changelog as the data-schema migrations.

The cost of this policy is one extra struct (`ConfigSnapshot`) and one extra field on the genesis event. The cost of *not* having it is a system that "works on my machine, in my environment, with my env vars" and silently fails to replay on a partner's host — exactly the failure mode that makes a trust chain meaningless.

## Randomness Namespace Policy

The Determinism Policy says the only acceptable source of randomness in the kernel is a seeded PRNG whose seed is part of the event payload. That sentence hides three decisions that, left implicit, will diverge at the worst possible time.

1. **Seed generation has a single owner: the event emitter.** The actor that emits an event is responsible for the seed if the event has a random component. The kernel does not "ask for a random number" — the kernel accepts a seed as part of the event payload, and the seed is canonical. The replay reads the seed, not the current time.
2. **Seeds are event-local by default.** Each event's randomness derives from a seed in that event's payload. A stream-global seed (one per process, or one per chain) is forbidden unless explicitly required — event-local seeds make individual events self-contained and replayable in isolation. (M1+ decides if any subsystem genuinely needs stream-global randomness; if it does, the seed is a `genesis_event_payload` field, not a process-global.)
3. **The PRNG algorithm is ADR-locked.** The kernel picks one PRNG (M1+ decides: `chacha20` block-cipher keystream is the tentative default — fast, deterministic, portable) and the choice is locked by an ADR. Changing the PRNG is a breaking change to the trust chain (replay produces different bytes) and is treated with the same gravity as changing the hash function (Deterministic Serialization Rule §5). The PRNG is exposed via the `ClockSource`-style seam: a `RandomSource` trait in `apw-protocol`, with `ReplayRandomSource` (reads from event payload) and `LiveRandomSource` (reads from a seeded OS source) as the M1+ impls.
4. **The PRNG is not used in canonical serialization.** The canonical serializer (Deterministic Serialization Rule) is purely deterministic — no salt, no nonce, no randomized encoding. A test asserts the canonical hash of a value is byte-for-byte identical across N runs of the same code on the same input.
5. **Forbidden in the kernel:** `rand::thread_rng()`, `getrandom::getrandom()`, `OsRng`, `fastrand`, any `lazy_static` PRNG. Allowed: a `RandomSource` injected at construction, or a seed parameter on a function. The forbidden-imports test in Layer 2 covers the common tokens.

This policy mirrors the Time Policy: a kernel primitive is promoted to a trait with a clear seam, the standard-library version is forbidden in the kernel, and the replay path is mechanically identical to the live path. The cost is one trait and one ADR; the cost of *not* having it is a trust chain that "almost" replays — different bytes for any operation that touched randomness, undetectable until production.

## Async Policy

Rust has many ways to run async code, and most of them are wrong for a long-lived server. The policy prevents the workspace from accidentally ending up with three competing runtimes, two thread pools, and a hidden executor somewhere.

1. **`apw-server` owns the only Tokio runtime.** The axum server is the single entry point that calls `tokio::main` and configures the runtime (worker threads, blocking pool). Every other binary (`apw-cli`, future `apw-office` TUI) that needs async also owns its own runtime, but they are **leaf binaries** — they call into `apw-server` over HTTP, not into `apw-kernel`/`apw-engine` directly.
2. **Libraries do not start a runtime.** `apw-kernel`, `apw-engine`, `apw-store`, `apw-protocol`, `apw-pixel-plugin` are **runtime-agnostic** in M0. They expose `async fn` only where the API is genuinely I/O-bound (storage read, network call). For the skeleton (M0), every function is sync; async signatures appear in M1+ when the I/O they wrap actually exists.
3. **No hidden executors.** No `tokio::spawn` from inside a library, no `block_on` from inside a library, no `smol`/`async-std`/`embassy`/`pollster` mixed in. If a library needs to do work concurrently, it returns a `Future` and lets the caller decide the executor.
4. **Blocking work is marked.** Any function that does CPU-heavy or blocking-I/O work exposes a `*_blocking` variant and is called via `tokio::task::spawn_blocking` by the caller, never from an async context directly.
5. **The async boundary is the network boundary.** `apw-server` ↔ `apw-office`/`apw-manager`/`apw-gateway`/`apw-cli` is the only async boundary in M0. It is `tokio::TcpStream` under HTTP. Internal calls inside the server (server → kernel → engine → store) are sync `async fn` over the same runtime, not separate processes.

This policy is a governance rule, not an M0 implementation detail. The skeleton has no async code, but the rule is locked so the M1+ implementation cannot accidentally fragment the runtime model.

## Panic & Failure Policy

A hash-chained event store, a trust replay, and a long-running async server are exactly the places where partial failures cause silent corruption. The policy is governance-level — concrete implementation lands in M1, but the rules are locked now.

1. **The kernel does not panic on invalid input.** Invalid events, malformed chains, out-of-range lease IDs, replay mismatches → returned as `Err(InvalidEvent | InvalidChain | InvalidLease | …)`. The kernel is a library; it propagates errors to the caller. The caller decides whether to reject, retry, or quarantine.
2. **Invalid events are rejected, not undefined behavior.** Every event the kernel ingests is validated against the schema and the current chain state. A malformed event is rejected *before* any state mutation. There is no code path where an unvalidated event reaches the chain.
3. **Panics do not produce partial commits.** Every state-mutating function is either **all-or-nothing** or **explicitly two-phase** (intent → commit). If a function panics between phase 1 and phase 2, the on-disk state is identical to the pre-call state. The chain append and the snapshot write are part of the same atomic unit.
4. **Event append + snapshot are atomic.** A successful `append_event` MUST also have durably committed the post-event snapshot. The two are not two separate writes; the implementation chooses the mechanism (WAL, copy-on-write, transactional store) but the *invariant* is mechanical: a chain length is always backed by a snapshot at that length.
5. **`apw-server` is the only process allowed to crash on unrecoverable errors.** When the server hits a state from which it cannot recover (corrupted store, unhandlable kernel invariant violation), it logs the full chain state, flushes the snapshot, and exits non-zero. A supervisor (systemd, k8s, a parent process) restarts it. The server does **not** try to "self-heal" — that hides the bug. The kernel is a library; it cannot crash the process.
6. **Client crates degrade, never silently lie.** If `apw-office` or `apw-webui` loses the connection to `apw-server`, the UI shows "disconnected" — it does not freeze, does not show stale data as fresh, does not fabricate events. A reconnect attempt is logged.
7. **Errors carry chain context.** Every `Err` returned by the kernel includes the `event_index` (or the relevant chain anchor) that triggered the failure. The trust report can correlate the error to the offending event without re-parsing the whole log.

These rules are testable: M1+ ships a `tests/chaos/` suite that feeds the kernel malformed events, truncated chains, double-appends, and snapshot-write failures, and asserts the invariant holds.

## Event Versioning Policy

`apw-protocol` is the wire schema for everything: HTTP requests/responses, SSE event stream, replay log format, and the LLM intent types (M3). Without explicit versioning, the system rots the first time a field is added without coordination. The policy is locked now, implementation in M1+.

1. **Every versioned type carries an explicit version field at the envelope level.** The `EventEnvelope` (and every equivalent top-level wrapper that crosses the network or the trust chain) carries `schema_version: u32`. The `Event` payload enum does **not** carry a version field — the envelope owns versioning, the payload is pure data. A wrapper without a `schema_version` field is forbidden (enforced by a `cargo test` in `apw-protocol`).
2. **Additive changes are non-breaking.** Adding a new field with a default value, adding a new `Event` variant, adding a new optional field — all backward-compatible. Old clients ignore the new field; new clients tolerate its absence.
3. **Breaking changes bump the major version of `apw-protocol`.** Removing a field, renaming a variant, changing the `serde` tag scheme, changing the meaning of an existing field — all require `apw-protocol` to bump its major `Cargo.toml` version and ship a migration guide in `docs/CHANGELOG_apw-protocol.md`.
4. **Deprecation window is one major version.** A field marked deprecated in `0.x` may be removed in `0.(x+1)`. Deprecated fields still serialize/deserialize; the server logs a deprecation warning on read. A 2-major-version deprecation window requires an ADR.
5. **Replay migration is explicit.** The replay log records `schema_version` at each event. When the kernel replays an old-version event, the M1+ migration layer normalizes it to the current version before applying it. Migration functions are themselves versioned and part of the trust chain — a replay must produce identical state regardless of which migration path was used.
6. **Wire-format stability is contractual, not accidental.** A `cargo test` in `apw-protocol` snapshots the `serde_json` output of every public type with `schema_version = N` and asserts it does not change without an explicit version bump. This catches "I just renamed a field" accidents in CI.

The M0 skeleton ships the `schema_version` field type and the snapshot test, with a single canonical version (`1`). The migration layer, the deprecation machinery, and the changelog are M1+ work — but the data shape is locked now so M1+ cannot accidentally ship a schema that cannot be versioned.

## Capability Policy

The system has multiple actors (the kernel, the LLM admin, the scheduler, an agent, an external plugin) and multiple sensitive operations (promote an agent, allocate a lease, submit a sprite proposal, modify the authority map). Without an explicit capability model, "who is allowed to do what" becomes implicit — encoded in a `if role == "CEO"` check somewhere, a hard-coded rule in a handler, a string comparison in the dispatcher. The policy locks the shape of the answer now, even though the implementation lands in M3+.

1. **Every sensitive operation is gated by a `Capability` enum variant.** Not a role string, not a boolean flag, not a `has_lease` check. The set of capabilities is the source of truth; everything else is derived. M1+ ships a `Capability` enum with at least these variants, all deriving `Ord` so that canonical ordering is mechanically enforced:
   - `Capability::PromoteAgent { from_floor, to_floor }`
   - `Capability::AllocateLease { task_id, ttl_ticks }`
   - `Capability::SubmitSpriteProposal { pack, frame }`
   - `Capability::ModifyAuthorityMap` — only the human operator at boot
   - `Capability::RunTowerAdmin` — only the LLM admin path (M3)
   - `Capability::ReplayChain` — only the replay tool
   The set is open; additions go through an ADR.
2. **Authority is `BTreeMap<ActorId, BTreeSet<Capability>>`, not a role check.** The kernel holds a single authority map, loaded at boot from a static config file (per the Determinism Policy — static config, part of the trust chain). Every operation looks up the actor in the map and asserts the required `Capability` is present. There is no `if actor == X` anywhere outside the lookup. **The collection types are `BTreeMap` and `BTreeSet` because the authority map is replay-authoritative** (Deterministic Iteration Policy applies; an earlier draft of this section used `HashMap`/`HashSet`, which was a self-contradiction with that policy and is now corrected).
3. **The LLM admin is treated as an actor with a narrow capability set.** The LLM is *not* trusted by default. It receives only the capabilities its operator-scoped config grants — typically `RunTowerAdmin` plus the read-only capabilities needed to produce suggestions. Promoting an agent, allocating a lease, or submitting a sprite proposal is a separate capability that the LLM must explicitly hold; the operator can grant or revoke it.
4. **Capabilities are checked at the boundary, not deep in the call stack.** The check happens in the kernel entry point for the operation, *before* any state mutation. A function that does the work does not also check the capability — the caller is responsible. (This mirrors the "validate at the boundary" pattern from the Panic & Failure Policy.)
5. **Capability denials are logged as events.** A denied capability is an `Event::CapabilityDenied { actor, capability, reason }` in the chain, with **typed fields** — `capability: Capability` (not a string) and `reason: DenialReason` (a closed enum, not a string), so denials are analyzable, migratable, and free of typo drift. It is *not* a panic, not a 500, not silent. A history of "the LLM tried to promote agent X 400 times in 10 minutes" is a real, replayable signal.
6. **The authority map is itself versioned.** Per the Event Versioning Policy, the authority map is `schema_version`-stamped. A replay that hits an old-version authority map applies the M1+ migration layer. The kernel never silently uses a different authority than the one recorded in the chain.

M0 ships the `Capability` enum (with `Ord` derived), the `DenialReason` enum, and the `AuthorityMap` type alias (`BTreeMap<ActorId, BTreeSet<Capability>>`, where the key is the actor's stable id). All `Serialize`/`Deserialize`. No logic. M1+ adds the lookup, the denial logging, the static config parser, and the migration layer. Locking the shape now — and locking the collection types in particular — prevents both the implicit-`if`-check rot and the later `HashMap`-in-replay-authoritative-state drift.

## Workspace Layout

```
apw-rs/
├── Cargo.toml                          # workspace manifest, resolver = "2", MSRV = 1.82
├── rust-toolchain.toml                 # pinned toolchain (1.82.x) for `cargo` + `rustfmt` + `clippy`
├── README.md                           # monorepo overview
├── .github/workflows/ci.yml            # build + test + clippy + fmt on linux/macos
├── docs/
│   ├── ROADMAP.md                      # M0..M5 milestone breakdown
│   ├── EVENTS.md                       # apw-protocol event schema (links to docs in code)
│   └── PIXEL.md                        # Aseprite → apw-pixel-plugin workflow
├── crates/
│   │
│   │   ── shared ──
│   ├── apw-protocol/                   # wire types (serde). SERVER + CLIENT depend on this.
│   │
│   │   ── SERVER (zero client deps) ──
│   ├── apw-kernel/                     # state, hash-chain, trust, replay, freezer, algebra; owns canonical serialization
│   ├── apw-engine/                     # agents, mailbox, economy, scheduler, sandbox, llm-router
│   ├── apw-store/                      # trait + adapters: memory, fs, sqlite. Owns durability only.
│   └── apw-server/                     # axum binary: composes kernel+engine+store, serves HTTP/JSON
│   │
│   │   ── CLIENT (zero server deps) ──
│   ├── apw-office/                     # ratatui TUI: embeds pixtuoid-core + bidding loop, speaks to server
│   ├── apw-manager/                    # ratatui TUI: opencode-manager-equivalent
│   ├── apw-gateway/                    # static-file server + reverse proxy to apw-server. (Originally named `apw-webui`; renamed because the crate is operationally a gateway/proxy, not a UI. See ADR-0001.)
│   └── apw-pixel-plugin/               # Aseprite export parser, used by office + gateway
│   │
│   └── tools/
│       └── apw-cli/                    # top-level `apw` binary: `apw office`, `apw replay`, `apw status`
```

**Total: 8 internal crates + 1 binary.** No catch-all crate. Every crate has a single, testable reason to exist.

## Per-Crate Skeleton

Every crate ships with **3 files** in the skeleton:

```
crates/<name>/
├── Cargo.toml                          # name, version = "0.1.0", edition = "2021", deps
├── README.md                           # what this crate becomes, current state
└── src/
    └── lib.rs                          # pub fn name() -> &'static str + 1 future-shaped struct
```

No business logic, no I/O. The public surface in M0 is exactly:
- `pub fn name() -> &'static str` — returns the crate name (smoke check that the crate links)
- One struct that hints at the future shape (e.g. `pub struct KernelState { chain_len: u64, last_hash: String }`)

This makes the skeleton reviewable in a single sitting and makes the milestone-based implementation obvious.

## Boundary Tests (the architecture enforcement)

Every crate gets a `tests/boundary.rs` test suite with three layers of mechanical enforcement:

### Layer 1: Forbidden `Cargo.toml` deps (architectural boundary)

Greps the crate's own `Cargo.toml` for forbidden crate names. A violation is almost always a wrong-direction dependency.

| Crate | Forbidden deps in `Cargo.toml` |
|---|---|
| `apw-protocol` | async runtimes: `tokio`, `smol`, `async-std`, `mio`, `pollster`; HTTP/networking: `reqwest`, `ureq`, `hyper`, `axum`, `warp`, `actix-web`, `tonic`, `libp2p`; filesystem & runtime I/O: `tokio::fs`, `async-fs`, `notify`, `walkdir`, `tempfile`, `dirs`, `home`, `directories`; clock: `chrono`, `time`, `tokio::time`; randomness: `rand`, `getrandom`, `fastrand`, `nanorand`; env/config leaks: `dotenvy`, `config`; plus any other `apw-*` crate. |
| `apw-kernel` | any `apw-office`, `apw-manager`, `apw-gateway`, `apw-pixel-plugin` |
| `apw-engine` | any `apw-office`, `apw-manager`, `apw-gateway`, `apw-pixel-plugin` |
| `apw-store` | any `apw-office`, `apw-manager`, `apw-gateway`, `apw-pixel-plugin` |
| `apw-server` | any `apw-office`, `apw-manager`, `apw-gateway`, `apw-pixel-plugin` |
| `apw-office` | `apw-server`, `apw-kernel`, `apw-engine`, `apw-store` |
| `apw-manager` | `apw-server`, `apw-kernel`, `apw-engine`, `apw-store` |
| `apw-gateway` | `apw-server`, `apw-kernel`, `apw-engine`, `apw-store` |
| `apw-pixel-plugin` | `apw-server`, `apw-kernel`, `apw-engine`, `apw-store` |

### Layer 2: Forbidden imports in `src/` (governance enforcement)

Greps the crate's `src/**/*.rs` for forbidden tokens. The implementation walks every `src/**/*.rs` file recursively (using stdlib `std::fs::read_dir` — no external dep, so the test itself does not introduce a forbidden dep) and concatenates their contents into a single string before grepping. Walking only `src/lib.rs` is insufficient: a contributor who creates `src/time.rs` or `src/replay/clock.rs` would otherwise bypass the test. The list is per-crate; the global rules are:

| Token | Forbidden in | Reason |
|---|---|---|
| `std::time::SystemTime`, `std::time::Instant`, `chrono::Utc`, `tokio::time::Instant` | `apw-kernel` | Time Policy |
| `HashMap`, `HashSet` | `apw-kernel`, `apw-protocol` | Deterministic Iteration Policy |
| `f32`, `f64` | `apw-kernel`, `apw-protocol` | Deterministic Serialization Rule |
| `serde_json::Value`, `serde_json::Map`, `serde_value::Value` | `apw-kernel`, `apw-protocol` | Deterministic Serialization Rule — typed boundary only, see §1.5 below |
| `tokio::spawn`, `block_on`, `smol`, `async_std`, `pollster` | any library crate (server + kernel + engine + store + protocol + pixel-plugin) | Async Policy |
| `AuthorityMap::insert`, `AuthorityMap::remove`, `AuthorityMap::append`, `grant_capability`, `revoke_capability` | outside `apw-kernel` (specifically outside `apw_kernel::authority`) | Capability Policy — see §2 below |

**§1 Why "no `serde_json::Value` anywhere" instead of "in canonical path":** The earlier wording "in canonical path" is semantically correct but mechanically un-testable by a string grep — a grep can find `serde_json::Value` or `Value` in source, but cannot tell whether the occurrence is on a canonical path or on a display path. The test is therefore intentionally *härter* than the policy: any `serde_json::Value` token in `apw-kernel` or `apw-protocol` fails the build. If a future use case genuinely requires a dynamic JSON tree (e.g. a generic subsystem-payload bridge), the path forward is: (a) a new boundary-conversion crate that translates between typed `BTreeMap<String, KvValue>` (Deterministic Serialization Rule §4's `KvValue` enum) and `serde_json::Value` *outside* the kernel; (b) a typed wrapper, never `Value`; or (c) an ADR that explicitly carves out a single seam. The grep test stays clean; the burden of the ADR is the price of admitting an untyped blob. The same reasoning applies to `serde_json::Map` and `serde_value::Value` (the latter being the non-`serde_json` flavor of the same anti-pattern).

**§2 Why "no `AuthorityMap::insert`" instead of "no `Capability::`":** The Capability Policy is not "nobody may reference the `Capability` enum" — that would be brittle and wrong, because `apw-engine` needs to *read* capabilities (e.g. "does this agent hold `AllocateLease`?"), `apw-server` needs to *serialize* and *log* them, and `apw-gateway` needs to *display* them. The actual architectural invariant is narrower and stronger: **only `apw-kernel` may mutate the authority map.** Reading is unrestricted; writing is gated. The grep test therefore targets mutation tokens (`insert`, `remove`, `append`, `grant_capability`, `revoke_capability`) on `AuthorityMap` rather than reference tokens on `Capability`. This is the difference between enforcing *the policy that matters* and enforcing *a token that resembles the policy* — a distinction the rest of this spec tries to make at every level.

### Layer 3: Wire-format snapshot (schema stability)

`apw-protocol` (and later `apw-kernel`) takes every public type, serializes it canonically, and asserts the byte sequence matches a checked-in fixture. Layer 3 lands in M1+ when the canonical serializer exists; the test scaffold is M0.

Implementation of Layer 1 (~10 lines per test):
```rust
#[test]
fn boundary_respected() {
    let toml = include_str!("../Cargo.toml");
    for forbidden in ["apw-server", "apw-kernel", "apw-engine", "apw-store"] {
        assert!(
            !toml.contains(&format!("\"{forbidden}\"")),
            "crate `apw-X` must not depend on forbidden crate `{forbidden}`"
        );
    }
}
```

Implementation of Layer 2 (ugly, cheap, M0-effective). Note: no `walkdir` dep — stdlib recursion keeps the test itself free of forbidden deps:
```rust
fn collect_rs_files(dir: &std::path::Path, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(s) = std::fs::read_to_string(&path) {
                out.push_str(&s);
            }
        }
    }
}

#[test]
fn no_systemtime_in_kernel() {
    let mut src = String::new();
    collect_rs_files(std::path::Path::new("src"), &mut src);
    assert!(
        !src.contains("SystemTime"),
        "apw-kernel must not use SystemTime; use ClockSource"
    );
}
```

Layer 1 + Layer 2 together catch architectural and governance drift at the speed of a `git push`. They are intentionally dumb (grep, not AST analysis); M1+ replaces them with `cargo metadata` graph validation (for Layer 1) and `clippy::disallowed_types` (for Layer 2).

Plus 1 trivial test per crate (`assert_eq!(name(), "apw-kernel")`). Total: ~16 tests, all run by `cargo test --workspace`.

## apw-protocol Wire Schema (sketch)

Pure `serde` types. The exact field lists are finalized in the M1 spec; this is the skeleton shape, with all known Deterministic Serialization, Iteration, and Capability self-consistency issues already fixed:

```rust
// identity
pub struct AgentId(pub String);
pub struct LeaseId(pub String);
pub struct ActorId(pub String);         // who emitted the event: an agent, "kernel", "llm-admin", "operator"

// Role is an enum, not a freeform string — typo drift is otherwise
// inevitable and migrations become guesswork. `Custom(String)` is the
// escape hatch for forward compatibility.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Implementation,
    Research,
    Review,
    Planning,
    Ceo,
    Sandbox,
    Economist,
    ReplayAgent,
    TrustAgent,
    Custom(String),
}

// Tick is a semantic newtype. Derives are locked now; conversions and
// `Deref` are explicitly NOT implemented, so a `Tick` cannot silently
// degrade to a `u64`.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u64);

#[derive(Serialize, Deserialize)]
pub struct AgentState {
    pub agent_id: AgentId,
    pub role: Role,
    pub floor: u8,
    pub desk_id: u8,
    pub has_lease: bool,
    pub blocked: bool,
    pub reputation_milli: u32,           // 0..=1000; f32 is forbidden (see Deterministic Serialization)
    pub wallet: u64,
    pub current_expression: Expr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr { Idle, Working, Thinking, Blocked, Walking, Seated, Sleeping, Custom(String) }

#[derive(Serialize, Deserialize)]
pub struct SubsystemState {
    pub room_id: String,
    pub floor: u8,
    pub status: Status,
    pub heat: u8,
    // NO `serde_json::Value` here — see Deterministic Serialization Rule §4.
    // Subsystem payloads are typed per-subsystem (e.g. enum variant or
    // `BTreeMap<String, KvValue>`); the M0 skeleton uses a typed enum
    // to keep the wire schema concrete.
    pub payload: SubsystemPayload,
}

#[derive(Serialize, Deserialize)]
pub enum SubsystemPayload {
    Algebra { chain_valid: bool, chain_length: u64 },
    Scheduler { queue_size: u64, interval_ticks: u64 },
    Economy { bids: u64, multiplier_milli: u32 },  // multiplier stored as fixed-point
    Sandbox { live_pids: u64 },
    GenericKv { entries: BTreeMap<String, KvValue> },  // escape hatch, canonical
}

#[derive(Serialize, Deserialize)]
pub enum KvValue { Int(i64), Uint(u64), Bool(bool), Text(String) }  // no floats, no nested objects

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status { Ok, Warn, Alert, Idle }

#[derive(Serialize, Deserialize)]
pub struct BidRequest { pub agent_id: AgentId, pub target_floor: u8, pub max_price: u64 }
#[derive(Serialize, Deserialize)]
pub struct BidResult { pub accepted: bool, pub new_floor: u8, pub price_paid: u64 }

// ============= Capability + authority (Capability Policy §1-2) =============
// `Ord` is derived so `BTreeSet<Capability>` is canonically orderable.

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Capability {
    PromoteAgent { from_floor: u8, to_floor: u8 },
    AllocateLease { task_id: String, ttl_ticks: u64 },
    SubmitSpriteProposal { pack: String, frame: String },
    ModifyAuthorityMap,                 // only the human operator at boot
    RunTowerAdmin,                      // only the LLM admin path (M3)
    ReplayChain,                        // only the replay tool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DenialReason {
    MissingCapability,
    ExpiredLease,
    InvalidScope,
    AuthorityMapRejected,
    ReplayOnlyOperation,
    Other(String),                      // escape hatch with explicit name; not a freeform bag
}

/// Authority is `BTreeMap`, not `HashMap` — the authority map is
/// replay-authoritative (Deterministic Iteration Policy). Earlier
/// drafts of this spec used `HashMap`/`HashSet`, which was a
/// self-contradiction with that policy; the type is now correct.
pub type AuthorityMap = BTreeMap<ActorId, BTreeSet<Capability>>;

// ============= Event envelope + payload =============
// Versioning, ticking, and actor live in the ENVELOPE, not in the payload.
// The payload is a pure data record. Hash-addressed (Option A in the
// review): the chain is identified by its hash, not by a separate `id`.

#[derive(Serialize, Deserialize)]
pub struct EventEnvelope {
    pub schema_version: u32,             // current canonical version, see Event Versioning Policy
    pub tick: Tick,                      // replay-authoritative timestamp
    pub actor: ActorId,                  // who emitted this
    pub event_hash: Option<[u8; 32]>,    // BLAKE3 of the canonical envelope; see Open Questions §6
    pub prev_event_hash: Option<[u8; 32]>,
    pub payload: Event,                  // the typed payload below
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    AgentExpressionChanged { agent_id: AgentId, expression: Expr },
    AgentPromoted { agent_id: AgentId, from_floor: u8, to_floor: u8, cost: u64 },
    ItemPurchased { agent_id: AgentId, item_id: String, cost: u64 },
    SubsystemStatusChanged { room_id: String, status: Status, heat: u8 },
    LeaseAcquired { agent_id: AgentId, lease_id: LeaseId },
    LeaseCompleted { agent_id: AgentId, lease_id: LeaseId, success: bool },
    // Typed, not stringly-typed: `capability: Capability` and
    // `reason: DenialReason` so denials are analyzable, migratable,
    // and free of typo drift (Capability Policy §5).
    CapabilityDenied { actor: ActorId, capability: Capability, reason: DenialReason },
}
```

**Why the envelope exists:** Versioning, ticking, and actor identity are *metadata* about an event, not *part* of the event's data. Putting them in the envelope means:

- Schema versions are recorded once per event, not duplicated in every variant.
- Adding a new `Event` variant never needs to think about versioning.
- The canonical hash covers both the metadata and the payload, so a `tick` or `actor` change is a real, replayable mutation.
- Replay migration (Event Versioning Policy §5) operates on the envelope, not on every variant.

**Why `event_hash` and not `event_id`:** the chain is hash-addressed. A separate `event_id` would be either a duplicate of the hash or a position index, both of which re-introduce the ambiguity this design is trying to remove. The hash *is* the identity; the `Option<[u8; 32]>` is `None` while the event is being constructed and `Some(...)` once the canonical bytes are hashed.

No methods, no business logic. The skeleton implements the types only.

## apw-pixel-plugin Skeleton (preview)

Even though the skeleton is a stub, the *shape* of this crate matters because it dictates the asset pipeline. The skeleton will ship:

- The `SpriteResolver` trait (signature only)
- A `FileSpriteResolver` struct (stub) that loads `manifest.toml` from a directory
- A `catalog()` method that returns `Vec<SpriteDescriptor>` (empty in the skeleton)
- An `assets/manifest.toml` file documenting the expected directory layout
- An `assets/README.md` describing the Aseprite → pixel-plugin → apw-pixel-plugin export workflow
- A fixture test: load a tiny hand-written Aseprite-JSON fixture and assert that the resolver returns 1 sprite

The actual PNG decoding and Aseprite JSON parsing come in M4.

## Build, Test, Verify

- `cargo build --workspace` — must succeed clean
- `cargo test --workspace` — must pass (boundary tests + 1 smoke test per crate)
- `cargo clippy --workspace --all-targets -- -D warnings` — must be clean
- `cargo fmt --check` — must be clean
- `just verify` (or `make verify`) chains all four
- CI: GitHub Actions matrix `ubuntu-latest` × `macos-latest` × `stable` rustc

No runtime smoke test in M0 — the skeleton has no runtime.

## Out of Scope (deferred milestones)

| Milestone | Scope |
|---|---|
| **M1** | Port `kernel/event-core`, `algebra`, `freezer`, `trust-report`, `replay`, `snapshot` to `apw-kernel`. Same JSON API surface as the Node kernel so the existing Node webui keeps working. |
| **M2** | Port `engine/agents`, `mailbox`, `economy`, `scheduler`, `sandbox-executor`, `llm-router` to `apw-engine`. `apw-server` (axum) exposes the same `/api/*` endpoints. |
| **M3** | `apw-engine/llm` trait `TowerAdmin` ships. Default impl is rule-based; a real self-hosted LLM (llama.cpp / Ollama / vLLM HTTP) plugs in as a single impl. |
| **M4** | `apw-pixel-plugin` full impl (Aseprite JSON + PNG decode). `apw-office` TUI: ratatui, embeds pixtuoid-core, bidding loop. Agent-authored sprite proposal endpoint: `POST /api/apw/sprites/propose`, gated by trust chain. |
| **M5** | `apw-manager` TUI (file tree, sessions, terminals). `apw-webui` static server + reverse proxy. Gradual retirement of the Node webui. |

## Repository Bootstrap

The new repo does not yet exist on GitHub. Bootstrap steps (run by the user; this design does not assume agent push access):

```bash
# 1. Create the empty repo on GitHub (web UI, or `gh repo create forgefabrik/apw-rs --private`)
# 2. Locally:
git clone https://github.com/forgefabrik/apw-rs.git
cd apw-rs
# 3. Apply this spec:
#    - copy Cargo.toml, rust-toolchain.toml, .github/, docs/, crates/, tools/ from the
#      first commit produced by following the M0 implementation plan
# 4. Push:
git push -u origin main
```

The M0 implementation plan (separate doc, produced by the writing-plans skill) will list the exact files and order.

## Open Questions for Future Milestones

These are deliberately NOT answered in the skeleton; they are tracked here so M1+ brainstorming has a starting point:

1. **Storage backend default for M1.** `apw-store` will support `memory`, `sqlite`, `fs` from day one. The M1 default is memory (matches the Node kernel's default). Persistent storage lands in M2.
2. **HTTP framework choice for `apw-server`.** Tentative: `axum` (tower ecosystem, tokio-native, widely used). Revisit in M1 spec.
3. **TUI framework for `apw-office` and `apw-manager`.** Tentative: `ratatui` (matches pixtuoid's choice, mature). Revisit in M4 spec.
4. **Embedding `pixtuoid-core`.** This requires `pixtuoid-core` to be published as a reusable crate on crates.io. If upstream doesn't agree, we vendor it as a submodule. Decision in M4.
5. **LLM SDK for M3.** Tentative: thin HTTP client to a self-hosted endpoint (no SDK lock-in). Revisit in M3 spec.
6. **`EventId` vs `event_index`.** The event chain currently identifies events by their position (`event_index: u64`). Long-term, a stable content-addressed `EventId` (BLAKE3 hash of the canonical envelope) is desirable: position-based ids break under reorgs and migrations; content-addressed ids survive both. The M0 envelope already carries an `event_id: Option<[u8; 32]>` placeholder; M1 makes it mandatory and defines the canonical-hash preimage. M0 ships the placeholder so the field shape is locked.
7. **Splitting `apw-protocol` into focused crates.** The crate currently carries wire types, `Tick`/`ClockSource`/`SimulationClock`, `Capability`/`AuthorityMap`, event envelopes, and identity types. That is manageable in M0. By M2, it may be large enough that enforcing the "no catch-all" rule (Workspace Philosophy §2) requires a split. The candidate split, to be done via ADR if/when the size justifies it:
   - `apw-protocol` — wire + serde only (request/response types, error codes)
   - `apw-time` — `Tick`, `ClockSource`, `SimulationClock`, `RandomSource`
   - `apw-capability` — `Capability`, `DenialReason`, `AuthorityMap`
   - `apw-canonical` — the canonical serializer, used by all the above
   The split is **deferred** — the rule "crate only if boundary exists" applies. Today, a single `apw-protocol` with a strong boundary test is the right shape. Tomorrow, when `apw-time` and `apw-capability` want different release cadences or different dep sets, the split happens with an ADR.
8. **Future `apw-testkit` crate (not for M0).** By M2/M3 the test suites across `apw-kernel`, `apw-engine`, and `apw-protocol` will share deterministic harnesses: replay fixtures, `FrozenClock`/`FrozenRandomSource`, snapshot builders, fake `AuthorityMap` constructors, canonical-hash helpers. The Workspace Philosophy forbids a catch-all crate, but a focused `apw-testkit` crate (a `dev-dependencies`-only crate published as a separate path, gated on ADR) is a legitimate boundary — it lets the kernel tests stay tiny and gives one place to maintain the deterministic test primitives. **Not created in M0** — the rule "crate only if multiple crates need it" applies. The trigger is the first time two crates duplicate a test helper.

---

## Changelog

| Version | Date | Notes |
|---|---|---|
| **0.1 (brainstorming-approved)** | 2026-06-05 | Initial design. 8 crates + 1 binary. Policies: Time, Determinism (serialization + iteration), Capability, Async, Panic, Event Versioning, Randomness, Configuration. Boundary tests Layer 1 + 2 (Layer 3 deferred to M1). MSRV pinned to 1.82. |
| **0.1.1 (docs cleanup)** | 2026-06-06 | Linked from the [roadmap](../2026-06-05-apw-rs-roadmap.md) and the [M0 implementation plan](../plans/2026-06-06-m0-workspace-skeleton-implementation.md). No content change to the design itself. |
| _Future_ | _M1_ | Add Layer 3 (canonical-serialization snapshot tests), `apw-canonical` crate, replace grep boundary tests with `cargo metadata` graph validation. |

This spec is the single source of truth for crate boundaries and policies. Any change here requires an ADR under `docs/adr/`.
