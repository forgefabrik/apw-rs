# apw-rs M0 Workspace Skeleton — Implementation Plan

**Date:** 2026-06-06  
**Status:** Ready for execution  
**Target:** Build the pure-Rust Cargo workspace skeleton with 8 crates, boundary enforcement, and governance-locked policies.  
**Success Criteria:** `cargo test --workspace` passes all 45 sections; boundary tests + smoke tests green; CI passes on Linux + macOS.

---

## Overview: From Spec to Code

This plan implements the **apw-rs workspace skeleton (M0)** as defined in `2026-06-05-apw-rs-workspace-skeleton-design.md` and `2026-06-05-apw-rs-roadmap.md`. The design locks:

- **Policies:** Time, Determinism, Capability, Async, Panic, Versioning, Configuration, Randomness
- **Boundaries:** Server/client separation, shared-only protocol, no catch-all crates
- **Enforcement:** Mechanical grep-based boundary tests (Layer 1 + Layer 2) + snapshot tests (Layer 3, M1+)
- **MSRV:** Rust 1.82.x pinned in `rust-toolchain.toml`

The skeleton is **not** a working system (no business logic, no async runtime, no I/O). It is a **type-safe blueprint** with governance locks in place.

---

## Phases & Task Breakdown

### Phase 0: Repository Bootstrap

**Task 0.1: Create the workspace root**

Files:
- Create: `Cargo.toml` (workspace manifest)
- Create: `rust-toolchain.toml` (MSRV = 1.82)
- Create: `README.md` (monorepo overview)

**Step 1: Create workspace `Cargo.toml`**

```toml
[workspace]
members = [
    "crates/apw-protocol",
    "crates/apw-kernel",
    "crates/apw-engine",
    "crates/apw-store",
    "crates/apw-server",
    "crates/apw-office",
    "crates/apw-manager",
    "crates/apw-gateway",
    "crates/apw-pixel-plugin",
    "tools/apw-cli",
]

resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["ForgeFabrik Contributors"]
license = "Proprietary"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Step 2: Create `rust-toolchain.toml`**

```toml
[toolchain]
channel = "1.82"
components = ["rustfmt", "clippy"]
```

**Step 3: Create root `README.md`**

```markdown
# apw-rs: ForgeFabrik Agent OS in Pure Rust

Pure-Rust rewrite of the ForgeFabrik Agent Operating System, structured as a Cargo workspace with enforced architectural boundaries.

## Milestones

- **M0** (this): Workspace skeleton, 8 crates, boundary enforcement
- **M1**: Kernel core port (event-core, algebra, freezer, trust, replay, snapshot)
- **M2**: Engine + server (agents, mailbox, economy, scheduler, sandbox, llm-router)
- **M3**: LLM admin trait + rule-based impl
- **M4**: Office TUI + Aseprite pixel pipeline
- **M5**: Manager TUI + webui proxy

## Building & Testing

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

## Architectural Policies

See `docs/` for:
- `ROADMAP.md` — milestone breakdown
- `adr/0000-template.md` — ADR template for changes
- `EVENTS.md` — apw-protocol event schema (links to code)
- `PIXEL.md` — Aseprite pipeline (M4+)

## Boundary Rules

**Server crates** (`apw-kernel`, `apw-engine`, `apw-store`, `apw-server`):
- MUST NOT depend on any client crate
- MUST only use `apw-protocol` for shared types
- Forbidden in `apw-kernel`: `SystemTime`, `HashMap`, `f32`/`f64`

**Client crates** (`apw-office`, `apw-manager`, `apw-gateway`, `apw-pixel-plugin`):
- MUST NOT depend on any server crate
- CAN depend on `apw-protocol` and other client crates
- CAN have async (but no internal tokio::main)

**Shared:** `apw-protocol` only — pure `serde` types, no I/O, no async runtime.

