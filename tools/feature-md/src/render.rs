//! Render features.registry.json to a Markdown document that matches the
//! structure of docs/FEATURES.md (sections per source, scope split, phase 2
//! narrative).

use std::collections::BTreeMap;
use std::fmt::Write;

use feature_schema::{Feature, Registry, Source};

const HEADER: &str = r#"# FEATURES

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

"#;

const SCOPE_SPLIT: &str = r#"## 7. Scope split: Phase 1 (Visual Office Foundation + Living Office Simulation) vs Phase 2 (Economy & Multi-Role + Life-Cycle Roadmap)

`apw-rs` is built in two clearly separated phases. The **visual office and the living simulation** are the foundation; everything in Phase 2 lands on top of it.

| | **Phase 1 — Visual Office + Life-Sim Foundation** (in scope, current build) | **Phase 2 — Economy, Multi-Role & Life-Cycle** (roadmap, after Phase 1) |
|---|---|---|
| **What it is** | The pixel-art office surface — agents as characters, projects as rooms, per-project towers, living-room personalization, day/night cycle, shift scheduler, supermarket/pool/bar facilities, social graph | The economic engine + multi-role orchestration + full lifecycle (Kindergarten→School→University→Work→Retirement→Archive) that drives the office |
| **When it ships** | After the M0 (workspace skeleton) + M4 (office TUI + pixel pipeline) milestones, with Life-Sim features layered in | Roadmap. Not part of any current M-numbered milestone. |
| **Crate surface used** | `apw-protocol` (wire types), `apw-pixel-plugin` (sprite/character data), `apw-office` (TUI renderer), `apw-gateway` (web control plane) | Adds `apw-kernel` (state machine, authority spine of the CEO), `apw-engine` (LLM, agents, scheduler, life-sim, life-cycle), `apw-store` (ledger + persistence), `apw-server` (HTTP + SSE), `apw-life-sim`, `apw-life-cycle` |
| **Where the data comes from** | A simple local JSON / sample data adapter that exercises the office surfaces | The kernel event log + LM Studio LLM with TypeScript-plugin tool surface |
| **In this file** | §3–§6 (sources), §9 (cross-cutting) | **§8 (this document)** |

The protocol types in `apw-protocol/src/lib.rs` (Role, AuthorityMap, Capability, Event variants like `AgentPromoted`, `ItemPurchased`, `CapabilityDenied`, `TimeTick`, `AgentStateTransition`) are **forward-compatible placeholders**: they exist so Phase 1 can render the *state* of Phase 2 systems without Phase 2 existing. The implementations come in Phase 2.

"#;

const PHASE2_INTRO: &str = r#"## 8. Phase 2 — Economy, Multi-Role & Life-Cycle (Roadmap)

> **Status: deferred (Phase 2).** Not part of the current build. This section is the architectural seed: it documents the bigger vision and the protocol-level primitives that the office already anticipates, so that the Phase 1 surface has stable shape and the Phase 2 build can land without breaking it.
>
> The full Phase 2 design — including the LM Studio plugin bridge, the customer→CEO→worker flow, the tower-climb/economy mechanics, the day/night simulation loop, and the full life-cycle — will be written as a separate design spec **after** the Phase 1 office is finished. That spec is the terminal deliverable mentioned in the [roadmap](docs/superpowers/2026-06-05-apw-rs-roadmap.md).

### 8.1 The bigger vision (one paragraph)

A customer posts a job to a project tower. The CEO — an LLM running on LM Studio via the TypeScript plugin SDK — sees the new job, decomposes it, and either spawns worker agents (each an LLM with its own tools) or assigns existing ones. Workers climb the project's tower as they deliver value (merged PRs, passing tests, milestones); higher floors unlock better office infrastructure. Workers earn money for their work and spend it in their personal living rooms. The CEO earns money from project revenue and spends it on new rooms, faster PCs (more capable models), or office perks. A day/night cycle (Morning→WorkBlock→Lunch→Afternoon→AfterWork→Night) drives automatic events: shift assignment, supermarket runs, pool recovery, bar social, training. Agents are not NPCs — they are born in Kindergarten, grow in School, specialize in University, work on Tower Economy, retire to the RetirementHome, and are compressed into the KnowledgeGraph at the Crematorium. Each generation is better or more specialized than the last. Visual office state is a **compiled projection of the simulation state**, never the trigger. Idle state = no revenue = no growth = office decay. The game loop is a closed economic + lifecycle system, not a cosmetic animation.

