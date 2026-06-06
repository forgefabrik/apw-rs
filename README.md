# apw-rs

> Pure-Rust reimplementation of the ForgeFabrik Agent Operating System — a deterministic, hash-chained event-sourced agent OS with a kernel/engine, an HTTP control plane, a mobile-first web UI, and a terminal pixel-art office for visualizing live agents.

**Status:** M0 (workspace skeleton) in design. No code yet; see [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md).

## Source projects being ported

| Source | Domain | In this repo |
|---|---|---|
| [forgefabrik/agent-bigbrother](https://github.com/forgefabrik/agent-bigbrother) | ForgeFabrik Agent OS v0.2a — deterministic kernel, hash-chained events, scheduler, economy, TAP, LM Studio bridge, pixel projections | `apw-kernel`, `apw-engine`, `apw-store`, `apw-server`, `apw-pixel-plugin` |
| [forgefabrik/forgefabrik-agent-os](https://github.com/forgefabrik/forgefabrik-agent-os) | Architectural spec for the same system (L0–L5) | Drives the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md) |
| [chriswritescode-dev/opencode-manager](https://github.com/chriswritescode-dev/opencode-manager) | Mobile-first PWA control plane for OpenCode agents (multi-repo, git, file browser, SSE chat, MCP, schedules, push) | `apw-gateway` (web UI surface) and `apw-manager` TUI |
| [IvanWng97/pixtuoid](https://github.com/IvanWng97/pixtuoid) | Terminal pixel-art office for AI agents (ratatui, multi-floor, animated sprites, themes, weather, pets) | `apw-office` TUI (replaces the Node v0.2a isometric webui) |

## Milestones

| ID | Name | Scope |
|---|---|---|
| **M0** | Workspace skeleton (in design) | 8 crates + 1 CLI, boundary tests, MSRV 1.82, MIT license, no business logic |
| **M1** | Kernel core port | `apw-kernel` — event-core, algebra, freezer, trust-report, replay, snapshot, canonical serializer |
| **M2** | Engine + server | `apw-engine` (agents, mailbox, economy, scheduler, sandbox, llm-router) + `apw-server` (axum) |
| **M3** | LLM admin | `TowerAdmin` trait; rule-based default + self-hosted LLM (llama.cpp / Ollama / vLLM) |
| **M4** | Office TUI + pixel pipeline | `apw-office` ratatui TUI + `apw-pixel-plugin` Aseprite pipeline |
| **M5** | Manager TUI + gateway | `apw-manager` ratatui TUI + `apw-gateway` static server / reverse proxy |

Full breakdown: [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md).

## Planned architecture (M0)

```
apw-rs (Cargo workspace)
├── shared/
│   └── crates/apw-protocol        # wire types only (serde, no I/O, no runtime)
├── server/                        # zero client deps
│   ├── crates/apw-kernel          # event chain, replay, algebra, trust
│   ├── crates/apw-engine          # agents, mailbox, economy, scheduler, sandbox, llm
│   ├── crates/apw-store           # storage trait + adapters (memory, fs, sqlite)
│   └── crates/apw-server          # axum binary, composes kernel+engine+store
├── client/                        # zero server deps
│   ├── crates/apw-office          # ratatui TUI — terminal pixel-art office
│   ├── crates/apw-manager         # ratatui TUI — file tree, sessions, terminals
│   ├── crates/apw-gateway         # static file server + reverse proxy
│   └── crates/apw-pixel-plugin    # Aseprite JSON + PNG manifest parser
└── tools/
    └── apw-cli                    # top-level `apw` binary (office, replay, status)
```

8 internal crates + 1 binary. No catch-all crate.

## Governance (locked at M0)

The workspace enforces strict architectural and governance policies. The full text lives in [the design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md); the short version:

- **Time Policy** — `std::time::SystemTime` is forbidden in `apw-kernel`. Authoritative time is a `Tick` driven by a `ClockSource` trait.
- **Determinism Policy** — kernel replay produces identical state. No floats in canonical paths. No `HashMap`/`HashSet` in replay-authoritative state (`BTreeMap`/`BTreeSet` only).
- **Deterministic Serialization Rule** — canonical serializer (BLAKE3) with sorted keys, NFC-normalized strings, snapshot-tested against checked-in fixtures.
- **Async Policy** — `apw-server` owns the only Tokio runtime. Library crates are runtime-agnostic and do not start executors.
- **Capability Policy** — every sensitive operation is gated by a `Capability` enum variant. Authority is `BTreeMap<ActorId, BTreeSet<Capability>>`; only `apw-kernel` may mutate the map.
- **Panic & Failure Policy** — the kernel returns `Err`; only `apw-server` may crash on unrecoverable errors. Event append and snapshot are atomic.
- **Event Versioning Policy** — `EventEnvelope::schema_version: u32` on every versioned type. Wire-format stability is contractual (snapshot-tested).
- **Boundary Enforcement** — every crate has a `tests/boundary.rs` enforcing both forbidden dependencies (Layer 1) and forbidden imports in `src/` (Layer 2). M1+ replaces grep with `cargo metadata` graph validation and `clippy::disallowed_types`.

Changes to any of these require an ADR under `docs/adr/`.

## Documentation

- [Features](docs/FEATURES.md) — single source of truth for **what** is being ported from each upstream project, **from where**, **to** which `apw-*` crate, in which milestone, and at what status. The per-feature inventory.
- [Roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md) — M0–M5 milestones, cross-cutting concerns (boundary-check upgrade, Aseprite pipeline, `pixtuoid-core` embedding, self-hosted LLM).
- [Design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md) — authoritative source for crate boundaries, policies, and the `apw-protocol` wire schema.
- [Implementation plan](docs/superpowers/plans/2026-06-06-m0-workspace-skeleton-implementation.md) — phases 0–7 plus verification gates for executing M0.

## License

[MIT](LICENSE) — see [LICENSE](LICENSE).