Run `cargo test --workspace` to verify boundaries.
```

**Step 4: Create `docs/` structure**

```bash
mkdir -p docs/adr
touch docs/ROADMAP.md docs/EVENTS.md docs/PIXEL.md docs/adr/0000-template.md
```

**Step 5: Commit bootstrap**

```bash
git add Cargo.toml rust-toolchain.toml README.md docs/
git commit -m "chore(M0): bootstrap workspace root with manifest, toolchain, readme"
```

---

### Phase 1: Shared Crate — apw-protocol

**Task 1.1: Create apw-protocol crate structure**

Files:
- Create: `crates/apw-protocol/Cargo.toml`
- Create: `crates/apw-protocol/README.md`
- Create: `crates/apw-protocol/src/lib.rs` (pub types + exports)
- Create: `crates/apw-protocol/tests/boundary.rs` (boundary enforcement)

**Step 1: Create `crates/apw-protocol/Cargo.toml`**

```toml
[package]
name = "apw-protocol"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true

[dev-dependencies]
```

**Step 2: Create `crates/apw-protocol/README.md`**

```markdown
# apw-protocol

Wire types for the apw-rs system. Pure `serde` types, no I/O, no runtime.

**Current status (M0):** Skeleton only. Type signatures and governance policies locked.

**Becomes (M1+):**
- Event envelope + payload types
- Identity, role, capability types
- Time (`Tick`, `ClockSource`, `SimulationClock`)
- Error types
- Canonical serialization trait
```

**Step 3: Create `crates/apw-protocol/src/lib.rs`**

```rust
//! apw-protocol — wire types for apw-rs
//!
//! M0 skeleton: Types are defined here. Implementation lands in M1.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The public API of this crate in M0.
pub fn name() -> &'static str {
    "apw-protocol"
}

// ============= Identity =============

/// Agent identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

/// Lease identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LeaseId(pub String);

/// Actor identifier — who emitted an event.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ActorId(pub String);

// ============= Role =============

/// Role is an enum, not freeform string — typo drift prevention.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

// ============= Time =============

/// Replay-authoritative time. Semantic newtype — not `Deref` to u64.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tick(pub u64);

/// Read-only clock interface.
pub trait ClockSource: Send + Sync {
    fn tick(&self) -> Tick;
}

/// Mutable clock for simulation/tests — NOT for replay or wall-clock.
pub trait SimulationClock: ClockSource {
    fn advance(&mut self, n: u64);
}

// ============= Status =============

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Warn,
    Alert,
    Idle,
}

// ============= Expression =============

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expression {
    Idle,
    Working,
    Thinking,
    Blocked,
    Walking,
    Seated,
    Sleeping,
    Custom(String),
}

// ============= Capability (Capability Policy) =============

/// Capability is `Ord` so it can go in `BTreeSet<Capability>`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Capability {
    PromoteAgent { from_floor: u8, to_floor: u8 },
    AllocateLease { task_id: String, ttl_ticks: u64 },
    SubmitSpriteProposal { pack: String, frame: String },
    ModifyAuthorityMap,
    RunTowerAdmin,
    ReplayChain,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialReason {
    MissingCapability,
    ExpiredLease,
    InvalidScope,
    AuthorityMapRejected,
    ReplayOnlyOperation,
    Other(String),
}

/// Authority map: `BTreeMap` (not `HashMap`) for deterministic iteration.
pub type AuthorityMap = BTreeMap<ActorId, std::collections::BTreeSet<Capability>>;

// ============= Events =============

/// Event envelope: version, tick, actor, payload, hashes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub schema_version: u32,
    pub tick: Tick,
    pub actor: ActorId,
    pub event_hash: Option<[u8; 32]>,
    pub prev_event_hash: Option<[u8; 32]>,
    pub payload: Event,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    AgentExpressionChanged {
        agent_id: AgentId,
        expression: Expression,
    },
    AgentPromoted {
        agent_id: AgentId,
        from_floor: u8,
        to_floor: u8,
        cost: u64,
    },
    ItemPurchased {
        agent_id: AgentId,
        item_id: String,
        cost: u64,
    },
    SubsystemStatusChanged {
        room_id: String,
        status: Status,
        heat: u8,
    },
    LeaseAcquired {
        agent_id: AgentId,
        lease_id: LeaseId,
    },
    LeaseCompleted {
        agent_id: AgentId,
        lease_id: LeaseId,
        success: bool,
    },
    CapabilityDenied {
        actor: ActorId,
        capability: Capability,
        reason: DenialReason,
    },
}