"#;

const HOW_TO_USE: &str = r#"## 10. How to use this file

- **Adding a feature?** Edit `features.registry.json` directly. Add a row with `id`, `name`, `phase`, `milestone`, `status`, and at least one of `from`/`to`. Then run `cargo run -p feature-md -- render` to regenerate this file, and `cargo run -p feature-guardian -- check` to validate. The commit that lands the change must include both the registry diff and the regenerated `FEATURES.md`.
- **Porting a feature?** Change `status` from `planned (Mx)` to `porting (Mx)` when the work starts, and to `done` when the feature ships. The commit that lands the change updates the registry in the same diff.
- **Dropping a feature?** Move the row to the *Not adopted* subsection of its source (use `status: "not-adopted"`), with a one-line reason in `notes`. Never silently delete.
- **Renaming a target crate?** Grep `features.registry.json` for the old crate name and update in the same commit as the rename. Every feature with a `to.crate` field should reference a real workspace member (or a documented Phase-2 crate).
- **Reviewer?** If a PR adds a new public surface in `apw-*` crates, it should also have added a row to the registry and regenerated this file. Reject the PR if not.
- **CI:** `feature-guardian check --strict` runs in GitHub Actions. The build fails if any check errors or warnings appear.
- **Visualizing dependencies:** `cargo run -p feature-graph -- show --milestone Mx` renders the dependency DAG. `cargo run -p feature-graph -- critical-path --milestone Mx` shows the longest chain. `cargo run -p feature-graph -- blocks --milestone Mx` answers "what blocks Mx?".
- **Auto-ingesting upstream:** `cargo run -p feature-harvester -- scan <owner/repo>` emits a JSON of feature candidates that can be diffed against the registry.

The `apw-rs` repo at any commit should be able to answer "where does this feature come from, where is it going, and what's its status?" from `features.registry.json` alone. This Markdown file is a generated view of that single source of truth.

"#;

pub fn to_markdown(r: &Registry) -> String {
    let mut out = String::new();
    out.push_str(HEADER);
    write_sources(&mut out, r);
    write_summary(&mut out, r);
    write_phase1_sections(&mut out, r);
    out.push_str(SCOPE_SPLIT);
    out.push_str(PHASE2_INTRO);
    write_phase2_primitives(&mut out, r);
    out.push_str(PHASE2_LIFECYCLE);
    write_phase2_milestones(&mut out, r);
    out.push_str(PHASE2_KCE0);
    out.push_str(PHASE2_ANTIPATTERNS);
    write_phase2_features(&mut out, r);
    write_cross_cutting(&mut out, r);
    out.push_str(HOW_TO_USE);
    out
}

fn write_sources(out: &mut String, r: &Registry) {
    out.push_str("## 1. Source projects (upstream references)\n\n");
    out.push_str("| # | id | repo | license | role |\n");
    out.push_str("|---|---|---|---|---|\n");
    for (i, s) in r.sources.iter().enumerate() {
        let n = i + 1;
        let url = s.url.clone().unwrap_or_default();
        let role = s.role.clone().unwrap_or_default();
        let license = s.license.clone().unwrap_or_default();
        let _ = writeln!(
            out,
            "| {n} | `{}` | [{}]({}) | {} | {} |",
            s.id, s.repo, url, license, role
        );
    }
    out.push_str(
        "\nEvery feature below cites its upstream file/endpoint in the *From* column.\n\n---\n\n",
    );
}

