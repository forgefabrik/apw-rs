# pixtuoid Refactor Map

This page is the focused source map for the `pixtuoid` row in `docs/FEATURES.md`.

| Source | Features catalogued | adopted | porting | planned | deferred / not-adopted |
|---|---:|---:|---:|---:|---:|
| `pixtuoid` | 14 | 0 | 2 | 12 | 0 |

## What We Reuse

`pixtuoid` is the Rust terminal pixel-art office reference. In `apw-rs`, it is not copied as a standalone app; its ideas are refactored into the Rust workspace boundaries:

| Upstream area | Refactored into | Status |
|---|---|---|
| `crates/` workspace shape | `apw-rs` workspace structure | porting |
| `crates/pixtuoid-hook/src/main.rs` | future `apw-hook` shim | porting |
| `crates/core/src/source.rs` | future `apw_hook::Source` trait | planned |
| `crates/core/src/sources/` | future `apw_hook::sources` adapters | planned |
| `crates/core/src/transport/unix.rs` | future `apw_hook::transport::unix` | planned |
| `crates/core/src/reducer.rs` | future `apw_pixel_plugin::reducer` | planned |
| `crates/core/src/palette.rs` | future `apw_pixel_plugin::palette` | planned |
| `crates/pixtuoid/src/render/` | future `apw_office::render` | planned |
| `crates/pixtuoid/src/ui/office.rs` | future `apw_office::office` | planned |
| `crates/pixtuoid/src/ui/glow.rs` | future `apw_office::glow` | planned |
| `crates/pixtuoid/src/ui/weather.rs` | future `apw_office::weather` | planned |
| `crates/pixtuoid/src/ui/tooltip.rs` | future `apw_office::tooltip` | planned |
| `crates/pixtuoid/src/ui/pets.rs` | future `apw_office::pets` | planned |
| `crates/pixtuoid/src/theme/` | future `apw_office::theme` | planned |

## Feature Entries

The canonical inventory remains in [FEATURES.md](../FEATURES.md#7-features-from-ivanwng97pixtuoid-rust-terminal-pixel-art-office) and `features.registry.json`.
