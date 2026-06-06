# apw-rs M0: Execution Summary & Verification Gates

**Date:** 2026-06-06  
**Assessment:** Independent technical review  
**Status:** ✅ Ready for implementation (with empirical verification required)  
**Risk Level:** Low (M0 skeleton work, assuming corrections applied exactly as described)  

---

## Independent Review Assessment

### Reviewer Notes

> I have not inspected the actual corrected document, repository, or codebase—only the summarized changes.
>
> The ultimate validation remains empirical: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, and clean-checkout verification on a fresh clone.
>
> CI passing on both Linux and macOS is the final gate, not the plan itself.
>
> Given the stated fixes, the plan is now structured, testable, and auditable enough to serve as the baseline for M0 execution and subsequent M1 kernel-port work.

### Classification

| Aspect | Rating |
|--------|--------|
| **Plan Structure** | ✅ Ready for implementation |
| **Risk Level** | 🟢 Low (M0 skeleton, corrections applied exactly) |
| **Testability** | ✅ Fully auditable |
| **Governance** | ✅ Serves as baseline for M1+ |
| **Critical Success Factor** | Empirical validation (see gates below) |

---

## M0 Completion Gates (All Must Pass)

M0 is **NOT** complete until ALL gates pass in sequence.

### Gate 1: Developer Local Verification

**Executor:** Developer implementing Phase 0-7  
**Command:**
```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

**Success Criteria:**
- ✅ Build: All 10 crates compile, no errors
- ✅ Test: 15+ tests pass, 0 failures
- ✅ Clippy: 0 warnings with `-D warnings` flag
- ✅ Fmt: All files formatted, no violations

**If Gate 1 fails:** Debug locally, fix, re-run Gate 1. Do NOT proceed.

**Gate 1 Status:** `[ ] PASS` (to be filled during execution)

---

### Gate 2: Fresh-Clone Verification

**Executor:** Same developer OR separate reviewer  
**Prerequisites:**
- Gate 1 passed
- All Phase 0-8 commits merged to main
- m0-skeleton-v1.0.0 tag created locally

**Procedure:** Execute clean-checkout test script exactly as documented in plan:

```bash
# Use a FRESH environment (not existing apw-rs clone)
cd /tmp
rm -rf apw-rs-test-checkout
mkdir apw-rs-test-checkout
cd apw-rs-test-checkout

# Clone from main branch (simulates fresh start)
git clone https://github.com/forgefabrik/apw-rs.git
cd apw-rs

# Verify on m0-skeleton commit
git checkout m0-skeleton-v1.0.0

# Run full sequence
rustup override set 1.82
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

**Success Criteria:**
- ✅ Fresh clone pulls latest code
- ✅ Build: All 10 crates compile
- ✅ Test: 15+ tests pass
- ✅ Clippy: 0 warnings
- ✅ Fmt: All files formatted
- ✅ No cached dependencies interfere
- ✅ No local toolchain differences
- ✅ No unstaged files

**Why this matters:** Catches issues hidden by local developer environments.

**If Gate 2 fails:** 
- Identify root cause (cached deps? toolchain? unstaged files?)
- Fix in main branch
- Re-run Gate 2 from scratch
- Do NOT proceed to Gate 3

**Gate 2 Status:** `[ ] PASS` (to be filled during execution)

---

### Gate 3: GitHub Actions CI

**Executor:** GitHub Actions (automated)  
**Trigger:** Push m0-skeleton-v1.0.0 tag to origin

**Matrix:**
- ubuntu-latest: build, test, clippy, fmt
- macos-latest: build, test, clippy, fmt

**Success Criteria:**
- ✅ Ubuntu job: All 4 steps pass
- ✅ macOS job: All 4 steps pass
- ✅ No flaky tests across runs
- ✅ No platform-specific failures

**CI is the final gate.** GitHub Actions passing is the proof that M0 is production-ready.

**If Gate 3 fails:**
- Review GitHub Actions logs
- Identify platform-specific issue
- Fix in main branch
- Re-run CI
- Do NOT declare M0 complete