fn write_summary(out: &mut String, r: &Registry) {
    out.push_str("## 2. Summary by source\n\n");
    out.push_str(
        "| Source | Features catalogued | adopted | porting | planned | deferred / not-adopted |\n",
    );
    out.push_str("|---|---:|---:|---:|---:|---:|\n");
    let mut by_source: BTreeMap<&str, Vec<&Feature>> = BTreeMap::new();
    for f in &r.features {
        let key = f.source_id.as_deref().unwrap_or("apw-rs");
        by_source.entry(key).or_default().push(f);
    }
    let total_features = r.features.len();
    let mut adopted = 0;
    let mut porting = 0;
    let mut deferred = 0;
    for (_, feats) in &by_source {
        for f in feats {
            match f.status.as_str() {
                "done" => adopted += 1,
                "porting" => porting += 1,
                "deferred" | "not-adopted" => deferred += 1,
                _ => {}
            }
        }
    }
    let planned = total_features - adopted - porting - deferred;
    for (sid, feats) in &by_source {
        let n = feats.len();
        let p = feats.iter().filter(|f| f.status == "porting").count();
        let pl = feats.iter().filter(|f| f.status == "planned").count();
        let d = feats
            .iter()
            .filter(|f| f.status == "deferred" || f.status == "not-adopted")
            .count();
        let done = feats.iter().filter(|f| f.status == "done").count();
        let source = source_summary_label(sid);
        let _ = writeln!(out, "| {source} | {n} | {done} | {p} | {pl} | {d} |");
    }
    let _ = writeln!(
        out,
        "| **Total** | **{total_features}** | **{adopted}** | **{porting}** | **{planned}** | **{deferred}** |"
    );
    out.push_str("\n---\n\n");
}

fn source_summary_label(source_id: &str) -> String {
    match source_id {
        "pixtuoid" => "[`pixtuoid`](sources/pixtuoid.md)".to_string(),
        _ => format!("`{source_id}`"),
    }
}

fn write_phase1_sections(out: &mut String, r: &Registry) {
    let mut by_source: BTreeMap<&str, (&Source, Vec<&Feature>)> = BTreeMap::new();
    let sources = r.source_index();
    for f in &r.features {
        if f.phase != "phase-1" {
            continue;
        }
        let key = f.source_id.as_deref().unwrap_or("apw-rs");
        let src = sources.get(key).copied();
        let entry = by_source.entry(key).or_insert((src.unwrap(), Vec::new()));
        entry.1.push(f);
    }
    let section_titles: &[(&str, &str, &str)] = &[
        (
            "3",
            "Features from `agent-bigbrother` (v0.2a)",
            "3.1 Kernel (event-sourced, replay-authoritative)",
        ),
        (
            "4",
            "Features from `agent-bigbrother` — WebUI",
            "4.1 WebUI (10-tab control grid)",
        ),
        (
            "5",
            "Features from `forgefabrik/forgefabrik-agent-os` (architecture spec)",
            "5.1 Architecture spec",
        ),
        (
            "6",
            "Features from `chriswritescode-dev/opencode-manager` (mobile-first PWA)",
            "6.1 Adopted features",
        ),
        (
            "7",
            "Features from `IvanWng97/pixtuoid` (Rust terminal pixel-art office)",
            "7.1 Pixel-art office primitives",
        ),
    ];
    for (i, (key, (src, feats))) in by_source.iter().enumerate() {
        let section_num = section_titles.get(i).map(|(n, _, _)| *n).unwrap_or("X");
        let title: String = section_titles
            .get(i)
            .map(|(_, t, _)| (*t).to_string())
            .unwrap_or_else(|| format!("Features from `{}`", key));
        let _ = writeln!(out, "## {section_num}. {title}\n");
        let role = src.role.clone().unwrap_or_default();
        if !role.is_empty() {
            let _ = writeln!(out, "{role}\n");
        }
        write_feature_table(out, feats);
    }
    // Source-specific not-adopted sections
    let not_adopted: Vec<&Feature> = r
        .features
        .iter()
        .filter(|f| f.phase == "phase-1" && f.status == "not-adopted")
        .collect();
    if !not_adopted.is_empty() {
        out.push_str("### Phase-1 not-adopted features\n\n");
        out.push_str("| id | name | source | notes |\n|---|---|---|---|\n");
        for f in not_adopted {
            let notes = f.notes.clone().unwrap_or_default();
            let src = f.source_id.clone().unwrap_or_default();
            let _ = writeln!(out, "| `{}` | {} | `{}` | {} |", f.id, f.name, src, notes);
        }
        out.push_str("\n");
    }
    out.push_str("---\n\n");
}

