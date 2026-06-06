# Archived M0 Planning Documents

These documents were used in the planning and design phases. They show the evolution from initial specification through independent technical review, blocker identification, and blocker fixes.

**The current M0 specification is documented in:**

1. ✅ `docs/superpowers/plans/2026-06-06-M0-FINAL-READY.md` — **Phases 0-9 (use this for implementation)**
2. ✅ `docs/superpowers/plans/2026-06-06-M0-EXECUTION-GATES.md` — **Gates 1-6 (use this for verification)**
3. ✅ `docs/superpowers/plans/2026-06-06-M0-EXECUTION-AUTHORITY.md` — **Final authority (current phase)**

## Planning Evolution

### Stage 1: Initial Specification
- **Document:** `2026-06-04-forgefabrik-hq-enhancement.md`
- **Status:** Starting point — ForgeFabrik HQ enhancement plan (Node.js v0.2a)
- **Purpose:** Original requirements gathering
- **Outcome:** Identified need for Rust workspace skeleton (M0)
- **Replaced by:** M0 workspace skeleton specification

### Stage 2: First Implementation Plan
- **Document:** `2026-06-06-m0-workspace-skeleton-implementation.md`
- **Status:** Initial detailed plan
- **Scope:** 9 phases, all tasks specified
- **Outcome:** ~80% complete; 7 blockers identified in tech review
- **Replaced by:** Corrected version with blocker fixes

### Stage 3: Comprehensive Production Plan
- **Document:** `2026-06-06-M0-COMPREHENSIVE-PRODUCTION-PLAN.md`
- **Status:** Added production-readiness phase
- **Scope:** Phases 0-9 + Phase 8 (production readiness)
- **Outcome:** ~95% complete; all blockers listed but not yet fixed
- **Replaced by:** Final corrected version

### Stage 4: Corrected Final Plan
- **Document:** `2026-06-06-M0-COMPREHENSIVE-PRODUCTION-PLAN-FINAL.md`
- **Status:** All 7 blockers addressed
- **Scope:** Phases 0-9, all corrections applied, clean-checkout test included
- **Outcome:** ~99% complete; ready for review
- **Replaced by:** Execution authority (with 7 mandatory gates)

## Key Changes Between Stages

### Blocker 1: Workspace Dependencies
- **Initial:** Internal crates referenced with `.workspace = true` but not declared
- **Fixed:** Root `Cargo.toml` now declares all 5 internal + 10 external deps

### Blocker 2: Recursive Smoke Tests
- **Initial:** Tests invoked `cargo build --workspace` inside test code
- **Fixed:** Replaced with lightweight linking tests (no recursion)

### Blocker 3: Clippy Violations
- **Initial:** Unused imports in skeleton code
- **Fixed:** All unused imports removed, only necessary imports kept

### Blocker 4: Crate Count Inconsistency
- **Initial:** "8 crates + 1 CLI" vs actual 10 members
- **Fixed:** Consistent terminology: "9 library crates + 1 binary" or "10 workspace members"

### Blocker 5: Test Organization
- **Initial:** Inline `#[test]` at module level
- **Fixed:** All use `#[cfg(test)] mod tests {}`

### Blocker 6: Unused Imports Globally
- **Initial:** Scattered unused imports across crates
- **Fixed:** Complete audit pass

### Blocker 7: Dead Documentation
- **Initial:** `docs/ROADMAP.md` referenced but empty
- **Fixed:** All documentation actually populated

## Independent Technical Review Findings

**Status:** Ready for implementation  
**Risk:** Low (M0 skeleton work)  
**Caveats:** 
- Review did NOT inspect actual code/repo (only summaries)
- Empirical validation required (all 7 gates must pass)
- CI passing on Linux + macOS is final proof

**Recommendation:** Execute clean-checkout verification + boundary test failure demonstration (Gate 7) before tagging M0.

## Transition to Execution

Once **all 7 gates pass** and **all required artifacts collected**, M0 is declared complete and ready for M1 planning.

**Artifacts required:**
1. Successful CI run URL
2. Fresh-clone verification log
3. `cargo test --workspace` output
4. `cargo clippy --workspace --all-targets -- -D warnings` output
5. `cargo fmt --check` output
6. Git tag `m0-skeleton-v1.0.0`
7. M0 release note

---

**Archive created:** 2026-06-06  
**Planning phase:** Complete  
**Current phase:** Execution & empirical verification