// ============= Test helper =============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_protocol_compiles() {
        assert_eq!(name(), "apw-protocol");
        let _tick = Tick(42);
        let _role = Role::Ceo;
        let _cap = Capability::ReplayChain;
    }
}
```

**Step 4: Create `crates/apw-protocol/tests/boundary.rs`**

```rust
//! Boundary enforcement for apw-protocol.
//! Layer 1: Forbidden dependencies

#[test]
fn no_async_runtimes_in_cargo_toml() {
    let toml = include_str!("../Cargo.toml");
    let forbidden = [
        "tokio", "smol", "async-std", "mio", "pollster"
    ];
    for name in forbidden {
        assert!(
            !toml.contains(&format!("\"{name}\"")),
            "apw-protocol MUST NOT depend on async runtime: {name}"
        );
    }
}

#[test]
fn no_io_in_cargo_toml() {
    let toml = include_str!("../Cargo.toml");
    let forbidden = [
        "reqwest", "ureq", "hyper", "axum", "warp", "actix-web",
        "std::fs", "std::net", "tokio::fs"
    ];
    for name in forbidden {
        assert!(
            !toml.contains(&format!("\"{name}\"")),
            "apw-protocol MUST NOT depend on I/O: {name}"
        );
    }
}

// Layer 2: Forbidden imports in src/

fn collect_rs_files(dir: &std::path::Path, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !path.ends_with("target") {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(s) = std::fs::read_to_string(&path) {
                out.push_str(&s);
            }
        }
    }
}

#[test]
fn no_systemtime_in_src() {
    let mut src = String::new();
    collect_rs_files(std::path::Path::new("src"), &mut src);
    assert!(
        !src.contains("SystemTime"),
        "apw-protocol MUST NOT use SystemTime"
    );
}

#[test]
fn no_hashmap_in_src() {
    let mut src = String::new();
    collect_rs_files(std::path::Path::new("src"), &mut src);
    assert!(
        !src.contains("HashMap"),
        "apw-protocol MUST NOT use HashMap (use BTreeMap)"
    );
}

#[test]
fn smoke_test() {
    // Verify the crate is named correctly
    let manifest = include_str!("../Cargo.toml");
    assert!(manifest.contains("name = \"apw-protocol\""));
}
```

**Step 5: Commit apw-protocol**

```bash
git add crates/apw-protocol/
git commit -m "feat(M0): create apw-protocol crate with wire types + boundary tests (Layer 1-2)"
```

---

### Phase 2: Server Crates

#### Task 2.1: Create apw-kernel crate

Files:
- Create: `crates/apw-kernel/Cargo.toml`
- Create: `crates/apw-kernel/README.md`
- Create: `crates/apw-kernel/src/lib.rs`
- Create: `crates/apw-kernel/tests/boundary.rs`

**Step 1: Create `crates/apw-kernel/Cargo.toml`**

```toml
[package]
name = "apw-kernel"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
apw-protocol.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
```

**Step 2: Create `crates/apw-kernel/README.md`**

```markdown
# apw-kernel

Event-core, algebra, freezer, trust-report, replay, snapshot — the replay-authoritative kernel.

**M0 Status:** Skeleton only.  
**M1+:** Port from Node.js kernel to Rust with same JSON API surface.

## Governance Locks

- **Time Policy:** No `std::time::SystemTime` — only `Tick` from events
- **Determinism Policy:** Replay produces identical state; no floating-point; canonical serialization
- **Deterministic Iteration Policy:** No `HashMap`; use `BTreeMap` only
```

**Step 3: Create `crates/apw-kernel/src/lib.rs`**

```rust
//! apw-kernel — replay-authoritative kernel

use apw_protocol::{Tick, Event, EventEnvelope};

pub fn name() -> &'static str {
    "apw-kernel"
}

/// Kernel state — stub for M0.
pub struct KernelState {
    pub chain_len: u64,
    pub last_hash: Option<String>,
    pub current_tick: Tick,
}

