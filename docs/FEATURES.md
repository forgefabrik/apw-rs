# FEATURES

> Single source of truth for **what** is being ported from each upstream project, **from where** (file, endpoint, module), **to** which `apw-*` crate, in **which milestone**, and at **what status**.
>
> This document is the answer to "where does feature X live, and where is it going?" The README's *Source projects being ported* table is the high-level crate mapping; this file is the per-feature inventory. The [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md) and the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md) are the architectural and milestone references.

## Status legend

- `done` — feature exists in `apw-rs`
- `porting (Mx)` — feature is being ported in milestone `Mx`
- `planned (Mx)` — feature is on the Mx roadmap but not yet started
- `deferred` — feature is recognized but explicitly out of scope for now
- `not-adopted` — feature exists upstream but `apw-rs` has decided not to take it

When the porting status changes, this file is updated as part of the same commit that lands the change. No stale entries.

---

## 1. Source projects (upstream references)

| # | Source | URL | Role in this port |
|---|---|---|---|
| 1 | forgefabrik / agent-bigbrother | <https://github.com/forgefabrik/agent-bigbrother> | ForgeFabrik Agent OS v0.2a — the concrete kernel/engine/webui implementation. Provides the bulk of the feature surface. |
| 2 | forgefabrik / forgefabrik-agent-os | <https://github.com/forgefabrik/forgefabrik-agent-os> | Architectural spec for the same system (L0–L5, docker stack, TAP layer, control plane UI). Drives the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md). |
| 3 | chriswritescode-dev / opencode-manager | <https://github.com/chriswritescode-dev/opencode-manager> | Mobile-first PWA control plane for OpenCode agents. Drives the manager/web feature surface (M2, M5). |
| 4 | IvanWng97 / pixtuoid | <https://github.com/IvanWng97/pixtuoid> | Terminal pixel-art office for AI agents. The only Rust source project. Drives the `apw-office` TUI (M4) and the `apw-hook` shim pattern. |

Every feature below cites its upstream file/endpoint in the *From* column. The full source-tree listings of these repos at the time of import are in the local cache; the URLs above are the canonical references.

---

## 2. Summary by source

| Source | Features catalogued here | adopted | porting | planned | deferred / not-adopted |
|---|---:|---:|---:|---:|---:|
| agent-bigbrother | 31 | 0 | 8 (M1–M2) | 19 (M1–M4) | 4 |
| forgefabrik-agent-os | 8 | 0 | 2 (M0) | 6 (M0–M4) | 0 |
| opencode-manager | 17 | 0 | 1 (M2) | 14 (M2, M5) | 2 |
| pixtuoid | 14 | 0 | 3 (M4) | 11 (M4–M5) | 0 |
| **Total** | **70** | **0** | **14** | **50** | **6** |

(Catalogued here = every feature explicitly called out by the upstream README. Smaller, downstream conveniences are absorbed into the parent feature and not counted separately.)

---

## 3. Features from `forgefabrik/agent-bigbrother` (v0.2a)

The concrete implementation. Most surface area. 56+ REST endpoints, 37+ smoke-test sections, 4-floor × 3-room office projection.