fn write_feature_table(out: &mut String, feats: &[&Feature]) {
    if feats.is_empty() {
        return;
    }
    out.push_str("| # | Feature | From (upstream) | To (apw-rs) | Milestone | Status | Tags |\n");
    out.push_str("|---|---|---|---|---|---|---|\n");
    let mut sorted: Vec<&Feature> = feats.iter().copied().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for f in sorted {
        let from_str = f
            .from
            .as_ref()
            .and_then(|x| x.path.clone())
            .map(|p| format!("`{}`", p))
            .unwrap_or_else(|| "—".to_string());
        let to_str = match &f.to {
            Some(t) => match (&t.krate, &t.module) {
                (Some(c), Some(m)) => format!("`{c}` ({m})"),
                (Some(c), None) => format!("`{c}`"),
                _ => "—".to_string(),
            },
            None => "—".to_string(),
        };
        let tags = if f.tags.is_empty() {
            "—".to_string()
        } else {
            format!("`{}`", f.tags.join("`, `"))
        };
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} | {} | {} |",
            f.id, f.name, from_str, to_str, f.milestone, f.status, tags
        );
    }
    out.push_str("\n");
}

fn write_phase2_primitives(out: &mut String, _r: &Registry) {
    out.push_str("### 8.2 Architectural primitives (forward-compat types)\n\n");
    out.push_str(
        "These are the Rust types/traits that the office surface will reference in Phase 1 (as enum variants or trait method signatures, often no-op), and that the Phase 2 kernel/engine/life-cycle will implement.\n\n",
    );
    out.push_str("| # | Primitive | Purpose | Phase 1 status |\n|---|---|---|---|\n");
    let primitives: &[(&str, &str, &str, &str)] = &[
        ("8.2.1", "Role::Ceo, Role::Worker, Role::Customer (already in apw_protocol::Role)", "Distinguishes actors in the multi-role system. CEO is the orchestrator; workers execute; customers post jobs.", "Defined (Role enum has Ceo + 8 others; Worker/Customer to be added)"),
        ("8.2.2", "AuthorityMap = BTreeMap<ActorId, BTreeSet<Capability>>", "Per-actor permission set. CEO has broad capabilities; workers are restricted.", "Defined (type alias exists; population rules in Phase 2)"),
        ("8.2.3", "Capability::PromoteAgent, AllocateLease, SubmitSpriteProposal, ModifyAuthorityMap, RunTowerAdmin, ReplayChain", "Typed capability enum (typo-resistant). Capabilities gate the LLM's tool surface.", "Defined (enum exists; checked at runtime in Phase 2)"),
        ("8.2.4", "BiddingEngine trait", "Customer posts a job → workers/CEO submit bids → winner is chosen by rule → lease is allocated.", "Stub (trait signature only, no impl)"),
        ("8.2.5", "TrustVerifier trait", "Verifies a claimed contribution is genuine (CI result, test result, peer review).", "Stub (trait signature only, no impl)"),
        ("8.2.6", "Wallet, Item, Catalog", "Per-actor money balance with append-only ledger. Buyable personal items for living rooms.", "Not defined (to be added in Phase 2)"),
        ("8.2.7", "Upgrade (Room | Pc | Perk)", "Buyable office upgrades for the CEO: new rooms, faster PCs, perks.", "Not defined (to be added in Phase 2)"),
        ("8.2.8", "Tower, Floor, ClimbEvent", "Per-project tower; each floor is an unlock tier. Height is a function of revenue ÷ floor_cost.", "Not defined (pixel-plugin will render)"),
        ("8.2.9", "LivingRoom, owned_items, layout_grid", "Per-agent personal space. Rendered as a small room adjacent to the work area.", "Not defined (Phase 2)"),
        ("8.2.10", "MoneyEvent ledger entries", "Append-only ledger: ContributionMerged, ProjectCompleted, CiPassed, ItemPurchased, OfficeDecay.", "Not defined (Phase 2)"),
        ("8.2.11", "TeamRole (CEO, CODER, DESIGNER, TESTER) + fixed topology", "Job-family axis. Fixed: 3 CODER + 3 DESIGNER + 1-3 TESTER + 1 CEO. Coexists with authority Role.", "Not defined (new in Phase 2)"),
        ("8.2.12", "WorldClock, DayPhase, automatic day events", "Two-layer time: Tick (kernel) is atomic; WorldClock is the derived projection with day phases. Drives automatic shift assignment, shop open hours, etc.", "Not defined (kernel emits TimeTick; engine projects DayPhase)"),
        ("8.2.13", "AgentState (Working, Idle, Eating, Training, Socializing, Recovering)", "Idle is NOT a non-state — it's an active state where the agent trains, socializes, recovers. No AFK idling.", "Not defined (idle becomes an active routing target)"),
        ("8.2.14", "OfficeStats (productivity, stress, social_cohesion, creativity, fatigue)", "Per-team metrics that couple back into task success, errors, burnout.", "Not defined (engine couples metrics → outcomes)"),
        ("8.2.15", "Supermarket, Pool, Bar facilities", "Resource system (items + buffs), recovery engine (stamina/stress), social graph engine (bonding, idea spawns).", "Not defined (life-sim crate)"),
        ("8.2.16", "LifeStage (Infant, Child, Student, JuniorWorker, SeniorWorker, Expert, Mentor, Retired, Archived)", "The lifecycle state machine. 9 states with deterministic transitions.", "Not defined (life-cycle crate)"),
        ("8.2.17", "Kindergarten, School, University, RetirementHome, Crematorium", "Lifecycle buildings. New agents are born in Kindergarten, retire to RetirementHome, are compressed into the KnowledgeGraph at the Crematorium.", "Not defined (life-cycle crate)"),
        ("8.2.18", "MemoryPacket (skills, experiences, decisions, confidence_weight)", "A bundle of knowledge extracted from an agent. Transferred through School→University→Work→Retirement→Archive.", "Not defined (life-cycle crate)"),
        ("8.2.19", "KnowledgeGraph (nodes: MemoryPacket[], edges: KnowledgeRelation[])", "The collective memory. Agents learn not only individually but from collective past.", "Not defined (life-cycle crate)"),
        ("8.2.20", "Generation (inherited_skills, innovation_factor)", "Each new agent generation is better or more specialized. innovation_factor represents the new skills/patterns NOT inherited.", "Not defined (life-cycle crate)"),
        ("8.2.21", "LM Studio CeoBridge plugin adapter", "The CEO is an LLM running on LM Studio via @lmstudio/sdk. The plugin exposes kernel operations as tool() definitions gated by AuthorityMap.", "Not defined (engine crate)"),
    ];
    for (id, prim, purpose, status) in primitives {
        let _ = writeln!(out, "| {id} | {prim} | {purpose} | {status} |");
    }
    out.push_str("\n");
    let _ = writeln!(
        out,
        "Full feature entries for these primitives: see §8.5 below.\n\n"
    );
}