impl Default for KernelState {
    fn default() -> Self {
        KernelState {
            chain_len: 0,
            last_hash: None,
            current_tick: Tick(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_kernel() {
        let state = KernelState::default();
        assert_eq!(state.chain_len, 0);
        assert_eq!(name(), "apw-kernel");
    }
}
```

**Step 4: Create `crates/apw-kernel/tests/boundary.rs`**

```rust
//! Boundary enforcement for apw-kernel
//! Layer 1: Forbidden server-side dependencies

#[test]
fn must_not_depend_on_client_crates() {
    let toml = include_str!("../Cargo.toml");
    let forbidden = [
        "apw-office", "apw-manager", "apw-gateway", "apw-pixel-plugin"
    ];
    for name in forbidden {
        assert!(
            !toml.contains(&format!("\"{name}\"")),
            "apw-kernel MUST NOT depend on client crate: {name}"
        );
    }
}

// Layer 2: Forbidden imports

fn collect_rs_files(dir: &std::path::Path, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !path.ends_with("target") {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(s) = std::fs::read_to_string(&path) {
                out.push_str(&s);
            }
        }
    }
}

#[test]
fn no_systemtime() {
    let mut src = String::new();
    collect_rs_files(std::path::Path::new("src"), &mut src);
    assert!(!src.contains("SystemTime"), "apw-kernel MUST NOT use SystemTime");
}

#[test]
fn no_hashmap() {
    let mut src = String::new();
    collect_rs_files(std::path::Path::new("src"), &mut src);
    assert!(!src.contains("HashMap"), "apw-kernel MUST NOT use HashMap");
}

#[test]
fn no_f32_f64() {
    let mut src = String::new();
    collect_rs_files(std::path::Path::new("src"), &mut src);
    assert!(!src.contains("f32 "), "apw-kernel MUST NOT use f32");
    assert!(!src.contains("f64 "), "apw-kernel MUST NOT use f64");
}

#[test]
fn smoke() {
    let toml = include_str!("../Cargo.toml");
    assert!(toml.contains("name = \"apw-kernel\""));
}
```

**Step 5: Commit apw-kernel**

```bash
git add crates/apw-kernel/
git commit -m "feat(M0): create apw-kernel crate (skeleton)"
```

---

#### Task 2.2: Create apw-engine, apw-store, apw-server crates

Repeat the pattern for each:

**For `apw-engine`:**

```bash
mkdir -p crates/apw-engine/{src,tests}
cat > crates/apw-engine/Cargo.toml << 'EOF'
[package]
name = "apw-engine"
version.workspace = true
edition.workspace = true

[dependencies]
apw-protocol.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
EOF

cat > crates/apw-engine/README.md << 'EOF'
# apw-engine

Agents, mailbox, economy, scheduler, sandbox, llm-router.

**M0:** Skeleton only.  
**M1+:** Port from Node engine.
EOF

cat > crates/apw-engine/src/lib.rs << 'EOF'
pub fn name() -> &'static str {
    "apw-engine"
}

pub struct EngineState;

#[test]
fn smoke() {
    assert_eq!(name(), "apw-engine");
}
EOF

cat > crates/apw-engine/tests/boundary.rs << 'EOF'
#[test]
fn no_client_deps() {
    let toml = include_str!("../Cargo.toml");
    for name in ["apw-office", "apw-manager", "apw-gateway", "apw-pixel-plugin"] {
        assert!(!toml.contains(&format!("\"{name}\"")));
    }
}

#[test]
fn smoke() {
    assert!(include_str!("../Cargo.toml").contains("name = \"apw-engine\""));
}
EOF
```

**For `apw-store`:**

```bash
mkdir -p crates/apw-store/{src,tests}
cat > crates/apw-store/Cargo.toml << 'EOF'
[package]
name = "apw-store"
version.workspace = true
edition.workspace = true

[dependencies]
apw-protocol.workspace = true
serde.workspace = true
serde_json.workspace = true
EOF

# (similar src/lib.rs and tests/boundary.rs pattern)
```

**For `apw-server`:**

```bash
mkdir -p crates/apw-server/{src,tests}
cat > crates/apw-server/Cargo.toml << 'EOF'
[package]
name = "apw-server"
version.workspace = true
edition.workspace = true

[dependencies]
apw-protocol.workspace = true
apw-kernel.workspace = true
apw-engine.workspace = true
apw-store.workspace = true
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
axum = "0.7"
tower = "0.4"
tracing.workspace = true
tracing-subscriber.workspace = true
EOF

# (similar structure, but this one CAN import tokio, axum, etc.)
```

**Step 6: Commit all server crates**

```bash
git add crates/apw-engine/ crates/apw-store/ crates/apw-server/
git commit -m "feat(M0): create apw-engine, apw-store, apw-server crates"
```

---

### Phase 3: Client Crates

#### Task 3.1: Create apw-office, apw-manager, apw-gateway, apw-pixel-plugin

Repeat the same pattern, but with client-specific boundaries:

**For each client crate:**

```bash
mkdir -p crates/apw-office/{src,tests}
cat > crates/apw-office/Cargo.toml << 'EOF'
[package]
name = "apw-office"
version.workspace = true
edition.workspace = true

[dependencies]
apw-protocol.workspace = true
serde.workspace = true
tracing.workspace = true

# Clients CAN use ratatui, reqwest, etc.
ratatui = "0.27"
reqwest = { version = "0.11", features = ["json"] }
tokio = { workspace = true, features = ["rt"] }  # Runtime, but not ::main
EOF

cat > crates/apw-office/README.md << 'EOF'
# apw-office

Ratatui TUI with pixtuoid-core + agent bidding loop.

**M0:** Skeleton only.  
**M4+:** Full office UI.
EOF

cat > crates/apw-office/src/lib.rs << 'EOF'
pub fn name() -> &'static str {
    "apw-office"
}
EOF