### 3.1 Kernel (event-sourced, replay-authoritative)

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status |
|---|---|---|---|---|---|
| 3.1.1 | Hash-chained event log (`event-core.mjs`) | `kernel/event-core.mjs` | `apw-kernel::event_core` | M1 | planned (M1) |
| 3.1.2 | Canonical JSON + SHA-256 (`canonicalizer.mjs`) | `kernel/canonicalizer.mjs` | `apw-kernel::canonical` (or `apw-canonical` if split per spec §7) | M1 | planned (M1) |
| 3.1.3 | 6-component execution trace (`tracer.mjs`) | `kernel/tracer.mjs` | `apw-kernel::tracer` | M1 | planned (M1) |
| 3.1.4 | Contract freezer, 8 + pure kinds (`freezer.mjs`) | `kernel/freezer.mjs` | `apw-kernel::freezer` | M1 | planned (M1) |
| 3.1.5 | Decision-graph algebra (`algebra.mjs`) | `kernel/algebra.mjs` | `apw-kernel::algebra` | M1 | planned (M1) |
| 3.1.6 | LM Studio HTTP bridge (`llmstudio.mjs`) | `kernel/llmstudio.mjs` | `apw-engine::llm::LmStudioBridge` | M2 | planned (M2) |
| 3.1.7 | Deterministic replay engine (`replay.mjs`) | `kernel/replay.mjs` | `apw-kernel::replay` | M1 | planned (M1) |
| 3.1.8 | Point-in-time world snapshots (`snapshot.mjs`) | `kernel/snapshot.mjs` | `apw-kernel::snapshot` | M1 | planned (M1) |
| 3.1.9 | Trust report + integrity verifier (`trust-report.mjs`) | `kernel/trust-report.mjs` | `apw-kernel::trust` | M1 | planned (M1) |
| 3.1.10 | Skills-lock loader (`skills-loader.mjs`) | `kernel/skills-loader.mjs` | `apw-kernel::skills_lock` | M2 | planned (M2) |
| 3.1.11 | Skill dispatcher + plugin transport rule (`skill-dispatcher.mjs`) | `kernel/skill-dispatcher.mjs` | `apw-kernel::skill_dispatcher` | M2 | planned (M2) |
| 3.1.12 | Sandbox executor (shell / docker / git_commit / code_edit) | `kernel/sandbox-executor.mjs` | `apw-engine::sandbox` | M2 | planned (M2) |
| 3.1.13 | Storage event middleware (`storage-events.mjs`) | `kernel/storage-events.mjs` | `apw-kernel::storage_events` | M1 | planned (M1) |
| 3.1.14 | HTTP server hosting webui + REST (`server.mjs`) | `kernel/server.mjs` | `apw-server` (axum) | M2 | planned (M2) |
| 3.1.15 | Runtime snapshot aggregator (`runtime_snapshot.mjs`) | `kernel/runtime_snapshot.mjs` | `apw-server::runtime_snapshot` | M2 | planned (M2) |
| 3.1.16 | Pixel-agent projection (avatar, badges, workstation) | `kernel/agent_projection.mjs` | `apw-pixel-plugin` + `apw-server::projections::agents` | M4 | planned (M4) |
| 3.1.17 | Forge tower 4-floor × 3-room projection | `kernel/forge_tower.mjs` | `apw-pixel-plugin` + `apw-server::projections::tower` | M4 | planned (M4) |
| 3.1.18 | Speech-bubble conversation threads | `kernel/conversations` (in `runtime_snapshot.mjs`) | `apw-server::projections::conversations` | M4 | planned (M4) |
| 3.1.19 | Sprite loader + manifest resolver (read-only) | `kernel/sprite_loader.mjs` | `apw-pixel-plugin::SpriteLoader` + `apw-pixel-plugin::SpriteResolver` | M4 | planned (M4) |

### 3.2 Engine (extraction, compilation, runtime)

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status |
|---|---|---|---|---|---|
| 3.2.1 | Unified LLM client (live + mock) | `engine/llm-router.mjs` | `apw-engine::llm::Router` (default: rule-based; live impl in M3) | M2 / M3 | planned (M2) |
| 3.2.2 | Prompt registry | `engine/prompts.mjs` | `apw-engine::llm::Prompts` | M2 | planned (M2) |
| 3.2.3 | World config + system limits loader | `engine/config-loader.mjs` | `apw-server::config` | M1 | porting (M1) |
| 3.2.4 | 4 fact extractors (req / con / cap / risk) | `engine/extractors.mjs` | `apw-engine::extractors` | M1 | planned (M1) |
| 3.2.5 | 3 deterministic compilers (arch / plan / tasks) | `engine/compilers.mjs` | `apw-engine::compilers` | M1 | planned (M1) |
| 3.2.6 | Trace drift, variance, confidence semantics | `engine/drift.mjs` | `apw-kernel::drift` | M1 | planned (M1) |
| 3.2.7 | TAP planning layer (observe / suggest) | `engine/tap.mjs` | `apw-tap` crate | M2 | planned (M2) |
| 3.2.8 | Agent runtime (registry, leases, heartbeats, reputation) | `engine/agents.mjs` | `apw-runtime::agents` | M2 | planned (M2) |
| 3.2.9 | Append-only agent mailbox | `engine/mailbox.mjs` | `apw-runtime::mailbox` | M2 | planned (M2) |
| 3.2.10 | Bid projection, market pressure, pricing | `engine/economy.mjs` | `apw-economy` crate | M2 | planned (M2) |
| 3.2.11 | World simulator + scenario loader | `engine/simulation.mjs` | `apw-sim` crate | M2 | planned (M2) |
| 3.2.12 | Plugin compiler (GitHub sources → plugin registry) | `engine/plugin_compiler.mjs` | `apw-plugin::Compiler` | M2 | planned (M2) |
| 3.2.13 | Plugin runtime index | `engine/plugin_registry.mjs` | `apw-plugin::Registry` | M2 | planned (M2) |
| 3.2.14 | PID layer for sandboxed executions | `engine/execution_tracker.mjs` | `apw-engine::execution_tracker` | M2 | planned (M2) |
| 3.2.15 | Drizzle-compatible JSONL store | `engine/store.mjs` | `apw-store` (memory / fs / sqlite adapters) | M1 | porting (M1) |
| 3.2.16 | Evaluation engine (start/finish evaluations) | `engine/evaluation.mjs` | `apw-engine::evaluation` | M3 | planned (M3) |
| 3.2.17 | Benchmark engine | `engine/benchmark.mjs` | `apw-engine::benchmark` | M3 | planned (M3) |
| 3.2.18 | Prompt version manager | `engine/prompt_versions.mjs` | `apw-engine::llm::PromptVersionManager` | M3 | planned (M3) |