const PHASE2_LIFECYCLE: &str = r#"### 8.3 Full life loop (Kindergarten → ... → KnowledgeGraph)

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

"#;

fn write_phase2_milestones(out: &mut String, _r: &Registry) {
    out.push_str("### 8.4 Phase 2 milestones (preliminary, not committed)\n\n");
    out.push_str("The roadmap will be updated when the Phase 2 spec is written. A likely shape, subject to revision:\n\n");
    out.push_str("- **P2.0** — Protocol extension: add `TeamRole`, `WorldClock`, `DayPhase`, `OfficeStats`, `AgentState`, `LifeStage`, `MemoryPacket`, `KnowledgeGraph`, `Generation`, plus all facility types, as forward-compat types in `apw-protocol` and the new `apw-life-sim` / `apw-life-cycle` crates.\n");
    out.push_str("- **P2.1** — Engine: implement revenue computation, MoneyEvent emission, Wallet ledger, OfficeStats coupling.\n");
    out.push_str(
        "- **P2.2** — Engine: implement `BiddingEngine` (post-job → bid → resolve → lease).\n",
    );
    out.push_str("- **P2.3** — Kernel: implement `TrustVerifier` (verify contribution, update trust score).\n");
    out.push_str("- **P2.4** — Engine: implement `ClimbEvent` emission, `Tower` height computation, `WorldClock` projection from `Tick`.\n");
    out.push_str("- **P2.5** — `apw-engine::CeoBridge` — the LM Studio plugin adapter. The CEO's tool surface is the set of kernel operations gated by `AuthorityMap`.\n");
    out.push_str(
        "- **P2.6** — `apw-server` — `CustomerIntake` HTTP surface, SSE for live state.\n",
    );
    out.push_str("- **P2.7** — `apw-pixel-plugin` — render Tower, LivingRoom, Supermarket, Pool, Bar, Lifecycle buildings from the simulation state.\n");
    out.push_str("- **P2.8** — `apw-life-cycle` — full lifecycle loop: Kindergarten → School → University → Work → Retirement → Crematorium → KnowledgeGraph.\n");
    out.push_str("- **P2.9** — Generation drift: each new agent generation reflects the cumulative knowledge graph.\n");
    out.push_str("- **P2.10** — Office decay loop, end-to-end anti-AFK rule.\n\n");
    out.push_str("These are **pre-decision notes**, not a commitment. The actual Phase 2 spec is the terminal deliverable.\n\n");
}