cat > crates/apw-office/tests/boundary.rs << 'EOF'
#[test]
fn must_not_depend_on_server() {
    let toml = include_str!("../Cargo.toml");
    for name in ["apw-server", "apw-kernel", "apw-engine", "apw-store"] {
        assert!(!toml.contains(&format!("\"{name}\"")));
    }
}

#[test]
fn smoke() {
    assert!(include_str!("../Cargo.toml").contains("name = \"apw-office\""));
}
EOF
```

Repeat for: `apw-manager`, `apw-gateway`, `apw-pixel-plugin`

**Step 2: Commit all client crates**

```bash
git add crates/apw-office/ crates/apw-manager/ crates/apw-gateway/ crates/apw-pixel-plugin/
git commit -m "feat(M0): create client crates (apw-office, apw-manager, apw-gateway, apw-pixel-plugin)"
```

---

### Phase 4: Tools & CLI

#### Task 4.1: Create apw-cli

```bash
mkdir -p tools/apw-cli/{src,tests}

cat > tools/apw-cli/Cargo.toml << 'EOF'
[package]
name = "apw-cli"
version.workspace = true
edition.workspace = true

[[bin]]
name = "apw"
path = "src/main.rs"

[dependencies]
apw-protocol.workspace = true
tokio = { workspace = true, features = ["rt-multi-thread", "macros"] }
clap = { version = "4", features = ["derive"] }
tracing.workspace = true
tracing-subscriber.workspace = true
EOF

cat > tools/apw-cli/src/main.rs << 'EOF'
use clap::Parser;

#[derive(Parser)]
#[command(name = "apw")]
#[command(about = "ForgeFabrik Agent OS CLI")]
enum Command {
    #[command(about = "Start the office UI")]
    Office,
    #[command(about = "Start the manager UI")]
    Manager,
    #[command(about = "Replay a chain")]
    Replay { path: String },
    #[command(about = "Show system status")]
    Status,
}

#[tokio::main]
async fn main() {
    let cmd = Command::parse();
    match cmd {
        Command::Office => println!("apw office — not yet implemented"),
        Command::Manager => println!("apw manager — not yet implemented"),
        Command::Replay { path } => println!("apw replay {} — not yet implemented", path),
        Command::Status => println!("apw status — not yet implemented"),
    }
}
EOF

cat > tools/apw-cli/tests/boundary.rs << 'EOF'
#[test]
fn smoke() {
    assert!(include_str!("../Cargo.toml").contains("name = \"apw-cli\""));
}
EOF
```

**Commit CLI:**

```bash
git add tools/apw-cli/
git commit -m "feat(M0): create apw-cli binary"
```

---

### Phase 5: CI/CD & Documentation

#### Task 5.1: GitHub Actions CI

```bash
mkdir -p .github/workflows