### 3.3 WebUI (10-tab control grid)

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status |
|---|---|---|---|---|---|
| 3.3.1 | Sticky header + 10 tab panes + footer | `webui/index.html` | `apw-gateway::webui` (Dioxus fullstack) | M5 | planned (M5) |
| 3.3.2 | Hash router, 10 per-tab renderers, 2-second polling | `webui/app.js` | `apw-gateway::webui::router` | M5 | planned (M5) |
| 3.3.3 | DM Serif Display + IBM Plex Mono + Inter Tight stylesheet | `webui/app.css` | `apw-gateway::webui::styles` | M5 | planned (M5) |
| 3.3.4 | Isometric office HQ (10 areas, agent lifecycle, heatmap) | `webui/app.js` (`renderIsometricOffice`, was `renderWorld`) | `apw-gateway::webui::office_iso` (also mirrored by `apw-office` TUI) | M4 / M5 | planned (M4) |
| 3.3.5 | 56+ REST API endpoints (`/api/*`) | `kernel/server.mjs` | `apw-server` routes | M2 | porting (M2) |
| 3.3.6 | 37-section smoke test | `tests/smoke.mjs` | `apw-server::tests::smoke` | M1 | porting (M1) |

### 3.4 Not adopted

| # | Feature | Why |
|---|---|---|
| 3.4.1 | Vanilla HTML/CSS/JS webui (no build step) | Superseded by Dioxus fullstack in `apw-gateway`. The runtime snapshot shape and endpoints are preserved; the renderer is replaced. |
| 3.4.2 | 8 new sprite roles / 3 new expressions added in the HQ enhancement (reviewing, planning, expired + economist, replay_agent, trust_agent) | Folded into §3.1.16 (pixel-agent projection). The expression set is extended in M4 when the projection lands. |
| 3.4.3 | Drizzle ORM (Node) | Replaced by `apw-store` (memory / fs / sqlite adapters behind one trait). |
| 3.4.4 | WebUI 2-second polling | Replaced by SSE streaming in `apw-server` for live data; polling kept as a fallback. |

---

## 4. Features from `forgefabrik/forgefabrik-agent-os` (architecture spec)

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status |
|---|---|---|---|---|---|
| 4.1 | L0–L5 architectural rules | `README.md` (architecture section) | [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md) | M0 | porting (M0) |
| 4.2 | Cargo workspace with 8 crates + 1 binary | `README.md` (repo skeleton section) | `crates/` + `tools/` | M0 | porting (M0) |
| 4.3 | Control-plane UI (TAP dashboard menus) | `README.md` (control plane section) | `apw-gateway::webui` (Dioxus) | M5 | planned (M5) |
| 4.4 | Dockerized control plane (docker-compose stack) | `Dockerfile`, `docker-compose.yml` | `Dockerfile` + `docker-compose.yml` at root | M2 | planned (M2) |
| 4.5 | TAP (LLM cognitive layer, read-only) | `tap/` (concept spec) | `apw-tap` crate | M2 | planned (M2) |
| 4.6 | Sim mode (parallel world / sandbox) | `sim/` (concept spec) | `apw-sim` crate | M2 | planned (M2) |
| 4.7 | Plugin marketplace (29 GitHub repos → 4 categories) | `data/plugin_sources.json` | `apw-plugin::Compiler` (initially seeded from same source list) | M2 | planned (M2) |
| 4.8 | Local LLM runtime (LM Studio / llama.cpp / Ollama / vLLM) | `lmstudio/` (concept spec) | `apw-engine::llm` trait with pluggable backends | M3 | planned (M3) |

---

## 5. Features from `chriswritescode-dev/opencode-manager` (mobile-first PWA)

Mature TypeScript monorepo (566★, 71 forks). The features below are the ones `apw-rs` adopts; the rest (frontend framework, bun tooling, specific UI library choices) are replaced by Dioxus / axum.