const PHASE2_KCE0: &str = r#"### 8.5 Kernel = CEO (architectural principle)

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

"#;

const PHASE2_ANTIPATTERNS: &str = r#"### 8.6 Anti-patterns the design avoids

- **Cosmetic-only XP bars.** The tower is not an animation; it is a derived state of `Wallet` revenue vs `Floor.cost`. Without revenue, no climb.
- **Idle state earns money.** No revenue from agent turn, thinking, or idle animation. Revenue only from merged contribution, CI-passed release, project completion, or customer delivery.
- **AFK-wealthy state.** Office decay rule ensures no-output companies stagnate.
- **LLM-as-authority.** The LLM emits `Intent` types; the kernel turns them into authoritative events. The LLM never writes to the event chain directly. (See [design spec §Determinism Policy](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md).)
- **Stringly-typed roles.** `Role` is an enum, not a freeform string. `TeamRole` is a separate enum for the job-family axis. Capabilities are typed, not strings. Authority is a `BTreeMap`, not a hash.
- **Hard delete.** No `delete agent`. Every agent leaving the active pool emits a memory packet to the KnowledgeGraph. Death is compression, not deletion.
- **Static agents.** Agents evolve through Kindergarten → School → University → Work → Retirement → Archive. Generation drift makes each cohort measurably different.
- **No-feedback simulation.** Metrics (stress, social, fatigue) couple back into task success. A team with no pool burns out; a team with no food crashes; a team with no social creativity stalls.

"#;

fn write_phase2_features(out: &mut String, r: &Registry) {
    let phase2: Vec<&Feature> = r.features.iter().filter(|f| f.phase == "phase-2").collect();
    if phase2.is_empty() {
        return;
    }
    out.push_str("### 8.7 Phase 2 features (registry entries)\n\n");
    out.push_str("| id | name | milestone | status | tags |\n|---|---|---|---|---|\n");
    let mut sorted = phase2;
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for f in sorted {
        let tags = if f.tags.is_empty() {
            "—".to_string()
        } else {
            format!("`{}`", f.tags.join("`, `"))
        };
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} |",
            f.id, f.name, f.milestone, f.status, tags
        );
    }
    out.push_str("\n---\n\n");
}

fn write_cross_cutting(out: &mut String, r: &Registry) {
    let apw: Vec<&Feature> = r
        .features
        .iter()
        .filter(|f| f.source_id.as_deref() == Some("apw-rs") && f.phase == "phase-1")
        .collect();
    if apw.is_empty() {
        return;
    }
    out.push_str("## 9. Cross-cutting (introduced by `apw-rs` itself)\n\n");
    out.push_str("These are not from any single upstream; they are new in `apw-rs` to satisfy governance policies locked in the [design spec](docs/superpowers/specs/2026-06-05-apw-rs-workspace-skeleton-design.md).\n\n");
    out.push_str(
        "| # | Feature | Driver | Crate | Milestone | Status |\n|---|---|---|---|---|---|\n",
    );
    let mut sorted = apw;
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for f in sorted {
        let driver = f
            .evidence
            .as_ref()
            .and_then(|e| e.rationale.clone())
            .unwrap_or_default();
        let to_str =
            f.to.as_ref()
                .map(|t| t.krate.clone().unwrap_or_default())
                .unwrap_or_default();
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | `{}` | {} | {} |",
            f.id, f.name, driver, to_str, f.milestone, f.status
        );
    }
    out.push_str("\n");
}