**Gate 3 Status:** `[ ] PASS` (to be verified on GitHub)

---

### Gate 4: Documentation Verification

**Executor:** Tech lead or reviewer  
**Checklist:**

- [ ] `docs/ROADMAP.md` populated (not placeholder)
- [ ] `docs/EVENTS.md` populated (event schema complete)
- [ ] `docs/PIXEL.md` populated (M4 reference valid)
- [ ] `docs/adr/0000-template.md` exists (ready for future ADRs)
- [ ] `docs/adr/0001-panic-policy-specification.md` exists (deferred to M1)
- [ ] Root `README.md` has correct crate count (9 + 1)
- [ ] `docs/PRODUCTION_READINESS.md` complete with all 7 blockers resolved
- [ ] `docs/DEPLOYMENT.md` runbook complete

**If Gate 4 fails:** Add/fix missing documentation, re-run Gate 4.

**Gate 4 Status:** `[ ] PASS`

---

### Gate 5: Crate Structure Verification

**Executor:** Tech lead  
**Checklist:**

```bash
# Verify workspace structure
cargo metadata --format-version 1 | jq '.workspace_members | length'
# Expected: 10

# Verify internal path dependencies
grep -c "\.workspace = true" crates/*/Cargo.toml tools/*/Cargo.toml
# Expected: Total matches = correct dependency usage

# Verify forbid(unsafe_code) in all crates
grep -r "#!\[forbid(unsafe_code)\]" crates/ tools/
# Expected: 10 matches (9 lib + 1 binary)

# Verify no server deps in client crates
for crate in crates/apw-office crates/apw-manager crates/apw-gateway crates/apw-pixel-plugin; do
  grep "apw-server\|apw-kernel\|apw-engine\|apw-store" "$crate/Cargo.toml" && echo "FAIL: $crate has server dep" || true
done
# Expected: No output (all clean)
```

**If Gate 5 fails:** Verify crate structure is exactly as documented.

**Gate 5 Status:** `[ ] PASS`

---

### Gate 6: Tag Verification

**Executor:** Developer  
**Checklist:**

```bash
# Tag exists locally
git tag -l m0-skeleton-v1.0.0
# Expected: m0-skeleton-v1.0.0

# Tag points to correct commit
git show m0-skeleton-v1.0.0 | head -10
# Expected: Commit hash from Phase 7

# Tag is annotated (not lightweight)
git cat-file -t m0-skeleton-v1.0.0
# Expected: tag
```

**If Gate 6 fails:** Verify tag creation command and re-run.

**Gate 6 Status:** `[ ] PASS`

---

## M0 Completion Checklist

All gates must pass before M0 is declared complete.