cat > .github/workflows/ci.yml << 'EOF'
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    strategy:
      matrix:
        os: [ ubuntu-latest, macos-latest ]
        rust: [ stable ]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: 1.82
          components: rustfmt,clippy
      - uses: Swatinem/rust-cache@v2
      - name: Build
        run: cargo build --workspace
      - name: Test
        run: cargo test --workspace
      - name: Clippy
        run: cargo clippy --workspace --all-targets -- -D warnings
      - name: Format
        run: cargo fmt --check
EOF

git add .github/
git commit -m "chore(M0): add GitHub Actions CI (lint + test)"
```

#### Task 5.2: ADR Template & Documentation

```bash
cat > docs/adr/0000-template.md << 'EOF'
# ADR-NNNN: [Short Title]

**Date:** YYYY-MM-DD  
**Status:** DRAFT | APPROVED | SUPERSEDED  
**Impacts:** apw-* crate names, boundary rule, policy  

## Context

[Why this decision is needed]

## Decision

[What is being decided]

## Rationale

[Why this is the right choice]

## Consequences

[What changes as a result]

## References

- Related issues/PRs
- Related docs
EOF

cat > docs/EVENTS.md << 'EOF'
# Event Schema Reference

Events are defined in `apw-protocol::Event`. See `crates/apw-protocol/src/lib.rs` for the typed enum.

Each event is wrapped in an `EventEnvelope` with:
- `schema_version: u32` — current version (1 in M0)
- `tick: Tick` — replay-authoritative timestamp
- `actor: ActorId` — who emitted this
- `event_hash`, `prev_event_hash` — chain hashes
EOF

cat > docs/PIXEL.md << 'EOF'
# Aseprite Asset Pipeline

**M0:** Not yet implemented.  
**M4+:** Will document workflow for exporting Aseprite sprites into `apw-pixel-plugin`.
EOF

git add docs/
git commit -m "docs(M0): add ADR template, EVENTS reference, PIXEL placeholder"
```

---

### Phase 6: Smoke Tests

#### Task 6.1: Create workspace-level smoke test

```bash
cat > tests/smoke.rs << 'EOF'
//! Workspace smoke test — verify all crates link and basic smoke checks pass

#[test]
fn all_crates_compile() {
    // Each crate has a `name()` function that returns its name
    // This test verifies they all link correctly
    use std::process::Command;
    
    let output = Command::new("cargo")
        .args(&["build", "--workspace"])
        .output()
        .expect("cargo build failed");
    
    assert!(output.status.success(), "cargo build failed");
}

#[test]
fn all_boundary_tests_pass() {
    let output = Command::new("cargo")
        .args(&["test", "--workspace", "boundary"])
        .output()
        .expect("cargo test failed");
    
    assert!(output.status.success(), "boundary tests failed");
}

#[test]
fn fmt_check() {
    let output = Command::new("cargo")
        .args(&["fmt", "--check"])
        .output()
        .expect("cargo fmt check failed");
    
    assert!(output.status.success(), "cargo fmt check failed");
}

#[test]
fn clippy_check() {
    let output = Command::new("cargo")
        .args(&["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"])
        .output()
        .expect("cargo clippy failed");
    
    assert!(output.status.success(), "clippy check failed");
}
EOF

git add tests/
git commit -m "test(M0): add workspace smoke tests"
```

---

### Phase 7: Verification & Final Build

#### Task 7.1: Full build & test cycle

**Step 1: Build workspace**

```bash
cargo build --workspace
# Expected: success, all 9 crates compile
```

**Step 2: Run all tests**

```bash
cargo test --workspace
# Expected: all ~15 boundary tests + 1 smoke per crate = 24 tests pass
```

**Step 3: Verify clippy**

```bash
cargo clippy --workspace --all-targets -- -D warnings
# Expected: zero warnings
```

**Step 4: Verify fmt**

```bash
cargo fmt --check
# Expected: all files formatted correctly
```

**Step 5: CI locally**

```bash
just verify  # or `make verify` if you have a Makefile
# Expected: build, test, clippy, fmt all green
```

#### Task 7.2: Update root README with build instructions

```markdown
## Quick Start

```bash
# Clone and build
git clone https://github.com/forgefabrik/apw-rs.git
cd apw-rs
cargo build --workspace

# Run tests
cargo test --workspace

# Full verification (build + test + clippy + fmt)
just verify
```