### 5.1 Adopted

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status |
|---|---|---|---|---|---|
| 5.1.1 | Multi-repo management (per-repo config, worktrees, SSH auth) | `backend/src/repos/*` | `apw-server::repos` + `apw-gateway::repos` | M5 | planned (M5) |
| 5.1.2 | Unified diffs, branch + commit management | `backend/src/git/*` | `apw-server::git` | M5 | planned (M5) |
| 5.1.3 | File tree view, syntax highlighting, ZIP download | `frontend/src/files/*` | `apw-gateway::files` | M5 | planned (M5) |
| 5.1.4 | Real-time SSE chat streaming | `backend/src/sessions/sse` | `apw-server::sessions::sse` | M2 | porting (M2) |
| 5.1.5 | Slash commands, `@file` mentions, Plan/Build modes | `frontend/src/chat/*` | `apw-gateway::chat` | M2 | planned (M2) |
| 5.1.6 | Mermaid diagram rendering | `frontend/src/components/Mermaid.tsx` | `apw-gateway::components::mermaid` | M2 | planned (M2) |
| 5.1.7 | Schedules (recurring repo jobs, run history, linked sessions) | `backend/src/schedules/*` | `apw-server::schedules` | M5 | planned (M5) |
| 5.1.8 | MCP server management (add, configure, OAuth) | `backend/src/mcp/*` | `apw-server::mcp` | M5 | planned (M5) |
| 5.1.9 | AI configuration (models, providers, API keys, OAuth) | `backend/src/ai/*` | `apw-server::ai_config` | M3 | planned (M3) |
| 5.1.10 | Skills (shareable, scoped definitions) | `backend/src/skills/*` | `apw-engine::skills` | M2 | planned (M2) |
| 5.1.11 | Push notifications (VAPID) for session events / questions / errors | `backend/src/push/*` | `apw-server::push` | M5 | planned (M5) |
| 5.1.12 | Audio TTS/STT (browser native + OpenAI-compatible) | `frontend/src/audio/*` | `apw-gateway::audio` | M5 | planned (M5) |
| 5.1.13 | PWA (service worker, installable) | `frontend/public/sw.js` + manifest | `apw-gateway::pwa` | M5 | planned (M5) |
| 5.1.14 | Mobile-first responsive navigation | `frontend/src/layouts/*` | `apw-gateway::layouts` | M5 | planned (M5) |
| 5.1.15 | `ocm` CLI (attach local TUI, push/pull tarball sync) | `ocm-cli/` | `apw-cli::ocm` | M2 | planned (M2) |

### 5.2 Not adopted

| # | Feature | Why |
|---|---|---|
| 5.2.1 | React + Vite frontend | Replaced by Dioxus fullstack (single Rust codebase). |
| 5.2.2 | Bun runtime | Replaced by tokio in `apw-server`. |

---

## 6. Features from `IvanWng97/pixtuoid` (Rust terminal pixel-art office)

The only Rust source. Three crates, 467 commits, 184★. Drives the `apw-office` TUI (M4) and the `apw-hook` shim pattern (M0/M4). `pixtuoid-core` will be embedded per the design spec §3.5 (decision: vendor as git submodule at a pinned commit until upstream publishes to crates.io; tracked in [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md)).

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status |
|---|---|---|---|---|---|
| 6.1 | 3-crate workspace (`pixtuoid-core` / `pixtuoid` / `pixtuoid-hook`) | `crates/` | shape mirrored in `apw-rs` workspace | M0 / M4 | porting (M0) |
| 6.2 | `Source` trait (hookable agent CLI adapter) | `crates/core/src/source.rs` | `apw-hook::Source` (or `apw-runtime::source`) | M4 | planned (M4) |
| 6.3 | `Reducer` → `SceneState` (watch channel) | `crates/core/src/reducer.rs` | `apw-pixel-plugin::reducer` | M4 | planned (M4) |
| 6.4 | Pose → pixel painter → half-block → ratatui rendering | `crates/pixtuoid/src/render/*` | `apw-office::render` | M4 | planned (M4) |
| 6.5 | Multi-floor office (page navigation, slide transition) | `crates/pixtuoid/src/ui/office.rs` | `apw-office::office` | M4 | planned (M4) |
| 6.6 | Per-tool monitor glow (Edit = blue, Bash = orange, Read = cyan) | `crates/pixtuoid/src/ui/glow.rs` | `apw-office::glow` | M4 | planned (M4) |
| 6.7 | Per-agent identity (deterministic palette from session hash, 16 outfits) | `crates/core/src/palette.rs` | `apw-pixel-plugin::palette` | M4 | planned (M4) |
| 6.8 | Weather effects (rain, storm, snow, fog, sunset) | `crates/pixtuoid/src/ui/weather.rs` | `apw-office::weather` | M4 | planned (M4) |
| 6.9 | Tooltip stats (session duration, tool calls, active %) | `crates/pixtuoid/src/ui/tooltip.rs` | `apw-office::tooltip` | M4 | planned (M4) |
| 6.10 | Office pets (cat / dog, sleep near idle agents) | `crates/pixtuoid/src/ui/pets.rs` | `apw-office::pets` | M4 | planned (M4) |
| 6.11 | Hook shim (200ms timeout, always exit 0) | `crates/pixtuoid-hook/src/main.rs` | `apw-hook` (binary) | M0 | porting (M0) — see design spec §3.3 |
| 6.12 | Unix-socket transport `/tmp/pixtuoid-{uid}.sock` | `crates/core/src/transport/unix.rs` | `apw-hook::transport::unix` | M4 | planned (M4) |
| 6.13 | 6 built-in themes (Normal, Cyberpunk, Dracula, Tokyo Night, Catppuccin, Gruvbox) | `crates/pixtuoid/src/theme/*` | `apw-office::theme` | M4 | planned (M4) |
| 6.14 | Source adapters (Claude Code, Codex, Antigravity) | `crates/core/src/sources/*` | `apw-hook::sources` | M4 | planned (M4) |

