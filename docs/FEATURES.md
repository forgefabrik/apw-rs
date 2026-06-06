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

## 7. Cross-cutting (introduced by `apw-rs` itself)

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

## 8. How to use this file

- **Adding a feature?** Find the source row, add a row to the corresponding §3–§6 with `planned (Mx)`, commit, push. If a feature has no upstream, add it to §7.
- **Porting a feature?** Change status from `planned (Mx)` to `porting (Mx)` when the work starts, and to `done` when the feature ships. The commit that lands the change updates this file in the same diff.
- **Dropping a feature?** Move the row to the *Not adopted* subsection of its source, with a one-line reason. Never silently delete.
- **Renaming a target crate?** Grep this file for the old crate name and update in the same commit as the rename. Every row in §3–§6 should reference a real crate once M0 lands.
- **Reviewer?** If a PR adds a new public surface in `apw-*` crates, it should also have added a row to this file. Reject the PR if not.

The `apw-rs` repo at any commit should be able to answer "where does this feature come from, where is it going, and what's its status?" from this file alone.