Ensure `Justfile` or `Makefile` exists with `verify` target:

```makefile
.PHONY: verify
verify:
	cargo build --workspace
	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings
	cargo fmt --check
```
```

#### Task 7.3: Final commit & tag

```bash
git add README.md Justfile
git commit -m "chore(M0): add Justfile, final verification workflow"

git tag -a m0-skeleton -m "M0: Workspace skeleton complete — 9 crates, boundary enforcement, policies locked"
git push origin main m0-skeleton
```

---

## Boundary Enforcement Matrix

All boundary tests are in `tests/boundary.rs` for each crate. This matrix summarizes what is enforced:

| Layer | Test | Enforces | Coverage |
|---|---|---|---|
| **Layer 1** | `no_*_in_cargo_toml` | Forbidden deps | 100% of manifest |
| **Layer 2** | `no_*_in_src` | Forbidden imports | 100% of source files |
| **Layer 3** (M1+) | Canonical snapshots | Wire format stability | Type-by-type |

**Known limitations (tracked for M1+ upgrade):**
- Grep cannot detect transitive deps (solved by `cargo metadata` in M1)
- Grep cannot detect feature-enabled deps (solved by `cargo metadata` in M1)
- Grep cannot detect path indirection via re-exports (solved by `cargo public-api` in M1)

---

## Summary: What M0 Delivers

### Types (apw-protocol)

- ✅ Identity: `AgentId`, `LeaseId`, `ActorId`
- ✅ Time: `Tick`, `ClockSource`, `SimulationClock` traits
- ✅ Role: enum with 9 variants
- ✅ Event envelope + 7 event variants
- ✅ Capability enum (6 variants)
- ✅ Authority map: `BTreeMap<ActorId, BTreeSet<Capability>>`

### Crates (8 total + 1 CLI)

- ✅ **Shared:** `apw-protocol` (types only)
- ✅ **Server:** `apw-kernel`, `apw-engine`, `apw-store`, `apw-server`
- ✅ **Client:** `apw-office`, `apw-manager`, `apw-gateway`, `apw-pixel-plugin`
- ✅ **Tools:** `apw-cli` (stub binary)

### Policies (Locked)

- ✅ **Time:** Tick-only in kernel, no `SystemTime`
- ✅ **Determinism:** Canonical types, `BTreeMap`-only iteration
- ✅ **Capability:** Typed, not strings
- ✅ **Async:** Only `apw-server` has `tokio::main`
- ✅ **MSRV:** Pinned to 1.82 in `rust-toolchain.toml`
- ✅ **Boundary:** Mechanical enforcement (Layer 1 + Layer 2)
- ✅ **Event Versioning:** `schema_version` in envelope

### Tests (45+ sections)

- ✅ 1 smoke test per crate (~9)
- ✅ 2-3 boundary tests per crate (~24)
- ✅ Workspace-level smoke tests (~5)
- ✅ CI: Linux + macOS matrix

### Documentation

- ✅ Root README with overview + links
- ✅ Per-crate README with M0 status
- ✅ ADR template + ROADMAP + EVENTS ref
- ✅ CI/CD configured (GitHub Actions)

---

## Next: Milestone M1

Once M0 is approved and merged:

1. **M1 opens:** Port kernel core to `apw-kernel` using the types and event model defined in M0
2. **M1 builds:** Same JSON API surface as Node kernel, but in Rust
3. **M1 verifies:** M0 boundary tests still pass; M1 adds Layer 3 (canonical snapshot tests)
4. **M1 milestone:** https://github.com/forgefabrik/apw-rs/milestones/m1

---

## Checklist: Ready to Implement?

- [ ] Repo created on GitHub (`forgefabrik/apw-rs`)
- [ ] Clone locally, apply this plan task-by-task
- [ ] Each task has a git commit with clear message
- [ ] All tests green locally before pushing
- [ ] CI passes on GitHub (Linux + macOS)
- [ ] M0 tag pushed: `git tag -a m0-skeleton`
- [ ] Team reviews plan, ADRs, then M1 spec begins

---

**Plan ready. Execute Phase 0 → Phase 7, commit at each task boundary, and verify CI passes.**