---

## 7. Scope split: Phase 1 (Visual Office Foundation) vs Phase 2 (Economy & Multi-Role Roadmap)

`apw-rs` is built in two clearly separated phases. The **visual office is the foundation**; everything in Phase 2 lands on top of it.

| | **Phase 1 — Visual Office Foundation** (in scope, current build) | **Phase 2 — Economy & Multi-Role** (roadmap, after Phase 1) |
|---|---|---|
| **What it is** | The pixel-art office surface — agents as characters, projects as rooms, per-project towers, living-room personalization, shop UIs | The economic engine + multi-role orchestration that drives the office: tower-climb mechanics, money, CEO shopping, LM-Studio-driven LLM agents, customer→CEO→worker project flow |
| **When it ships** | After the M0 (workspace skeleton) + M4 (office TUI + pixel pipeline) milestones | Roadmap. Not part of any current M-numbered milestone. The Phase 2 doc is the **last** deliverable, written after the office is finished. |
| **Crate surface used** | `apw-protocol` (wire types), `apw-pixel-plugin` (sprite/character data), `apw-office` (TUI renderer), `apw-gateway` (web control plane) | Adds `apw-kernel` (state machine), `apw-engine` (LLM, agents, scheduler), `apw-store` (ledger + persistence), `apw-server` (HTTP + SSE) on top of Phase 1 |
| **Where the data comes from** | A simple local JSON / sample data adapter that exercises the office surfaces | The kernel event log + LM Studio LLM with TypeScript-plugin tool surface |
| **In this file** | §3–§6 (sources), §7 (cross-cutting) | **§9 (this document)** |

The protocol types in `apw-protocol/src/lib.rs` (Role, AuthorityMap, Capability, Event variants like `AgentPromoted`, `ItemPurchased`, `CapabilityDenied`) are **forward-compatible placeholders**: they exist so Phase 1 can render the *state* of Phase 2 systems without Phase 2 existing. The implementations come in Phase 2.

---

## 8. Phase 2 — Economy & Multi-Role (Roadmap)

> **Status: deferred (Phase 2).** Not part of the current build. This section is the architectural seed: it documents the bigger vision and the protocol-level primitives that the office already anticipates, so that the Phase 1 surface has stable shape and the Phase 2 build can land without breaking it.
>
> The full Phase 2 design — including the LM Studio plugin bridge, the customer→CEO→worker flow, the tower-climb/economy mechanics, and the office↔kernel event mapping — will be written as a separate design spec **after** the Phase 1 office is finished. That spec is the terminal deliverable mentioned in the [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md).

### 8.1 The bigger vision (one paragraph)

A customer posts a job to a project tower. The CEO — an LLM running on LM Studio via the TypeScript plugin SDK — sees the new job, decomposes it, and either spawns worker agents (each an LLM with its own tools) or assigns existing ones. Workers climb the project's tower as they deliver value (merged PRs, passing tests, milestones); higher floors unlock better office infrastructure. Workers earn money for their work and spend it in their personal living rooms. The CEO earns money from project revenue and spends it on new rooms, faster PCs (more capable models), or office perks. Visual office state is a **compiled projection of the economic state**, never the trigger. Idle state = no revenue = no growth = office decay. The game loop is a closed economic system, not a cosmetic animation.

### 8.2 Architectural primitives (forward-compat types)

These are the Rust types/traits that the office surface will reference in Phase 1 (as enum variants or trait method signatures, often no-op), and that the Phase 2 kernel/engine will implement.