```
Pre-Execution
[ ] Review final plan: 2026-06-06-M0-FINAL-READY.md
[ ] Confirm all 7 blockers documented as fixed
[ ] Confirm risk assessment: LOW (M0 skeleton)
[ ] Confirm correction list matches plan

Execution (Phases 0-9)
[ ] Phase 0: Bootstrap — committed
[ ] Phase 1: apw-protocol — committed
[ ] Phase 2: Server crates — committed
[ ] Phase 3: Client crates — committed
[ ] Phase 4: CLI tool — committed
[ ] Phase 5: CI/CD + docs — committed
[ ] Phase 6: Smoke tests — committed
[ ] Phase 7: Final verification — committed
[ ] Phase 8: Production readiness — committed
[ ] Phase 9: Deployment prep — committed

Gate 1: Developer Local Verification
[ ] cargo build --workspace ✅
[ ] cargo test --workspace ✅
[ ] cargo clippy ... -D warnings ✅
[ ] cargo fmt --check ✅
[ ] Result: PASS

Gate 2: Fresh-Clone Verification
[ ] /tmp/apw-rs-test-checkout created
[ ] git clone from main ✅
[ ] rustup override set 1.82 ✅
[ ] cargo build --workspace ✅
[ ] cargo test --workspace ✅
[ ] cargo clippy ... -D warnings ✅
[ ] cargo fmt --check ✅
[ ] Result: PASS

Gate 3: GitHub Actions CI
[ ] m0-skeleton-v1.0.0 tag pushed
[ ] GitHub Actions triggered
[ ] ubuntu-latest: build ✅
[ ] ubuntu-latest: test ✅
[ ] ubuntu-latest: clippy ✅
[ ] ubuntu-latest: fmt ✅
[ ] macos-latest: build ✅
[ ] macos-latest: test ✅
[ ] macos-latest: clippy ✅
[ ] macos-latest: fmt ✅
[ ] Result: PASS

Gate 4: Documentation Verification
[ ] ROADMAP.md populated
[ ] EVENTS.md populated
[ ] PIXEL.md populated
[ ] ADR template ready
[ ] Panic policy ADR created
[ ] README.md has correct crate count
[ ] PRODUCTION_READINESS.md complete
[ ] DEPLOYMENT.md runbook complete
[ ] Result: PASS

Gate 5: Crate Structure Verification
[ ] 10 workspace members confirmed
[ ] All internal deps use workspace refs
[ ] forbid(unsafe_code) in all 10 crates
[ ] No server deps in client crates
[ ] Result: PASS

Gate 6: Tag Verification
[ ] Tag m0-skeleton-v1.0.0 exists
[ ] Tag points to correct commit
[ ] Tag is annotated (not lightweight)
[ ] Result: PASS

Final Declaration
[ ] All 6 gates PASS
[ ] M0 skeleton is production-ready
[ ] Ready for M1 kernel core port planning
[ ] Declaration date: ___________
[ ] Approved by: ___________
```

---

## Important Caveats

### 1. Plan vs. Reality

> I have not inspected the actual corrected document, repository, or codebase—only the summarized changes.

**Implication:** The plan is sound, but empirical execution is the true test. Code changes might have unexpected interactions.

**Mitigation:** Gate 2 (fresh-clone) catches these issues.

### 2. Empirical Validation Required

> The ultimate validation remains empirical: `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`.

**Implication:** No amount of plan review replaces actually running the code.

**Mitigation:** Execute all gates in sequence; do NOT skip.

### 3. CI is the Final Gate

> CI passing on both Linux and macOS is the final gate, not the plan itself.

**Implication:** If GitHub Actions fails, M0 is not complete, even if local tests pass.

**Reason:** Local developer environments can mask issues that only appear in CI environments (different Rust versions, different package managers, different cache states).

**Mitigation:** Gate 3 is mandatory.

### 4. Corrections Applied Exactly

> Risk level: Low for M0 skeleton work, **assuming the corrections were applied exactly as described**.

**Implication:** If corrections deviate from the documented changes, risk increases.

**Example deviations that would fail gates:**
- Workspace deps declared but not all used
- Some crates still have unused imports
- forbid(unsafe_code) added to some crates but not all
- CI matrix configuration incomplete

**Mitigation:** Gate 5 (crate structure verification) validates this.

---

## Decision Points

### Decision Point 1: Should We Execute M0?

**Yes if:**
- ✅ All 7 blockers documented as fixed
- ✅ Corrections match plan exactly
- ✅ Risk assessment accepted (Low for skeleton work)
- ✅ Empirical validation planned

**No if:**
- ❌ Blockers not fully documented
- ❌ Corrections not detailed
- ❌ Risk tolerance unknown

### Decision Point 2: Is Gate 1 Passing Sufficient?

**No.** Gate 1 (local verification) is necessary but not sufficient.

**Why:** Local environments can have:
- Cached dependencies from previous builds
- Unstaged files not in git
- Different toolchain versions
- Different OS/package manager behavior

**Gate 2 (fresh-clone) is required** to validate Gate 1 results.

### Decision Point 3: Can We Proceed Without GitHub Actions?

**No.** GitHub Actions CI is the final gate.

**Why:**
- CI environment is immutable and reproducible
- If CI fails, M0 fails (regardless of local tests)
- CI failure indicates platform-specific issues
- CI passing is proof of production-readiness

**All three gates (1, 2, 3) must pass.**

### Decision Point 4: What if Gate 3 Fails?

**Action:** Do NOT declare M0 complete.

