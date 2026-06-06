# apw-rs Roadmap

Tracks the milestones after the workspace skeleton (M0) lands. Each milestone is its own brainstorm → spec → plan → build cycle.

> **Authoritative source** for crate boundaries, governance policies, and the `apw-protocol` wire schema is the [design spec](specs/2026-06-05-apw-rs-workspace-skeleton-design.md). This roadmap summarizes the milestone progression; refer to the spec when in doubt.
>
> **Implementation** is in the [M0 implementation plan](plans/2026-06-06-m0-workspace-skeleton-implementation.md), which includes the verification gates that must pass before M0 is declared complete.

## Milestones

| ID | Name | Status | Goal |
|---|---|---|---|
| **M0** | Workspace skeleton | Spec approved, awaiting plan | Pure-Rust Cargo workspace, 8 crates + 1 CLI, no business logic. Enforces server/client boundary. |
| **M1** | Kernel core port | Not started | Port `kernel/event-core`, `algebra`, `freezer`, `trust-report`, `replay`, `snapshot` to `apw-kernel`. Same JSON API surface so the existing Node webui keeps working. |
| **M2** | Engine + server | Not started | Port `engine/agents`, `mailbox`, `economy`, `scheduler`, `sandbox-executor`, `llm-router` to `apw-engine`. `apw-server` (axum) exposes the same `/api/*` endpoints. |
| **M3** | LLM admin | Not started | `apw-engine/llm` trait `TowerAdmin`. Default impl is rule-based; self-hosted LLM (llama.cpp / Ollama / vLLM HTTP) plugs in as one impl. |
| **M4** | Office TUI + pixel pipeline | Not started | `apw-pixel-plugin` full impl (Aseprite JSON + PNG decode). `apw-office` ratatui TUI embeds `pixtuoid-core` + bidding loop. Agent-authored sprite proposal endpoint, trust-gated. |
| **M5** | Manager TUI + webui proxy | Not started | `apw-manager` ratatui TUI (file tree, sessions, terminals). `apw-webui` static server + reverse proxy. Gradual retirement of the Node webui. |

## Cross-Cutting Concerns

These cut across milestones and are tracked here so they don't get lost.

### Boundary check upgrade (replace grep with `cargo metadata`)

**Why:** The M0 boundary tests grep `Cargo.toml` for forbidden crate names. This is the smallest thing that works, but it has four known blind spots:

- **Transitive deps** — a forbidden crate sneaks in via a permitted one. Grep on `Cargo.toml` cannot see this.
- **Workspace dependency aliases** — a `workspace = true` dep can resolve to a forbidden crate. Grep cannot see the resolution.
- **Feature-induced deps** — an `optional = true` dep resolved by a feature flag. Grep on `[dependencies]` won't see it unless features are inspected.
- **Path indirection** — a permitted crate re-exports a forbidden crate's types at the API surface. Grep on `Cargo.toml` cannot see this at all.

**Planned solution:** a single binary `tools/check-boundaries/` (or a test in `apw-protocol` that runs at workspace root) that:

1. Runs `cargo metadata --format-version 1` from the workspace root
2. Parses the resolved dependency graph (the `resolve.nodes[*].deps[*]` array)
3. Asserts that, for each crate, no node in its transitive closure has `name` matching a forbidden pattern
4. Optionally also walks the public API surface of each crate (via `cargo public-api` or `cargo rustdoc --emit=invocation-specific`) to detect path indirection

When this lands, the per-crate `tests/boundary.rs` files are deleted and the single root-level check takes over. The forbidden-dep list is now defined in one place (`tools/check-boundaries/src/forbidden.toml`), not duplicated across 8 files.

**Trigger to start this work:** any of the following becomes true:
- A forbidden dep almost slips in during an M1+ review
- A new crate is added that makes the per-crate grep tests hard to maintain
- Someone notices a transitive dep that should be forbidden

**Estimated scope:** ~200 lines of Rust. 1 day of focused work for a contributor familiar with `cargo metadata`'s JSON shape.

### Aseprite asset pipeline

**Why it matters:** The visual quality of the office is bottlenecked on actual Aseprite art. The pipeline is:

1. Artist opens Aseprite, draws the character/room/item sprite
2. Export via `pixel-plugin` (`/pixel-export json sprite.json format=aseprite`) which produces:
   - `sprite.json` — Aseprite JSON with `meta`, `frames`, `layers`, `slices`, `frameTags`
   - `sprite.png` — companion spritesheet
3. Drop into `crates/apw-pixel-plugin/assets/packs/<pack>/`
4. Add an entry to `crates/apw-pixel-plugin/assets/manifest.toml`
5. `apw-pixel-plugin` loads it at runtime; the office TUI and webui consume it

**Trigger to formalize this:** the M4 spec needs a worked example with a real sprite.

### `pixtuoid-core` embedding decision

**Why it matters:** `apw-office` plans to embed `pixtuoid-core` (the headless Rust library from the `pixtuoid` project). Two options:

- (a) Upstream publishes `pixtuoid-core` to crates.io. We add a `Cargo.toml` dep.
- (b) Upstream does not publish. We vendor it as a git submodule at a pinned commit.

**Trigger to decide:** M4 spec. If upstream maintainer is reachable and willing, prefer (a). If not, (b) is fine and not a blocker.

### Self-hosted LLM endpoint

**Why it matters:** M3's `TowerAdmin` trait needs a real LLM. The tentative design is a thin HTTP client to a self-hosted endpoint (no SDK lock-in). Concrete options to evaluate in M3:

- `llama.cpp` HTTP server (`llama-server`)
- Ollama (`ollama serve`)
- vLLM (`vllm serve`)

**Trigger to decide:** M3 spec.