| # | Primitive | Lives in (Phase 2) | Purpose | Phase 1 status |
|---|---|---|---|---|
| 8.2.1 | `Role::Ceo`, `Role::Worker`, `Role::Customer` (already in `apw-protocol::Role` enum) | `apw-protocol` | Distinguishes actors in the multi-role system. CEO is the orchestrator; workers execute; customers post jobs. | **Defined** (Role enum has Ceo + 8 others; Worker/Customer to be added) |
| 8.2.2 | `AuthorityMap = BTreeMap<ActorId, BTreeSet<Capability>>` (already in `apw-protocol`) | `apw-protocol` | Per-actor permission set. CEO has broad capabilities; workers are restricted. | **Defined** (type alias exists; population rules in Phase 2) |
| 8.2.3 | `Capability::PromoteAgent { from_floor, to_floor }`, `Capability::AllocateLease`, `Capability::SubmitSpriteProposal`, `Capability::ModifyAuthorityMap`, `Capability::RunTowerAdmin`, `Capability::ReplayChain` | `apw-protocol` | Typed capability enum (typo-resistant). Capabilities gate the LLM's tool surface. | **Defined** (enum exists; checked at runtime in Phase 2) |
| 8.2.4 | `BiddingEngine` trait | `apw-engine` (Phase 2) | Customer posts a job → workers/CEO submit bids → winner is chosen by rule (price, trust, ETA) → lease is allocated. Trait surface: `post_job`, `submit_bid`, `resolve`, `cancel`. | **Stub** (trait signature only, no impl) |
| 8.2.5 | `TrustVerifier` trait | `apw-kernel` (Phase 2) | Verifies a claimed contribution is genuine. Inputs: contribution event, observed state (CI result, test result, peer review), trust score history. Output: accepted / rejected / partial. | **Stub** (trait signature only, no impl) |
| 8.2.6 | `Wallet { actor: ActorId, balance: u64, ledger: Vec<MoneyEvent> }` | `apw-store` (Phase 2) | Per-actor money balance with append-only event ledger. CEO has a treasury; each worker has a wallet; the project tower has a money pool. | **Not defined** (to be added in Phase 2 spec) |
| 8.2.7 | `Item { id, name, price, slot: LivingRoomSlot, effect }` + `Catalog` | `apw-protocol` + `apw-pixel-plugin` | Buyable personal items for the worker's living room: plants, posters, pets, coffee machines, mini-games, etc. | **Not defined** (Phase 2; pixel-plugin renders the inventory) |
| 8.2.8 | `Upgrade { id, name, price, kind: Room | Pc | Perk, effect }` | `apw-protocol` + `apw-pixel-plugin` | Buyable office upgrades for the CEO: new rooms, faster PCs (model tier up), perks (review bots, automation, etc.). | **Not defined** (Phase 2) |
| 8.2.9 | `Tower { project_id, floors: Vec<Floor>, current_height: u8, total_revenue: u64 }` | `apw-pixel-plugin` | Per-project tower. Each floor is an unlock tier. Height is a function of revenue ÷ floor_cost. | **Not defined** (Phase 2; pixel-plugin will render) |
| 8.2.10 | `ClimbEvent { worker_id, project_id, from_floor, to_floor, revenue_delta, reason, tick }` | `apw-protocol::Event` (new variant) | Emitted by the engine when a worker has produced enough revenue to climb a floor. Drives office animation. | **Not defined** (new event variant in Phase 2) |
| 8.2.11 | `LivingRoom { agent_id, owned_items: Vec<ItemId>, layout_grid: Vec<u8>, decoration_score: u32 }` | `apw-store` + `apw-pixel-plugin` | Per-agent personal space. Rendered as a small room adjacent to the work area. | **Not defined** (Phase 2) |
| 8.2.12 | `MoneyEvent { actor: ActorId, delta: i64, reason: MoneyReason, tick }` | `apw-protocol::Event` (new variant) | Append-only ledger entry. Reasons: ContributionMerged, ProjectCompleted, CiPassed, ItemPurchased, UpgradePurchased, OfficeDecay. | **Not defined** (Phase 2) |
| 8.2.13 | `CeoBridge` (LM Studio plugin adapter) | `apw-engine` (Phase 2) | The CEO is an LLM running on LM Studio via the TypeScript plugin SDK (`@lmstudio/sdk`, `LMStudioClient.act(chat, tools)`). The bridge exposes Rust kernel operations as LM Studio `tool()` definitions, so the CEO can `spawn_agent`, `assign_lease`, `purchase_upgrade`, etc. | **Not defined** (Phase 2; deferred design until Phase 1 office is done) |
| 8.2.14 | `CustomerIntake` (job-posting flow) | `apw-server` + `apw-engine` (Phase 2) | Customer-facing surface: post a job, set a budget, see bids, accept the winner. Triggers a new project tower. | **Not defined** (Phase 2) |
| 8.2.15 | Office decay rule | `apw-engine` (Phase 2) | If `total_revenue == 0` for N consecutive ticks, office visuals degrade (coffee quality drops, equipment becomes outdated, rooms visibly decay). Anti-AFK-wealthy state. | **Not defined** (Phase 2) |