**Process:**
1. Review GitHub Actions logs
2. Identify failure (build, test, clippy, or fmt)
3. Fix root cause in main branch
4. Push fix
5. Re-trigger CI
6. Re-run Gate 3 from scratch
7. Only when Gate 3 passes, proceed

**Do NOT patch around failures.** Investigate and fix root cause.

---

## Timeline & Effort Estimate

| Phase | Estimate | Actual |
|-------|----------|--------|
| Phases 0-7 (implementation) | 3-4 hours | `[ ]` |
| Gate 1 (local verification) | 15 min | `[ ]` |
| Gate 2 (fresh-clone) | 15 min | `[ ]` |
| Gate 3 (GitHub Actions) | 5-10 min (automated) | `[ ]` |
| Gates 4-6 (documentation + verification) | 30 min | `[ ]` |
| **Total M0 execution** | **4-5 hours** | `[ ]` |

**Can be parallelized:**
- Multiple developers on different phases (if coordinated)
- CI runs automatically (no manual intervention)
- Expected total: 2-3 hours with 2 developers

---

## M0 Definition (From Review)

M0 is complete when:

1. ✅ 10 workspace members (9 library + 1 binary) compile cleanly
2. ✅ 15+ test sections pass (boundary + smoke + unit)
3. ✅ Clippy reports 0 warnings with `-D warnings`
4. ✅ Fmt reports 0 violations
5. ✅ Fresh clone from main passes all above
6. ✅ GitHub Actions passes on ubuntu-latest AND macos-latest
7. ✅ All documentation is complete (no placeholders)
8. ✅ Crate structure verified (no server deps in clients, forbid(unsafe_code) everywhere)
9. ✅ Tag m0-skeleton-v1.0.0 created and pushed
10. ✅ This checklist is 100% complete

When all 10 conditions are met: **M0 skeleton is production-ready.**

---

## Approval & Sign-Off

### Technical Reviewer Approval

**Assessment:** Plan is sound and implementable.

**Approval:** ✅ YES — Ready for execution (with gates)

**Caveats:** See "Important Caveats" section (plan review ≠ code review ≠ CI approval)

**Next action:** Execute Phase 0, run all gates in sequence.

---

### Developer Sign-Off (To be filled during execution)

**Developer name:** `___________`  
**Date started:** `___________`  
**Date completed:** `___________`  
**All gates passed:** `[ ] YES / [ ] NO`  
**Issues encountered:** `___________`  
**M0 declared complete:** `[ ] YES / [ ] NO`  

---

### Tech Lead Approval (Final)

**Tech lead name:** `___________`  
**Gate 1 reviewed:** `[ ] YES`  
**Gate 2 reviewed:** `[ ] YES`  
**Gate 3 reviewed:** `[ ] YES`  
**Gates 4-6 reviewed:** `[ ] YES`  
**M0 approved for M1 planning:** `[ ] YES`  
**Date:** `___________`  

---

## What Happens After M0 is Complete

Once all gates pass and M0 is approved:

1. ✅ Tag `m0-skeleton-v1.0.0` is merged to main
2. ✅ GitHub Release created with notes
3. ✅ M1 specification phase begins
4. ✅ M1 implementation planning (kernel core port)
5. ✅ M1 development begins (event-core, algebra, freezer, trust, replay, snapshot)

**M0 is the foundation.** Everything else builds on top.

---

## Conclusion

The M0 implementation plan has moved from design proposal to **plausibly executable specification**. 

**Status:** ✅ Ready for implementation (with empirical validation gates)

**Risk:** 🟢 Low (M0 skeleton work, corrections applied exactly)

**Critical success factors:**
1. All 7 blockers fixed exactly as documented
2. All gates (1-6) pass in sequence
3. No skipping gates or shortcuts
4. Fresh-clone verification catches local environment issues
5. GitHub Actions validates cross-platform compatibility

**Recommended next action:** Execute clean-checkout verification procedure before tagging `m0-skeleton`.

---

**Document prepared:** 2026-06-06  
**Assessment valid until:** M0 execution begins  
**Once M0 execution starts, update this document with actual gate results.**

