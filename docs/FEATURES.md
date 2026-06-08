# FEATURES

> **This file is generated.** Do not edit it directly. Edit `features.registry.json` and run `cargo run -p feature-md -- render` to regenerate.
>
> Single source of truth for **what** is being ported from each upstream project, **from where** (file, endpoint, module), **to** which `apw-*` crate, in **which milestone**, and at **what status**. The [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md) and the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md) are the architectural and milestone references. The four meta-tools (feature-guardian, feature-md, feature-harvester, feature-graph) read this file.

## Status legend

- `done` — feature exists in `apw-rs`
- `porting (Mx)` — feature is being ported in milestone `Mx`
- `planned (Mx)` — feature is on the Mx roadmap but not yet started
- `deferred` — feature is recognized but explicitly out of scope for now
- `not-adopted` — feature exists upstream but `apw-rs` has decided not to take it

When the porting status changes, `features.registry.json` is updated as part of the same commit that lands the change. No stale entries.

## 1. Source projects (upstream references)

| # | id | repo | license | role |
|---|---|---|---|---|
| 1 | `agent-bigbrother` | [forgefabrik/agent-bigbrother](https://github.com/forgefabrik/agent-bigbrother) | MIT | ForgeFabrik Agent OS v0.2a — the concrete kernel/engine/webui implementation. Provides the bulk of the feature surface. |
| 2 | `agent-os` | [forgefabrik/forgefabrik-agent-os](https://github.com/forgefabrik/forgefabrik-agent-os) | MIT | Architectural spec for the same system (L0–L5, docker stack, TAP layer, control plane UI). Drives the design spec. |
| 3 | `opencode-manager` | [chriswritescode-dev/opencode-manager](https://github.com/chriswritescode-dev/opencode-manager) | MIT | Mobile-first PWA control plane for OpenCode agents. Drives the manager/web feature surface (M2, M5). |
| 4 | `pixtuoid` | [IvanWng97/pixtuoid](https://github.com/IvanWng97/pixtuoid) | MIT | Rust terminal pixel-art office for AI agents. Drives apw-office TUI (M4) and the apw-hook shim pattern. The only pure-Rust source project. |
| 5 | `agentroom` | [liuyixin-louis/agentroom](https://github.com/liuyixin-louis/agentroom) | MIT | Visual pixel-art layer for AgentRoom — Canvas 2D + Tauri v2 + CASS search backend. Reference for the visual surface; the React/Canvas code is a UX inspiration, not a port target (we use Dioxus fullstack in apw-gateway). |
| 6 | `apw-rs` | [forgefabrik/apw-rs](https://github.com/forgefabrik/apw-rs) | MIT | This repository. Cross-cutting features (policies, boundaries, governance) are catalogued with source_id=apw-rs. |
| 7 | `lmstudio` | [lmstudio-ai/lms](https://lmstudio.ai/docs/typescript/plugins) | Apache-2.0 | LM Studio TypeScript plugin SDK (@lmstudio/sdk). The CEO's cognitive layer runs on this — LMStudioClient + tool() + model.act() turn an LLM into an orchestrator with a typed tool surface backed by apw-kernel. |

Every feature below cites its upstream file/endpoint in the *From* column.

---

## 2. Summary by source

| Source | Features catalogued | adopted | porting | planned | deferred / not-adopted |
|---|---:|---:|---:|---:|---:|
| `agent-bigbrother` | 47 | 0 | 4 | 39 | 4 |
| `agent-os` | 8 | 0 | 2 | 6 | 0 |
| `apw-rs` | 54 | 0 | 12 | 3 | 39 |
| `lmstudio` | 2 | 0 | 0 | 0 | 2 |
| `opencode-manager` | 17 | 0 | 1 | 14 | 2 |
| [`pixtuoid`](sources/pixtuoid.md) | 14 | 0 | 2 | 12 | 0 |
| **Total** | **142** | **0** | **21** | **74** | **47** |

---

## 3. Features from `agent-bigbrother` (v0.2a)

ForgeFabrik Agent OS v0.2a — the concrete kernel/engine/webui implementation. Provides the bulk of the feature surface.

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status | Tags |
|---|---|---|---|---|---|---|
| `1.0.0` | Hash-chained event log (event-core) | `kernel/event-core.mjs` | `apw-kernel` (apw_kernel::event_core) | M1 | planned | `kernel`, `events`, `replay` |
| `1.0.1` | Canonical JSON + SHA-256 (canonicalizer) | `kernel/canonicalizer.mjs` | `apw-protocol` (apw_protocol::canonical) | M1 | planned | `kernel`, `canonical`, `hashing` |
| `1.0.10` | Pixel-agent projection (avatar, badges, workstation) | `kernel/agent_projection.mjs` | `apw-pixel-plugin` (apw_pixel_plugin::projection::agents) | M4 | planned | `pixel`, `projection`, `office` |
| `1.0.11` | Forge tower 4-floor × 3-room projection | `kernel/forge_tower.mjs` | `apw-pixel-plugin` (apw_pixel_plugin::projection::tower) | M4 | planned | `pixel`, `projection`, `tower`, `office` |
| `1.0.12` | Speech-bubble conversation threads | `kernel/conversations (in runtime_snapshot.mjs)` | `apw-pixel-plugin` (apw_pixel_plugin::projection::conversations) | M4 | planned | `pixel`, `projection`, `speech` |
| `1.0.13` | Sprite loader + manifest resolver (read-only) | `kernel/sprite_loader.mjs` | `apw-pixel-plugin` (apw_pixel_plugin::SpriteLoader) | M4 | planned | `pixel`, `sprites`, `loader` |
| `1.0.2` | 6-component execution trace (tracer) | `kernel/tracer.mjs` | `apw-kernel` (apw_kernel::tracer) | M1 | planned | `kernel`, `tracing` |
| `1.0.3` | Contract freezer (8 + pure kinds) | `kernel/freezer.mjs` | `apw-kernel` (apw_kernel::freezer) | M1 | planned | `kernel`, `freezer`, `contracts` |
| `1.0.4` | Decision-graph algebra | `kernel/algebra.mjs` | `apw-kernel` (apw_kernel::algebra) | M1 | planned | `kernel`, `algebra`, `decisions` |
| `1.0.5` | Deterministic replay engine | `kernel/replay.mjs` | `apw-kernel` (apw_kernel::replay) | M1 | planned | `kernel`, `replay` |
| `1.0.6` | Point-in-time world snapshots | `kernel/snapshot.mjs` | `apw-kernel` (apw_kernel::snapshot) | M1 | planned | `kernel`, `snapshot` |
| `1.0.7` | Trust report + integrity verifier | `kernel/trust-report.mjs` | `apw-kernel` (apw_kernel::trust) | M1 | planned | `kernel`, `trust`, `replay` |
| `1.0.8` | Storage event middleware | `kernel/storage-events.mjs` | `apw-kernel` (apw_kernel::storage_events) | M1 | planned | `kernel`, `persistence` |
| `1.0.9` | Runtime snapshot aggregator | `kernel/runtime_snapshot.mjs` | `apw-server` (apw_server::runtime_snapshot) | M2 | planned | `server`, `snapshot`, `api` |
| `2.0.0` | LM Studio HTTP bridge (kernel-side) | `kernel/llmstudio.mjs` | `apw-engine` (apw_engine::llm::LmStudioBridge) | M2 | planned | `kernel`, `llm`, `bridge` |
| `2.0.1` | Skills-lock loader | `kernel/skills-loader.mjs` | `apw-kernel` (apw_kernel::skills_lock) | M2 | planned | `kernel`, `skills` |
| `2.0.2` | Skill dispatcher + plugin transport rule | `kernel/skill-dispatcher.mjs` | `apw-kernel` (apw_kernel::skill_dispatcher) | M2 | planned | `kernel`, `skills`, `dispatch` |
| `2.0.3` | Sandbox executor (shell / docker / git_commit / code_edit) | `kernel/sandbox-executor.mjs` | `apw-engine` (apw_engine::sandbox) | M2 | planned | `engine`, `sandbox`, `execution` |
| `2.0.4` | HTTP server hosting webui + REST | `kernel/server.mjs` | `apw-server` | M2 | planned | `server`, `http`, `api` |
| `3.0.0` | Unified LLM client (live + mock) | `engine/llm-router.mjs` | `apw-engine` (apw_engine::llm::Router) | M2 | planned | `engine`, `llm`, `router` |
| `3.0.1` | Prompt registry | `engine/prompts.mjs` | `apw-engine` (apw_engine::llm::Prompts) | M2 | planned | `engine`, `llm`, `prompts` |
| `3.0.10` | World simulator + scenario loader | `engine/simulation.mjs` | `apw-sim` | M2 | planned | `engine`, `simulation` |
| `3.0.11` | Plugin compiler (GitHub sources → plugin registry) | `engine/plugin_compiler.mjs` | `apw-plugin` (apw_plugin::Compiler) | M2 | planned | `engine`, `plugin`, `compiler` |
| `3.0.12` | Plugin runtime index | `engine/plugin_registry.mjs` | `apw-plugin` (apw_plugin::Registry) | M2 | planned | `engine`, `plugin`, `registry` |
| `3.0.13` | PID layer for sandboxed executions | `engine/execution_tracker.mjs` | `apw-engine` (apw_engine::execution_tracker) | M2 | planned | `engine`, `execution`, `tracking` |
| `3.0.14` | Drizzle-compatible JSONL store | `engine/store.mjs` | `apw-store` | M1 | porting | `store`, `persistence`, `jsonl` |
| `3.0.15` | Evaluation engine (start/finish evaluations) | `engine/evaluation.mjs` | `apw-engine` (apw_engine::evaluation) | M3 | planned | `engine`, `evaluation` |
| `3.0.16` | Benchmark engine | `engine/benchmark.mjs` | `apw-engine` (apw_engine::benchmark) | M3 | planned | `engine`, `benchmark` |
| `3.0.17` | Prompt version manager | `engine/prompt_versions.mjs` | `apw-engine` (apw_engine::llm::PromptVersionManager) | M3 | planned | `engine`, `llm`, `prompts` |
| `3.0.2` | World config + system limits loader | `engine/config-loader.mjs` | `apw-server` (apw_server::config) | M1 | porting | `server`, `config` |
| `3.0.3` | 4 fact extractors (req / con / cap / risk) | `engine/extractors.mjs` | `apw-engine` (apw_engine::extractors) | M1 | planned | `engine`, `extractors` |
| `3.0.4` | 3 deterministic compilers (arch / plan / tasks) | `engine/compilers.mjs` | `apw-engine` (apw_engine::compilers) | M1 | planned | `engine`, `compilers` |
| `3.0.5` | Trace drift, variance, confidence semantics | `engine/drift.mjs` | `apw-kernel` (apw_kernel::drift) | M1 | planned | `kernel`, `drift`, `metrics` |
| `3.0.6` | TAP planning layer (observe / suggest) | `engine/tap.mjs` | `apw-tap` | M2 | planned | `engine`, `tap`, `planning` |
| `3.0.7` | Agent runtime (registry, leases, heartbeats, reputation) | `engine/agents.mjs` | `apw-runtime` (apw_runtime::agents) | M2 | planned | `engine`, `agents`, `runtime` |
| `3.0.8` | Append-only agent mailbox | `engine/mailbox.mjs` | `apw-runtime` (apw_runtime::mailbox) | M2 | planned | `engine`, `mailbox` |
| `3.0.9` | Bid projection, market pressure, pricing | `engine/economy.mjs` | `apw-economy` | M2 | planned | `engine`, `economy`, `bidding` |
| `4.0.0` | Sticky header + 10 tab panes + footer (WebUI) | `webui/index.html` | `apw-gateway` (apw_gateway::webui) | M5 | planned | `gateway`, `webui`, `ui` |
| `4.0.1` | Hash router + 10 per-tab renderers + 2-second polling | `webui/app.js` | `apw-gateway` (apw_gateway::webui::router) | M5 | planned | `gateway`, `webui`, `router` |
| `4.0.2` | DM Serif Display + IBM Plex Mono + Inter Tight stylesheet | `webui/app.css` | `apw-gateway` (apw_gateway::webui::styles) | M5 | planned | `gateway`, `webui`, `styles` |
| `4.0.3` | Isometric office HQ (10 areas, agent lifecycle, heatmap) | `webui/app.js (renderIsometricOffice)` | `apw-gateway` (apw_gateway::webui::office_iso) | M4 | planned | `gateway`, `office`, `iso`, `projection` |
| `4.0.4` | 56+ REST API endpoints (/api/*) | `kernel/server.mjs` | `apw-server` | M2 | porting | `server`, `api`, `rest` |
| `4.0.5` | 37-section smoke test | `tests/smoke.mjs` | `apw-server` (apw_server::tests::smoke) | M1 | porting | `server`, `test`, `smoke` |
| `4.1.0` | Vanilla HTML/CSS/JS webui (no build step) | `webui/` | `apw-gateway` | M5 | not-adopted | `webui` |
| `4.1.1` | 8 new sprite roles / 3 new expressions (HQ enhancement) | `webui/app.js (HQ enhancement)` | `apw-pixel-plugin` | M4 | not-adopted | `pixel`, `sprites` |
| `4.1.2` | Drizzle ORM (Node) | `engine/store.mjs (Drizzle adapter)` | `apw-store` | M1 | not-adopted | `store`, `orm` |
| `4.1.3` | WebUI 2-second polling | `webui/app.js (polling loop)` | `apw-gateway` | M5 | not-adopted | `webui`, `polling` |

## 4. Features from `agent-bigbrother` — WebUI

Architectural spec for the same system (L0–L5, docker stack, TAP layer, control plane UI). Drives the design spec.

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status | Tags |
|---|---|---|---|---|---|---|
| `5.0.0` | L0–L5 architectural rules | `README.md (architecture section)` | `apw-rs` (docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md) | M0 | porting | `architecture`, `design` |
| `5.0.1` | Cargo workspace with 8 crates + 1 binary | `README.md (repo skeleton section)` | `apw-rs` | M0 | porting | `workspace`, `structure` |
| `5.0.2` | Control-plane UI (TAP dashboard menus) | `README.md (control plane section)` | `apw-gateway` (apw_gateway::webui) | M5 | planned | `gateway`, `webui`, `control-plane` |
| `5.0.3` | Dockerized control plane (docker-compose stack) | `Dockerfile, docker-compose.yml` | `apw-rs` | M2 | planned | `docker`, `infra` |
| `5.0.4` | TAP (LLM cognitive layer, read-only) | `tap/ (concept spec)` | `apw-tap` | M2 | planned | `tap`, `llm`, `planning` |
| `5.0.5` | Sim mode (parallel world / sandbox) | `sim/ (concept spec)` | `apw-sim` | M2 | planned | `sim`, `sandbox` |
| `5.0.6` | Plugin marketplace (29 GitHub repos → 4 categories) | `data/plugin_sources.json` | `apw-plugin` (apw_plugin::Compiler) | M2 | planned | `plugin`, `marketplace` |
| `5.0.7` | Local LLM runtime (LM Studio / llama.cpp / Ollama / vLLM) | `lmstudio/ (concept spec)` | `apw-engine` (apw_engine::llm) | M3 | planned | `llm`, `runtime`, `local` |

## 5. Features from `forgefabrik/forgefabrik-agent-os` (architecture spec)

This repository. Cross-cutting features (policies, boundaries, governance) are catalogued with source_id=apw-rs.

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status | Tags |
|---|---|---|---|---|---|---|
| `8.0.0` | MSRV pin (rust-toolchain.toml = 1.82) | `rust-toolchain.toml` | `apw-rs` | M0 | porting | `policy`, `msrv` |
| `8.0.1` | #![forbid(unsafe_code)] in every crate | `crates/*/src/lib.rs (preamble)` | `apw-rs` | M0 | porting | `policy`, `safety` |
| `8.0.10` | Machine-readable Feature Registry (features.registry.json) | `docs/FEATURES.md (this file)` | `apw-rs` (features.registry.json + tools/feature-*) | M0 | porting | `meta`, `registry`, `ci` |
| `8.0.11` | feature-guardian: registry validator (CI-enforced) | `tools/feature-guardian/` | `apw-rs` (tools/feature-guardian/) | M0 | porting | `meta`, `ci`, `validation` |
| `8.0.12` | feature-md: JSON→Markdown renderer (FEATURES.md is a compiled artifact) | `tools/feature-md/` | `apw-rs` (tools/feature-md/) | M0 | porting | `meta`, `render`, `ci` |
| `8.0.13` | feature-harvester: auto-ingest upstream (README + file tree + endpoint scan) | `tools/feature-harvester/` | `apw-rs` (tools/feature-harvester/) | M1 | planned | `meta`, `automation`, `ingest` |
| `8.0.14` | feature-graph: dependency DAG + critical-path + 'what blocks Mx?' | `tools/feature-graph/` | `apw-rs` (tools/feature-graph/) | M0 | porting | `meta`, `graph`, `ci` |
| `8.0.2` | Layer 1/2 boundary tests (tests/boundary.rs per crate) | `crates/*/tests/boundary.rs` | `apw-rs` | M0 | porting | `policy`, `boundaries`, `test` |
| `8.0.3` | BTreeMap / BTreeSet only in replay-authoritative state | `docs/superpowers/specs/* (Deterministic Iteration Policy)` | `apw-kernel` (apw_protocol) | M0 | porting | `policy`, `determinism`, `iteration` |
| `8.0.4` | ClockSource / SimulationClock / RandomSource traits | `docs/superpowers/specs/* (Time + Randomness policies)` | `apw-protocol` | M0 | porting | `policy`, `time`, `randomness`, `trait` |
| `8.0.5` | Capability enum + AuthorityMap type alias | `docs/superpowers/specs/* (Capability Policy)` | `apw-protocol` | M0 | porting | `policy`, `capability`, `authority` |
| `8.0.6` | EventEnvelope::schema_version: u32 on every versioned type | `docs/superpowers/specs/* (Event Versioning Policy)` | `apw-protocol` | M0 | porting | `policy`, `versioning`, `events` |
| `8.0.7` | ADR process under docs/adr/ | `docs/superpowers/specs/* (ADR Policy)` | `apw-rs` | M0 | porting | `policy`, `adr`, `governance` |
| `8.0.8` | cargo-deny license + advisory gates | `opencode-manager inspired (stricter)` | `apw-rs` (.cargo/deny.toml + CI) | M1 | planned | `policy`, `license`, `security`, `ci` |
| `8.0.9` | cargo metadata graph validator (replaces grep boundary tests) | `docs/superpowers/specs/* (M0 grep known limitations)` | `apw-rs` (tools/check-boundaries/) | M1 | planned | `policy`, `boundaries`, `graph` |

## 6. Features from `chriswritescode-dev/opencode-manager` (mobile-first PWA)

Mobile-first PWA control plane for OpenCode agents. Drives the manager/web feature surface (M2, M5).

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status | Tags |
|---|---|---|---|---|---|---|
| `6.0.0` | Multi-repo management (per-repo config, worktrees, SSH auth) | `backend/src/repos/` | `apw-server` (apw_server::repos) | M5 | planned | `server`, `repos`, `git` |
| `6.0.1` | Unified diffs, branch + commit management | `backend/src/git/` | `apw-server` (apw_server::git) | M5 | planned | `server`, `git`, `diffs` |
| `6.0.10` | Push notifications (VAPID) for session events / questions / errors | `backend/src/push/` | `apw-server` (apw_server::push) | M5 | planned | `server`, `push`, `notifications` |
| `6.0.11` | Audio TTS/STT (browser native + OpenAI-compatible) | `frontend/src/audio/` | `apw-gateway` (apw_gateway::audio) | M5 | planned | `gateway`, `audio`, `tts`, `stt` |
| `6.0.12` | PWA (service worker, installable) | `frontend/public/sw.js + manifest` | `apw-gateway` (apw_gateway::pwa) | M5 | planned | `gateway`, `pwa` |
| `6.0.13` | Mobile-first responsive navigation | `frontend/src/layouts/` | `apw-gateway` (apw_gateway::layouts) | M5 | planned | `gateway`, `ui`, `layout` |
| `6.0.14` | ocm CLI (attach local TUI, push/pull tarball sync) | `ocm-cli/` | `apw-cli` (apw_cli::ocm) | M2 | planned | `cli`, `sync` |
| `6.0.2` | File tree view, syntax highlighting, ZIP download | `frontend/src/files/` | `apw-gateway` (apw_gateway::files) | M5 | planned | `gateway`, `files`, `ui` |
| `6.0.3` | Real-time SSE chat streaming | `backend/src/sessions/sse` | `apw-server` (apw_server::sessions::sse) | M2 | porting | `server`, `sse`, `chat`, `streaming` |
| `6.0.4` | Slash commands, @file mentions, Plan/Build modes | `frontend/src/chat/` | `apw-gateway` (apw_gateway::chat) | M2 | planned | `gateway`, `chat`, `ui` |
| `6.0.5` | Mermaid diagram rendering | `frontend/src/components/Mermaid.tsx` | `apw-gateway` (apw_gateway::components::mermaid) | M2 | planned | `gateway`, `ui`, `diagrams` |
| `6.0.6` | Schedules (recurring repo jobs, run history, linked sessions) | `backend/src/schedules/` | `apw-server` (apw_server::schedules) | M5 | planned | `server`, `schedules`, `cron` |
| `6.0.7` | MCP server management (add, configure, OAuth) | `backend/src/mcp/` | `apw-server` (apw_server::mcp) | M5 | planned | `server`, `mcp`, `oauth` |
| `6.0.8` | AI configuration (models, providers, API keys, OAuth) | `backend/src/ai/` | `apw-server` (apw_server::ai_config) | M3 | planned | `server`, `ai`, `config` |
| `6.0.9` | Skills (shareable, scoped definitions) | `backend/src/skills/` | `apw-engine` (apw_engine::skills) | M2 | planned | `engine`, `skills` |
| `6.1.0` | React + Vite frontend | `frontend/` | `apw-gateway` | M5 | not-adopted | `frontend` |
| `6.1.1` | Bun runtime | `package.json (bun scripts)` | `apw-server` | M2 | not-adopted | `runtime` |

## 7. Features from `IvanWng97/pixtuoid` (Rust terminal pixel-art office)

Rust terminal pixel-art office for AI agents. Drives apw-office TUI (M4) and the apw-hook shim pattern. The only pure-Rust source project.

| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status | Tags |
|---|---|---|---|---|---|---|
| `7.0.0` | 3-crate workspace (pixtuoid-core / pixtuoid / pixtuoid-hook) | `crates/` | `apw-rs` | M0 | porting | `workspace`, `structure` |
| `7.0.1` | Source trait (hookable agent CLI adapter) | `crates/core/src/source.rs` | `apw-hook` (apw_hook::Source) | M4 | planned | `office`, `hook`, `trait` |
| `7.0.10` | Hook shim (200ms timeout, always exit 0) | `crates/pixtuoid-hook/src/main.rs` | `apw-hook` | M0 | porting | `office`, `hook`, `shim` |
| `7.0.11` | Unix-socket transport /tmp/pixtuoid-{uid}.sock | `crates/core/src/transport/unix.rs` | `apw-hook` (apw_hook::transport::unix) | M4 | planned | `office`, `transport`, `unix-socket` |
| `7.0.12` | 6 built-in themes (Normal, Cyberpunk, Dracula, Tokyo Night, Catppuccin, Gruvbox) | `crates/pixtuoid/src/theme/` | `apw-office` (apw_office::theme) | M4 | planned | `office`, `theme`, `ui` |
| `7.0.13` | Source adapters (Claude Code, Codex, Antigravity) | `crates/core/src/sources/` | `apw-hook` (apw_hook::sources) | M4 | planned | `office`, `agents`, `adapter` |
| `7.0.2` | Reducer → SceneState (watch channel) | `crates/core/src/reducer.rs` | `apw-pixel-plugin` (apw_pixel_plugin::reducer) | M4 | planned | `office`, `reducer`, `state` |
| `7.0.3` | Pose → pixel painter → half-block → ratatui rendering | `crates/pixtuoid/src/render/` | `apw-office` (apw_office::render) | M4 | planned | `office`, `render`, `ratatui` |
| `7.0.4` | Multi-floor office (page navigation, slide transition) | `crates/pixtuoid/src/ui/office.rs` | `apw-office` (apw_office::office) | M4 | planned | `office`, `tower`, `multi-floor` |
| `7.0.5` | Per-tool monitor glow (Edit = blue, Bash = orange, Read = cyan) | `crates/pixtuoid/src/ui/glow.rs` | `apw-office` (apw_office::glow) | M4 | planned | `office`, `ui`, `tool-glow` |
| `7.0.6` | Per-agent identity (deterministic palette from session hash, 16 outfits) | `crates/core/src/palette.rs` | `apw-pixel-plugin` (apw_pixel_plugin::palette) | M4 | planned | `office`, `identity`, `palette` |
| `7.0.7` | Weather effects (rain, storm, snow, fog, sunset) | `crates/pixtuoid/src/ui/weather.rs` | `apw-office` (apw_office::weather) | M4 | planned | `office`, `weather` |
| `7.0.8` | Tooltip stats (session duration, tool calls, active %) | `crates/pixtuoid/src/ui/tooltip.rs` | `apw-office` (apw_office::tooltip) | M4 | planned | `office`, `ui`, `stats` |
| `7.0.9` | Office pets (cat / dog, sleep near idle agents) | `crates/pixtuoid/src/ui/pets.rs` | `apw-office` (apw_office::pets) | M4 | planned | `office`, `pets`, `ambient` |

### Phase-1 not-adopted features

| id | name | source | notes |
|---|---|---|---|
| `4.1.0` | Vanilla HTML/CSS/JS webui (no build step) | `agent-bigbrother` | Superseded by Dioxus fullstack in apw-gateway. The runtime snapshot shape and endpoints are preserved; the renderer is replaced. |
| `4.1.1` | 8 new sprite roles / 3 new expressions (HQ enhancement) | `agent-bigbrother` | Folded into 1.0.10 (pixel-agent projection). The expression set is extended in M4 when the projection lands. |
| `4.1.2` | Drizzle ORM (Node) | `agent-bigbrother` | Replaced by apw-store (memory / fs / sqlite adapters behind one trait). |
| `4.1.3` | WebUI 2-second polling | `agent-bigbrother` | Replaced by SSE streaming in apw-server for live data; polling kept as a fallback. |
| `6.1.0` | React + Vite frontend | `opencode-manager` | Replaced by Dioxus fullstack (single Rust codebase). |
| `6.1.1` | Bun runtime | `opencode-manager` | Replaced by tokio in apw-server. |

---

## 7. Scope split: Phase 1 (Visual Office Foundation + Living Office Simulation) vs Phase 2 (Economy & Multi-Role + Life-Cycle Roadmap)

`apw-rs` is built in two clearly separated phases. The **visual office and the living simulation** are the foundation; everything in Phase 2 lands on top of it.

| | **Phase 1 — Visual Office + Life-Sim Foundation** (in scope, current build) | **Phase 2 — Economy, Multi-Role & Life-Cycle** (roadmap, after Phase 1) |
|---|---|---|
| **What it is** | The pixel-art office surface — agents as characters, projects as rooms, per-project towers, living-room personalization, day/night cycle, shift scheduler, supermarket/pool/bar facilities, social graph | The economic engine + multi-role orchestration + full lifecycle (Kindergarten→School→University→Work→Retirement→Archive) that drives the office |
| **When it ships** | After the M0 (workspace skeleton) + M4 (office TUI + pixel pipeline) milestones, with Life-Sim features layered in | Roadmap. Not part of any current M-numbered milestone. |
| **Crate surface used** | `apw-protocol` (wire types), `apw-pixel-plugin` (sprite/character data), `apw-office` (TUI renderer), `apw-gateway` (web control plane) | Adds `apw-kernel` (state machine, authority spine of the CEO), `apw-engine` (LLM, agents, scheduler, life-sim, life-cycle), `apw-store` (ledger + persistence), `apw-server` (HTTP + SSE), `apw-life-sim`, `apw-life-cycle` |
| **Where the data comes from** | A simple local JSON / sample data adapter that exercises the office surfaces | The kernel event log + LM Studio LLM with TypeScript-plugin tool surface |
| **In this file** | §3–§6 (sources), §9 (cross-cutting) | **§8 (this document)** |

The protocol types in `apw-protocol/src/lib.rs` (Role, AuthorityMap, Capability, Event variants like `AgentPromoted`, `ItemPurchased`, `CapabilityDenied`, `TimeTick`, `AgentStateTransition`) are **forward-compatible placeholders**: they exist so Phase 1 can render the *state* of Phase 2 systems without Phase 2 existing. The implementations come in Phase 2.

## 8. Phase 2 — Economy, Multi-Role & Life-Cycle (Roadmap)

> **Status: deferred (Phase 2).** Not part of the current build. This section is the architectural seed: it documents the bigger vision and the protocol-level primitives that the office already anticipates, so that the Phase 1 surface has stable shape and the Phase 2 build can land without breaking it.
>
> The full Phase 2 design — including the LM Studio plugin bridge, the customer→CEO→worker flow, the tower-climb/economy mechanics, the day/night simulation loop, and the full life-cycle — will be written as a separate design spec **after** the Phase 1 office is finished. That spec is the terminal deliverable mentioned in the [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md).

### 8.1 The bigger vision (one paragraph)

A customer posts a job to a project tower. The CEO — an LLM running on LM Studio via the TypeScript plugin SDK — sees the new job, decomposes it, and either spawns worker agents (each an LLM with its own tools) or assigns existing ones. Workers climb the project's tower as they deliver value (merged PRs, passing tests, milestones); higher floors unlock better office infrastructure. Workers earn money for their work and spend it in their personal living rooms. The CEO earns money from project revenue and spends it on new rooms, faster PCs (more capable models), or office perks. A day/night cycle (Morning→WorkBlock→Lunch→Afternoon→AfterWork→Night) drives automatic events: shift assignment, supermarket runs, pool recovery, bar social, training. Agents are not NPCs — they are born in Kindergarten, grow in School, specialize in University, work on Tower Economy, retire to the RetirementHome, and are compressed into the KnowledgeGraph at the Crematorium. Each generation is better or more specialized than the last. Visual office state is a **compiled projection of the simulation state**, never the trigger. Idle state = no revenue = no growth = office decay. The game loop is a closed economic + lifecycle system, not a cosmetic animation.

### 8.2 Architectural primitives (forward-compat types)

These are the Rust types/traits that the office surface will reference in Phase 1 (as enum variants or trait method signatures, often no-op), and that the Phase 2 kernel/engine/life-cycle will implement.

| # | Primitive | Purpose | Phase 1 status |
|---|---|---|---|
| 8.2.1 | Role::Ceo, Role::Worker, Role::Customer (already in apw_protocol::Role) | Distinguishes actors in the multi-role system. CEO is the orchestrator; workers execute; customers post jobs. | Defined (Role enum has Ceo + 8 others; Worker/Customer to be added) |
| 8.2.2 | AuthorityMap = BTreeMap<ActorId, BTreeSet<Capability>> | Per-actor permission set. CEO has broad capabilities; workers are restricted. | Defined (type alias exists; population rules in Phase 2) |
| 8.2.3 | Capability::PromoteAgent, AllocateLease, SubmitSpriteProposal, ModifyAuthorityMap, RunTowerAdmin, ReplayChain | Typed capability enum (typo-resistant). Capabilities gate the LLM's tool surface. | Defined (enum exists; checked at runtime in Phase 2) |
| 8.2.4 | BiddingEngine trait | Customer posts a job → workers/CEO submit bids → winner is chosen by rule → lease is allocated. | Stub (trait signature only, no impl) |
| 8.2.5 | TrustVerifier trait | Verifies a claimed contribution is genuine (CI result, test result, peer review). | Stub (trait signature only, no impl) |
| 8.2.6 | Wallet, Item, Catalog | Per-actor money balance with append-only ledger. Buyable personal items for living rooms. | Not defined (to be added in Phase 2) |
| 8.2.7 | Upgrade (Room | Pc | Perk) | Buyable office upgrades for the CEO: new rooms, faster PCs, perks. | Not defined (to be added in Phase 2) |
| 8.2.8 | Tower, Floor, ClimbEvent | Per-project tower; each floor is an unlock tier. Height is a function of revenue ÷ floor_cost. | Not defined (pixel-plugin will render) |
| 8.2.9 | LivingRoom, owned_items, layout_grid | Per-agent personal space. Rendered as a small room adjacent to the work area. | Not defined (Phase 2) |
| 8.2.10 | MoneyEvent ledger entries | Append-only ledger: ContributionMerged, ProjectCompleted, CiPassed, ItemPurchased, OfficeDecay. | Not defined (Phase 2) |
| 8.2.11 | TeamRole (CEO, CODER, DESIGNER, TESTER) + fixed topology | Job-family axis. Fixed: 3 CODER + 3 DESIGNER + 1-3 TESTER + 1 CEO. Coexists with authority Role. | Not defined (new in Phase 2) |
| 8.2.12 | WorldClock, DayPhase, automatic day events | Two-layer time: Tick (kernel) is atomic; WorldClock is the derived projection with day phases. Drives automatic shift assignment, shop open hours, etc. | Not defined (kernel emits TimeTick; engine projects DayPhase) |
| 8.2.13 | AgentState (Working, Idle, Eating, Training, Socializing, Recovering) | Idle is NOT a non-state — it's an active state where the agent trains, socializes, recovers. No AFK idling. | Not defined (idle becomes an active routing target) |
| 8.2.14 | OfficeStats (productivity, stress, social_cohesion, creativity, fatigue) | Per-team metrics that couple back into task success, errors, burnout. | Not defined (engine couples metrics → outcomes) |
| 8.2.15 | Supermarket, Pool, Bar facilities | Resource system (items + buffs), recovery engine (stamina/stress), social graph engine (bonding, idea spawns). | Not defined (life-sim crate) |
| 8.2.16 | LifeStage (Infant, Child, Student, JuniorWorker, SeniorWorker, Expert, Mentor, Retired, Archived) | The lifecycle state machine. 9 states with deterministic transitions. | Not defined (life-cycle crate) |
| 8.2.17 | Kindergarten, School, University, RetirementHome, Crematorium | Lifecycle buildings. New agents are born in Kindergarten, retire to RetirementHome, are compressed into the KnowledgeGraph at the Crematorium. | Not defined (life-cycle crate) |
| 8.2.18 | MemoryPacket (skills, experiences, decisions, confidence_weight) | A bundle of knowledge extracted from an agent. Transferred through School→University→Work→Retirement→Archive. | Not defined (life-cycle crate) |
| 8.2.19 | KnowledgeGraph (nodes: MemoryPacket[], edges: KnowledgeRelation[]) | The collective memory. Agents learn not only individually but from collective past. | Not defined (life-cycle crate) |
| 8.2.20 | Generation (inherited_skills, innovation_factor) | Each new agent generation is better or more specialized. innovation_factor represents the new skills/patterns NOT inherited. | Not defined (life-cycle crate) |
| 8.2.21 | LM Studio CeoBridge plugin adapter | The CEO is an LLM running on LM Studio via @lmstudio/sdk. The plugin exposes kernel operations as tool() definitions gated by AuthorityMap. | Not defined (engine crate) |

Full feature entries for these primitives: see §8.5 below.


### 8.3 Full life loop (Kindergarten → ... → KnowledgeGraph)

```
Kindergarten (Early Learning Hub)
   ↓
School (Skill Foundation System: CODER/DESIGNER/TESTER baseline)
   ↓
University (Advanced Specialization: AI, Architecture, Systems, Design)
   ↓
Work (Tower Economy: 3 CODER + 3 DESIGNER + 1-3 TESTER + CEO)
   ↓
Retirement Home (Legacy Preservation: agent becomes Knowledge Node)
   ↓
Crematorium (Memory Compression: archive into global knowledge)
   ↓
KnowledgeGraph (System Memory: agents learn from collective past)
```

The four design rules:
1. **No hard delete.** Every agent leaving the active pool emits a memory packet. No `delete agent` API exists.
2. **Memory is economy-relevant.** Better memory → better agents → higher revenue. The knowledge graph is queried by the BiddingEngine and the LLM router.
3. **School/University affect real output.** No cosmetic XP system. A Senior Worker who went through University produces measurably different output than one who didn't.
4. **Love is a multiplier, not UI.** Relationship.affinity is a multiplier on task success, collaboration speed, creativity. The visual representation is a derived state, not a UI choice.

### 8.4 Phase 2 milestones (preliminary, not committed)

The roadmap will be updated when the Phase 2 spec is written. A likely shape, subject to revision:

- **P2.0** — Protocol extension: add `TeamRole`, `WorldClock`, `DayPhase`, `OfficeStats`, `AgentState`, `LifeStage`, `MemoryPacket`, `KnowledgeGraph`, `Generation`, plus all facility types, as forward-compat types in `apw-protocol` and the new `apw-life-sim` / `apw-life-cycle` crates.
- **P2.1** — Engine: implement revenue computation, MoneyEvent emission, Wallet ledger, OfficeStats coupling.
- **P2.2** — Engine: implement `BiddingEngine` (post-job → bid → resolve → lease).
- **P2.3** — Kernel: implement `TrustVerifier` (verify contribution, update trust score).
- **P2.4** — Engine: implement `ClimbEvent` emission, `Tower` height computation, `WorldClock` projection from `Tick`.
- **P2.5** — `apw-engine::CeoBridge` — the LM Studio plugin adapter. The CEO's tool surface is the set of kernel operations gated by `AuthorityMap`.
- **P2.6** — `apw-server` — `CustomerIntake` HTTP surface, SSE for live state.
- **P2.7** — `apw-pixel-plugin` — render Tower, LivingRoom, Supermarket, Pool, Bar, Lifecycle buildings from the simulation state.
- **P2.8** — `apw-life-cycle` — full lifecycle loop: Kindergarten → School → University → Work → Retirement → Crematorium → KnowledgeGraph.
- **P2.9** — Generation drift: each new agent generation reflects the cumulative knowledge graph.
- **P2.10** — Office decay loop, end-to-end anti-AFK rule.

These are **pre-decision notes**, not a commitment. The actual Phase 2 spec is the terminal deliverable.

### 8.5 Kernel = CEO (architectural principle)

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

**Naming note:** `Role::Ceo` in the protocol is the *role tag* the kernel applies to the (kernel+LLM+engine) trio when it acts. The trio has many internal events (LLM emits `Intent`, engine emits `Event`); the role tag tells the rest of the system "this event was produced by the CEO's joint action". The kernel is the part of the trio that *can* be tagged `Ceo` for the purposes of `AuthorityMap` lookups, because the kernel is the authoritative spine.

### 8.6 Anti-patterns the design avoids

- **Cosmetic-only XP bars.** The tower is not an animation; it is a derived state of `Wallet` revenue vs `Floor.cost`. Without revenue, no climb.
- **Idle state earns money.** No revenue from agent turn, thinking, or idle animation. Revenue only from merged contribution, CI-passed release, project completion, or customer delivery.
- **AFK-wealthy state.** Office decay rule ensures no-output companies stagnate.
- **LLM-as-authority.** The LLM emits `Intent` types; the kernel turns them into authoritative events. The LLM never writes to the event chain directly. (See [design spec §Determinism Policy](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md).)
- **Stringly-typed roles.** `Role` is an enum, not a freeform string. `TeamRole` is a separate enum for the job-family axis. Capabilities are typed, not strings. Authority is a `BTreeMap`, not a hash.
- **Hard delete.** No `delete agent`. Every agent leaving the active pool emits a memory packet to the KnowledgeGraph. Death is compression, not deletion.
- **Static agents.** Agents evolve through Kindergarten → School → University → Work → Retirement → Archive. Generation drift makes each cohort measurably different.
- **No-feedback simulation.** Metrics (stress, social, fatigue) couple back into task success. A team with no pool burns out; a team with no food crashes; a team with no social creativity stalls.

### 8.7 Phase 2 features (registry entries)

| id | name | milestone | status | tags |
|---|---|---|---|---|
| `10.0.0` | Kindergarten (Early Learning Hub) | M6+ | deferred | `lifecycle`, `birth`, `learning` |
| `10.0.1` | School (Skill Foundation System) | M6+ | deferred | `lifecycle`, `school`, `skills` |
| `10.0.10` | Memory flow: School → University → Work → Retirement → Archive | M6+ | deferred | `lifecycle`, `memory`, `flow` |
| `10.0.11` | Design rule: no hard delete (everything is preserved or compressed) | M6+ | deferred | `lifecycle`, `policy`, `rule` |
| `10.0.12` | Design rule: memory is economy-relevant | M6+ | deferred | `lifecycle`, `policy`, `rule`, `economy` |
| `10.0.13` | Design rule: school/university affect real output | M6+ | deferred | `lifecycle`, `policy`, `rule`, `education` |
| `10.0.14` | Design rule: love system is multiplier, not UI feature | M6+ | deferred | `lifecycle`, `policy`, `rule`, `social` |
| `10.0.15` | Full life loop: Kindergarten → School → University → Work → Retirement → Crematorium → KnowledgeGraph | M6+ | deferred | `lifecycle`, `loop`, `integration` |
| `10.0.2` | University (Advanced Specialization Engine) | M6+ | deferred | `lifecycle`, `university`, `specialization` |
| `10.0.3` | Relationship struct (Social Bond Layer) | M6+ | deferred | `lifecycle`, `social`, `bond` |
| `10.0.4` | RetirementHome (Legacy Preservation Layer) | M6+ | deferred | `lifecycle`, `retirement`, `memory`, `legacy` |
| `10.0.5` | Crematorium (Finalization / Memory Compression) | M6+ | deferred | `lifecycle`, `death`, `compression`, `archive` |
| `10.0.6` | LifeStage enum (Infant, Child, Student, JuniorWorker, SeniorWorker, Expert, Mentor, Retired, Archived) | M6+ | deferred | `lifecycle`, `stage`, `state-machine` |
| `10.0.7` | MemoryPacket struct (skills, experiences, decisions, confidence_weight) | M6+ | deferred | `lifecycle`, `memory`, `packet` |
| `10.0.8` | KnowledgeGraph struct (nodes: MemoryPacket[], edges: KnowledgeRelation[]) | M6+ | deferred | `lifecycle`, `knowledge`, `graph` |
| `10.0.9` | Generation struct (id, inherited_skills, innovation_factor) | M6+ | deferred | `lifecycle`, `generation`, `evolution` |
| `9.0.0` | TeamRole enum (CEO, CODER, DESIGNER, TESTER) | M6+ | deferred | `lifesim`, `role`, `team` |
| `9.0.1` | Fixed team topology rule (3 CODER + 3 DESIGNER + 1-3 TESTER + 1 CEO) | M6+ | deferred | `lifesim`, `team`, `rule` |
| `9.0.10` | CEO tool surface (purchase_item, assign_shift, open_facility, evaluate_team_state, rebalance_resources) | M6+ | deferred | `lifesim`, `ceo`, `tools`, `capability` |
| `9.0.11` | WorldClock struct (tick, phase) | M6+ | deferred | `lifesim`, `time`, `day` |
| `9.0.12` | DayPhase enum (Morning, WorkBlock, Lunch, Afternoon, AfterWork, Night) | M6+ | deferred | `lifesim`, `time`, `phase` |
| `9.0.13` | Automatic day events (Morning→shift, Lunch→shop+bar, etc.) | M6+ | deferred | `lifesim`, `events`, `scheduler` |
| `9.0.14` | OfficeStats struct (productivity, stress, social_cohesion, creativity, fatigue) | M6+ | deferred | `lifesim`, `metrics`, `stats` |
| `9.0.15` | Metric→outcome coupling (stress↑→errors↑, social↓→creativity↓, no food→productivity crash, no pool→burnout) | M6+ | deferred | `lifesim`, `coupling`, `rules` |
| `9.0.16` | apw-engine: life_simulator module | M6+ | deferred | `lifesim`, `engine`, `module` |
| `9.0.17` | apw-engine: shift_scheduler module | M6+ | deferred | `lifesim`, `engine`, `scheduler`, `shift` |
| `9.0.18` | apw-engine: resource_economy module | M6+ | deferred | `lifesim`, `engine`, `resource`, `economy` |
| `9.0.19` | apw-pixel-plugin: bar animation | M6+ | deferred | `lifesim`, `pixel`, `animation`, `bar` |
| `9.0.2` | Shift struct (id, agents, start_tick, end_tick, mode) | M6+ | deferred | `lifesim`, `shift`, `time` |
| `9.0.20` | apw-pixel-plugin: supermarket shelves | M6+ | deferred | `lifesim`, `pixel`, `animation`, `supermarket` |
| `9.0.21` | apw-pixel-plugin: pool zone | M6+ | deferred | `lifesim`, `pixel`, `animation`, `pool` |
| `9.0.22` | apw-pixel-plugin: idle training animations | M6+ | deferred | `lifesim`, `pixel`, `animation`, `idle` |
| `9.0.23` | apw-kernel: time_tick_event | M6+ | deferred | `lifesim`, `kernel`, `event`, `time` |
| `9.0.24` | apw-kernel: agent_state_transition_event | M6+ | deferred | `lifesim`, `kernel`, `event`, `state` |
| `9.0.3` | ShiftMode enum (Work, Break, Social, Training) | M6+ | deferred | `lifesim`, `shift`, `mode` |
| `9.0.4` | AgentState enum (Working, Idle, Eating, Training, Socializing, Recovering) | M6+ | deferred | `lifesim`, `agent`, `state` |
| `9.0.5` | Idle = active state (no AFK) | M6+ | deferred | `lifesim`, `idle`, `anti-afk` |
| `9.0.6` | Supermarket (Resource System) | M6+ | deferred | `lifesim`, `resource`, `shop` |
| `9.0.7` | Pool (Recovery Engine) | M6+ | deferred | `lifesim`, `recovery`, `facility` |
| `9.0.8` | Bar (Social Graph Engine) | M6+ | deferred | `lifesim`, `social`, `facility` |
| `9.0.9` | LM Studio CEO tool loop | M6+ | deferred | `lifesim`, `ceo`, `lmstudio`, `loop` |

---

## 9. Cross-cutting (introduced by `apw-rs` itself)

These are not from any single upstream; they are new in `apw-rs` to satisfy governance policies locked in the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md).

| # | Feature | Driver | Crate | Milestone | Status |
|---|---|---|---|---|---|
| `8.0.0` | MSRV pin (rust-toolchain.toml = 1.82) |  | `apw-rs` | M0 | porting |
| `8.0.1` | #![forbid(unsafe_code)] in every crate |  | `apw-rs` | M0 | porting |
| `8.0.10` | Machine-readable Feature Registry (features.registry.json) |  | `apw-rs` | M0 | porting |
| `8.0.11` | feature-guardian: registry validator (CI-enforced) |  | `apw-rs` | M0 | porting |
| `8.0.12` | feature-md: JSON→Markdown renderer (FEATURES.md is a compiled artifact) |  | `apw-rs` | M0 | porting |
| `8.0.13` | feature-harvester: auto-ingest upstream (README + file tree + endpoint scan) |  | `apw-rs` | M1 | planned |
| `8.0.14` | feature-graph: dependency DAG + critical-path + 'what blocks Mx?' |  | `apw-rs` | M0 | porting |
| `8.0.2` | Layer 1/2 boundary tests (tests/boundary.rs per crate) |  | `apw-rs` | M0 | porting |
| `8.0.3` | BTreeMap / BTreeSet only in replay-authoritative state |  | `apw-kernel` | M0 | porting |
| `8.0.4` | ClockSource / SimulationClock / RandomSource traits |  | `apw-protocol` | M0 | porting |
| `8.0.5` | Capability enum + AuthorityMap type alias |  | `apw-protocol` | M0 | porting |
| `8.0.6` | EventEnvelope::schema_version: u32 on every versioned type |  | `apw-protocol` | M0 | porting |
| `8.0.7` | ADR process under docs/adr/ |  | `apw-rs` | M0 | porting |
| `8.0.8` | cargo-deny license + advisory gates |  | `apw-rs` | M1 | planned |
| `8.0.9` | cargo metadata graph validator (replaces grep boundary tests) |  | `apw-rs` | M1 | planned |

## 10. How to use this file

- **Adding a feature?** Edit `features.registry.json` directly. Add a row with `id`, `name`, `phase`, `milestone`, `status`, and at least one of `from`/`to`. Then run `cargo run -p feature-md -- render` to regenerate this file, and `cargo run -p feature-guardian -- check` to validate. The commit that lands the change must include both the registry diff and the regenerated `FEATURES.md`.
- **Porting a feature?** Change `status` from `planned (Mx)` to `porting (Mx)` when the work starts, and to `done` when the feature ships. The commit that lands the change updates the registry in the same diff.
- **Dropping a feature?** Move the row to the *Not adopted* subsection of its source (use `status: "not-adopted"`), with a one-line reason in `notes`. Never silently delete.
- **Renaming a target crate?** Grep `features.registry.json` for the old crate name and update in the same commit as the rename. Every feature with a `to.crate` field should reference a real workspace member (or a documented Phase-2 crate).
- **Reviewer?** If a PR adds a new public surface in `apw-*` crates, it should also have added a row to the registry and regenerated this file. Reject the PR if not.
- **CI:** `feature-guardian check --strict` runs in GitHub Actions. The build fails if any check errors or warnings appear.
- **Visualizing dependencies:** `cargo run -p feature-graph -- show --milestone Mx` renders the dependency DAG. `cargo run -p feature-graph -- critical-path --milestone Mx` shows the longest chain. `cargo run -p feature-graph -- blocks --milestone Mx` answers "what blocks Mx?".
- **Auto-ingesting upstream:** `cargo run -p feature-harvester -- scan <owner/repo>` emits a JSON of feature candidates that can be diffed against the registry.

The `apw-rs` repo at any commit should be able to answer "where does this feature come from, where is it going, and what's its status?" from `features.registry.json` alone. This Markdown file is a generated view of that single source of truth.