### 8.3 Phase 2 milestones (preliminary, not committed)

The roadmap will be updated when the Phase 2 spec is written. A likely shape, subject to revision:

- **P2.0** — Protocol extension: add `Role::Worker`, `Role::Customer`, `Wallet`, `Item`, `Upgrade`, `Tower`, `ClimbEvent`, `MoneyEvent`, `LivingRoom`, `BiddingEngine`, `TrustVerifier` to `apw-protocol` and `apw-engine` as forward-compat types (compilable, no impl).
- **P2.1** — Engine: implement revenue computation (merged contribution, CI-passed, milestone), `MoneyEvent` emission, `Wallet` ledger.
- **P2.2** — Engine: implement `BiddingEngine` (post-job → bid → resolve → lease).
- **P2.3** — Kernel: implement `TrustVerifier` (verify contribution, update trust score).
- **P2.4** — Engine: implement `ClimbEvent` emission and `Tower` height computation.
- **P2.5** — `apw-engine::CeoBridge` — the LM Studio `@lmstudio/sdk` plugin adapter. The CEO's tool surface is the set of kernel operations gated by `AuthorityMap`.
- **P2.6** — `apw-server` — `CustomerIntake` HTTP surface.
- **P2.7** — `apw-pixel-plugin` — render Tower, LivingRoom, Shop UI from the new state.
- **P2.8** — Office decay loop, end-to-end.

These are **pre-decision notes**, not a commitment. The actual Phase 2 spec is the terminal deliverable.

### 8.4 Why the visual office is built first

The office is the **visible surface** for everything in Phase 2. If we build the office first against simple local data, we get:

1. **Stable visual contracts.** Pixel placements, animation transitions, click targets, layout grids are nailed down while the data layer is simple. Phase 2 plugs into the same visual surface without breaking it.
2. **No coupling to a specific LLM.** The Phase 1 office reads from a static adapter, so the office works without LM Studio, without a kernel, without a server. Easy to ship, easy to test.
3. **The office is the test rig.** Once Phase 2 lands, the office becomes the live debugger — when the economic engine miscalculates revenue, you see the worker on the wrong floor; when trust is wrong, you see the worker denied a climb.
4. **Office-first = user-first.** The thing people will look at and share is the office. Phase 2's correctness is invisible to most users; the office's look is the whole product.

### 8.5 Anti-patterns the design avoids

- **Cosmetic-only XP bars.** The tower is not an animation; it is a derived state of `Wallet` revenue vs `Floor.cost`. Without revenue, no climb.
- **Idle state earns money.** No revenue from agent turn, thinking, or idle animation. Revenue only from merged contribution, CI-passed release, project completion, or customer delivery.
- **AFK-wealthy state.** Office decay rule ensures no-output companies stagnate.
- **LLM-as-authority.** The LLM emits `Intent` types; the kernel turns them into authoritative events. The LLM never writes to the event chain directly. (See [design spec §Determinism Policy](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md).)
- **Stringly-typed roles.** `Role` is an enum, not a freeform string. Capabilities are typed, not strings. Authority is a `BTreeMap`, not a hash.

### 8.6 Kernel = CEO (architectural principle)

**The kernel *is* the CEO.** It is not a low-level state machine that the CEO plugs into from outside. The CEO is a *distributed role* spread across three components, and the kernel is its **source of truth**:

| Layer | Component | What it owns | What it does NOT own |
|---|---|---|---|
| **Authority** (the spine) | `apw-kernel` | Event chain (hash-chained), `AuthorityMap`, trust scores, `Wallet` ledger, `Tower` height computation, replay authority, freeze/snapshot | Does not decide what to do; does not generate intents |
| **Cognition** (the brain) | LM Studio LLM (via `@lmstudio/sdk` `LMStudioClient`) | Reads kernel state, generates `Intent` types (bid, assign-lease, purchase-upgrade, spawn-agent), calls kernel operations through the engine bridge | Does not write to the event chain; does not bypass `AuthorityMap` |
| **Execution** (the hands) | `apw-engine` | Validates intents against `AuthorityMap`, turns validated intents into authoritative events in the kernel, runs the LLM plugin bridge, manages agents and leases | Does not generate intents; does not own authoritative state |

**Consequences for the design:**

