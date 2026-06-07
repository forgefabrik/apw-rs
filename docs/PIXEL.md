# Pixel-Art Visualisierung — Pixtuoid-Integration

## Übersicht

apw-rs visualisiert Agenten als Pixel-Art-Charaktere in einem Terminal-Büro.
Die Integration baut auf [pixtuoid](https://github.com/IvanWng97/pixtuoid) auf.

| Crate | Zweck | apw-rs-Nutzung |
|---|---|---|
| pixtuoid-core | Headless Lib: Sources, State, Reducer, Sprites, Layout, Physics, Pose | apw-office embedded als Lib |
| pixtuoid | Binary: CLI, Config, Runtime, TUI-Renderer | Referenz für apw-office TUI |
| pixtuoid-hook | Agent-Shim: Unix-Socket, 200ms Write-Timeout | Nicht genutzt |

## Render-Pipeline (geplant M4)

```
apw-kernel ──Event-Stream──▶ apw-office ──▶ Pixtuoid-Core ──▶ TUI/Canvas
                                    │
                              apw-pixel-plugin
                              (Aseprite Parser)
```

### Track B (Web) — Parallel zu M2–M5

```
apw-server ──SSE/WS──▶ apw-gateway ──▶ Dioxus WASM-App
                              │
                    Pixtuoid-Core (WASM)
                    Pixel → HTML5 Canvas
```

## Wichtige Invarianten (aus pixtuoid)

1. pixtuoid-core hat keine Terminal-Abhängigkeiten — kein ratatui, kein crossterm
2. Events fließen durch genau EINEN Channel
3. Walkable Mask = Ground Footprint nur
4. Layout-Generierung ist deterministisch via Seed
5. Event-State → Pose-Reduction muss deterministisch sein

Siehe docs/superpowers/specs/* für die vollständige Design-Spec.

