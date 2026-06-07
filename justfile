set shell := ["bash", "-o", "pipefail", "-c"]

default:
    @just --list

fmt:
    cargo fmt --all
fmt-check:
    cargo fmt --all -- --check
clippy-fix:
    cargo clippy --workspace --fix --allow-dirty --allow-staged

check:
    cargo check --workspace --all-targets
build:
    cargo build --workspace --all-targets
test:
    cargo test --workspace
test-quiet:
    cargo test --workspace -q

clean:
    cargo clean

deny:
    cargo deny check
audit:
    cargo audit

lint: fmt-check clippy test deny audit
verify: check lint fmt-check
ci-smoke: check lint

# Guided local QA
pythonization-guard:
    @echo "Stub"

.PHONY: default fmt fmt-check clippy-fix check build test test-quiet clean deny audit lint verify ci-smoke