1. **The CEO is not an LLM.** The CEO is the trio (kernel + LLM + engine) working together. A failure of any one breaks the CEO; a clean handoff between them is the design problem. The visual office shows the *joint output* of the trio.
2. **The kernel is replay-authoritative, always.** Even if the LLM hallucinates, the kernel state is recoverable from the event chain. Trust is computed deterministically from the chain. The CEO's authority is *its history*, not its current LLM temperature.
3. **The LLM's tool surface IS the AuthorityMap.** The CEO can only call kernel operations it has capabilities for. `Capability::PromoteAgent`, `Capability::AllocateLease`, `Capability::RunTowerAdmin` etc. are the typed tools the LM Studio plugin exposes. The plugin IS the CEO's bridge to its own authority.
4. **The engine is a *reducer*.** It takes `Intent` → validates → emits `Event`. The LLM never writes events directly. (See design spec §Determinism Policy #3.)
5. **The tower is a kernel projection.** Tower height is derived from `Wallet` + `Floor.cost` + `ClimbEvent` history. The LLM can suggest a climb; only the kernel can authoritatively record one. The visual office reads the kernel's record.

**Why this matters for Phase 1:**

- The Phase 1 visual office reads from a *static adapter* (no real kernel, no real LLM). The office sees a fake `AuthorityMap`, fake `Wallet` ledgers, fake `Tower` state — all seeded from sample data.
- The visual contracts (sprite for "CEO", sprite for "climbing worker", sprite for "rich living room", sprite for "empty office decay") are designed against this fake state.
- When Phase 2 lands, the same visual contracts read the *real* kernel state. The office's UI doesn't change. The data source does.
- The kernel-first principle means: when Phase 2 is designed, the kernel gets the `Wallet`, `Tower`, `ClimbEvent`, `MoneyEvent` types first; the engine gets the LM Studio bridge second; the office's pixel data adapter switches from fake to real last.

**Naming note:** `Role::Ceo` in the protocol is the *role tag* the kernel applies to the (kernel+LLM+engine) trio when it acts. The trio has many internal events (LLM emits `Intent`, engine emits `Event`); the role tag tells the rest of the system "this event was produced by the CEO's joint action". The kernel is the part of the trio that *can* be tagged `Ceo` for the purposes of `AuthorityMap` lookups, because the kernel is the authoritative spine.

---

## 9. Cross-cutting (introduced by `apw-rs` itself)

These are not from any single upstream; they are new in `apw-rs` to satisfy governance policies locked in the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md).

| # | Feature | Driver | Crate | Milestone | Status |
|---|---|---|---|---|---|
| 7.1 | MSRV pin (`rust-toolchain.toml` = `1.82`) | Time/Determinism/Async policies | workspace | M0 | porting (M0) |
| 7.2 | `#![forbid(unsafe_code)]` in every crate | Capability/Panic policies | all crates | M0 | porting (M0) |
| 7.3 | Layer 1/2 boundary tests (`tests/boundary.rs` per crate) | Workspace Philosophy §1–5 | all crates | M0 | porting (M0) |
| 7.4 | `BTreeMap` / `BTreeSet` only in replay-authoritative state | Deterministic Iteration Policy | `apw-kernel`, `apw-protocol` | M0 | porting (M0) |
| 7.5 | `ClockSource` / `SimulationClock` / `RandomSource` traits | Time + Randomness policies | `apw-protocol` | M0 | porting (M0) |
| 7.6 | `Capability` enum + `AuthorityMap` type alias | Capability Policy | `apw-protocol`, `apw-kernel` | M0 | porting (M0) |
| 7.7 | `EventEnvelope::schema_version: u32` on every versioned type | Event Versioning Policy | `apw-protocol` | M0 | porting (M0) |
| 7.8 | ADR process under `docs/adr/` | ADR Policy (design spec §3.2) | repo | M0 | porting (M0) |
| 7.9 | `cargo-deny` license + advisory gates | opencode-manager inspired, stricter | `.cargo/deny.toml` + CI | M1 | planned (M1) |
| 7.10 | `cargo metadata` graph validator (replaces grep boundary tests) | M0 grep-known limitations | `tools/check-boundaries/` | M1 | planned (M1) |

---

## 10. How to use this file

- **Adding a feature?** Find the source row, add a row to the corresponding §3–§6 with `planned (Mx)`, commit, push. If a feature has no upstream, add it to §7.
- **Porting a feature?** Change status from `planned (Mx)` to `porting (Mx)` when the work starts, and to `done` when the feature ships. The commit that lands the change updates this file in the same diff.
- **Dropping a feature?** Move the row to the *Not adopted* subsection of its source, with a one-line reason. Never silently delete.
- **Renaming a target crate?** Grep this file for the old crate name and update in the same commit as the rename. Every row in §3–§6 should reference a real crate once M0 lands.
- **Reviewer?** If a PR adds a new public surface in `apw-*` crates, it should also have added a row to this file. Reject the PR if not.

The `apw-rs` repo at any commit should be able to answer "where does this feature come from, where is it going, and what's its status?" from this file alone.
