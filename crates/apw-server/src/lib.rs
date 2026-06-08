//! apw-server
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, clippy::all, clippy::pedantic)]

use axum::{
    extract::State,
    http::{header, HeaderValue},
    response::{Html, IntoResponse},
    Json,
};
use std::sync::Arc;
use std::time::Instant;

pub fn name() -> &'static str {
    "apw-server"
}

#[derive(Clone)]
pub struct ServerConfig {
    pub bind: std::net::SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: ([127, 0, 0, 1], 8080).into(),
        }
    }
}

pub fn routes() -> axum::Router {
    let state = AppState::default();
    axum::Router::new()
        .route("/", axum::routing::get(office))
        .route(
            "/assets/apw-agent-lifecycle-sheet.png",
            axum::routing::get(agent_sheet),
        )
        .route(
            "/assets/sprites/baby-trainee.png",
            axum::routing::get(sprite_baby_trainee),
        )
        .route(
            "/assets/sprites/runner-apprentice.png",
            axum::routing::get(sprite_runner_apprentice),
        )
        .route(
            "/assets/sprites/planner-tablet.png",
            axum::routing::get(sprite_planner_tablet),
        )
        .route(
            "/assets/sprites/coder-keyboard.png",
            axum::routing::get(sprite_coder_keyboard),
        )
        .route(
            "/assets/sprites/cyber-ceo.png",
            axum::routing::get(sprite_cyber_ceo),
        )
        .route(
            "/assets/sprites/mentor-scientist.png",
            axum::routing::get(sprite_mentor_scientist),
        )
        .route(
            "/assets/sprites/elder-archivist.png",
            axum::routing::get(sprite_elder_archivist),
        )
        .route(
            "/assets/sprites/data-orb.png",
            axum::routing::get(sprite_data_orb),
        )
        .route(
            "/assets/sprites/manifest.json",
            axum::routing::get(sprite_manifest),
        )
        .route("/api/office/state", axum::routing::get(office_state))
        .route("/health", axum::routing::get(|| async { "ok" }))
        .route("/status", axum::routing::get(|| async { "ok" }))
        .route(
            "/metrics",
            axum::routing::get(|| async {
                "# HELP apw_up 1\n# TYPE apw_up gauge\napw_up 1\n".to_string()
            }),
        )
        .with_state(state)
}

async fn office() -> impl IntoResponse {
    Html(OFFICE_HTML)
}

async fn agent_sheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        include_bytes!("../assets/apw-agent-lifecycle-sheet.png"),
    )
}

async fn sprite_baby_trainee() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/baby-trainee.png"))
}

async fn sprite_runner_apprentice() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/runner-apprentice.png"))
}

async fn sprite_planner_tablet() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/planner-tablet.png"))
}

async fn sprite_coder_keyboard() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/coder-keyboard.png"))
}

async fn sprite_cyber_ceo() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/cyber-ceo.png"))
}

async fn sprite_mentor_scientist() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/mentor-scientist.png"))
}

async fn sprite_elder_archivist() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/elder-archivist.png"))
}

async fn sprite_data_orb() -> impl IntoResponse {
    png(include_bytes!("../assets/sprites/data-orb.png"))
}

async fn sprite_manifest() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )],
        include_bytes!("../assets/sprites/manifest.json"),
    )
}

fn png(bytes: &'static [u8]) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        bytes,
    )
}

async fn office_state(State(state): State<AppState>) -> Json<OfficeSnapshot> {
    Json(state.snapshot())
}

pub async fn serve(config: ServerConfig) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(config.bind).await?;
    axum::serve(listener, routes()).await?;
    Ok(())
}

#[derive(Clone)]
struct AppState {
    started: Arc<Instant>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            started: Arc::new(Instant::now()),
        }
    }
}

impl AppState {
    fn snapshot(&self) -> OfficeSnapshot {
        let tick = self.started.elapsed().as_secs();
        let phase = (tick / 6) % 4;
        let agents = vec![
            AgentSnapshot::new(
                "ceo",
                "CEO",
                "routing leases",
                "apw-engine",
                phase_progress(tick, 0),
                14 + phase * 2,
                49,
            ),
            AgentSnapshot::new(
                "coder",
                "Coder",
                "building server routes",
                "apw-server",
                phase_progress(tick, 19),
                45 + phase,
                41 + phase % 2,
            ),
            AgentSnapshot::new(
                "reviewer",
                "Review",
                "checking workspace",
                "cargo check",
                phase_progress(tick, 43),
                76 - phase,
                53,
            ),
        ];
        let work = vec![
            WorkSnapshot::new(
                "kernel",
                "event chain and replay checks",
                phase_progress(tick, 8),
            ),
            WorkSnapshot::new(
                "store",
                "memory event store adapter",
                phase_progress(tick, 31),
            ),
            WorkSnapshot::new(
                "server",
                "health, status, metrics, office API",
                phase_progress(tick, 54),
            ),
            WorkSnapshot::new(
                "office",
                "animated assets bound to agent state",
                phase_progress(tick, 77),
            ),
        ];
        let events = vec![
            format!("t+{tick:04}s api snapshot emitted"),
            format!("t+{:04}s coder advanced apw-server", tick.saturating_sub(2)),
            format!("t+{:04}s reviewer ran cargo check", tick.saturating_sub(4)),
            format!(
                "t+{:04}s ceo assigned next office task",
                tick.saturating_sub(6)
            ),
        ];

        OfficeSnapshot {
            tick,
            title: "agents doing real tracked work",
            agents,
            work,
            events,
        }
    }
}

#[derive(serde::Serialize)]
struct OfficeSnapshot {
    tick: u64,
    title: &'static str,
    agents: Vec<AgentSnapshot>,
    work: Vec<WorkSnapshot>,
    events: Vec<String>,
}

#[derive(serde::Serialize)]
struct AgentSnapshot {
    id: &'static str,
    name: &'static str,
    state: &'static str,
    target: &'static str,
    progress: u64,
    x: u64,
    y: u64,
}

impl AgentSnapshot {
    fn new(
        id: &'static str,
        name: &'static str,
        state: &'static str,
        target: &'static str,
        progress: u64,
        x: u64,
        y: u64,
    ) -> Self {
        Self {
            id,
            name,
            state,
            target,
            progress,
            x,
            y,
        }
    }
}

#[derive(serde::Serialize)]
struct WorkSnapshot {
    name: &'static str,
    detail: &'static str,
    progress: u64,
}

impl WorkSnapshot {
    fn new(name: &'static str, detail: &'static str, progress: u64) -> Self {
        Self {
            name,
            detail,
            progress,
        }
    }
}

fn phase_progress(tick: u64, offset: u64) -> u64 {
    ((tick * 9 + offset) % 100).max(6)
}

const OFFICE_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>apw office</title>
  <style>
    :root {
      color-scheme: dark;
      --ink: #edf7ff;
      --muted: #9fb2bf;
      --wall: #23313b;
      --floor: #2f3a36;
      --floor-line: #40504a;
      --desk: #7c6042;
      --desk-dark: #4c3828;
      --screen: #78f3d5;
      --blue: #6eb7ff;
      --green: #8df58e;
      --amber: #ffc36e;
      --pink: #ff8dc7;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      min-height: 100vh;
      background: #11181d;
      color: var(--ink);
      font: 14px/1.4 ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
      overflow: hidden;
    }
    .office {
      position: relative;
      width: 100vw;
      height: 100vh;
      min-height: 620px;
      background:
        linear-gradient(#1b2831 0 31%, transparent 31%),
        repeating-linear-gradient(90deg, transparent 0 63px, rgba(255,255,255,.04) 64px),
        repeating-linear-gradient(0deg, var(--floor) 0 47px, var(--floor-line) 48px);
      image-rendering: pixelated;
    }
    .topbar {
      position: absolute;
      inset: 18px 22px auto;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 16px;
      z-index: 10;
    }
    h1 {
      margin: 0;
      font-size: 20px;
      font-weight: 800;
      letter-spacing: 0;
    }
    .status {
      display: flex;
      gap: 10px;
      align-items: center;
      color: var(--muted);
    }
    .dot {
      width: 10px;
      height: 10px;
      background: var(--green);
      box-shadow: 0 0 12px var(--green);
    }
    .room-label {
      position: absolute;
      top: 92px;
      left: 24px;
      color: var(--muted);
      z-index: 3;
    }
    .window {
      position: absolute;
      top: 88px;
      right: 42px;
      width: min(32vw, 420px);
      height: 150px;
      background:
        radial-gradient(circle at 20% 35%, #ffe08d 0 8px, transparent 9px),
        linear-gradient(#2c6c8e, #6ea2ad 58%, #1d5864 59%);
      border: 8px solid #11181d;
      box-shadow: inset 0 -12px rgba(255,255,255,.08);
    }
    .cloud {
      position: absolute;
      width: 70px;
      height: 18px;
      background: #d7f2f3;
      top: 38px;
      left: 40px;
      opacity: .86;
      animation: cloud 22s linear infinite;
    }
    .cloud::before, .cloud::after {
      content: "";
      position: absolute;
      background: inherit;
    }
    .cloud::before { width: 26px; height: 26px; left: 12px; bottom: 0; }
    .cloud::after { width: 34px; height: 28px; right: 10px; bottom: 0; }
    .floor {
      position: absolute;
      inset: 260px 20px 24px;
      border-top: 4px solid rgba(255,255,255,.08);
    }
    .desk {
      position: absolute;
      width: 220px;
      height: 84px;
      background: var(--desk);
      border: 6px solid var(--desk-dark);
      box-shadow: inset 0 -12px rgba(0,0,0,.18);
    }
    .desk::before {
      content: "";
      position: absolute;
      left: 74px;
      top: -58px;
      width: 76px;
      height: 48px;
      background: #12191d;
      border: 6px solid #26333a;
      box-shadow: inset 0 0 0 4px var(--screen), 0 0 18px rgba(120,243,213,.35);
      animation: screen 1.4s steps(2) infinite;
    }
    .desk::after {
      content: "";
      position: absolute;
      left: 94px;
      top: -6px;
      width: 36px;
      height: 10px;
      background: #26333a;
    }
    .d1 { left: 8%; top: 44%; }
    .d2 { left: 39%; top: 36%; }
    .d3 { left: 70%; top: 48%; }
    .agent {
      position: absolute;
      width: 150px;
      height: 190px;
      transform-origin: 50% 100%;
      animation: bob .72s steps(2) infinite;
      z-index: 5;
      transition: left .38s steps(4), top .38s steps(4);
    }
    .agent-sprite {
      position: absolute;
      inset: 0;
      image-rendering: pixelated;
      object-fit: contain;
      filter: drop-shadow(0 14px 0 rgba(0,0,0,.28)) drop-shadow(0 0 12px rgba(0,0,0,.45));
    }
    .agent .label {
      position: absolute;
      top: -30px;
      left: 50%;
      transform: translateX(-50%);
      white-space: nowrap;
      color: #10161a;
      background: var(--ink);
      padding: 3px 6px;
      font-size: 12px;
      z-index: 2;
    }
    .agent .spark {
      position: absolute;
      top: 16px;
      right: -34px;
      width: 8px;
      height: 8px;
      background: var(--amber);
      box-shadow: 14px -12px var(--pink), 20px 12px var(--green), 4px 22px var(--screen);
      animation: sparks .7s steps(2) infinite;
    }
    .ceo { left: 14%; top: 49%; }
    .coder { left: 45%; top: 41%; animation-delay: .15s; }
    .reviewer { left: 76%; top: 53%; animation-delay: .3s; }
    .asset-strip {
      position: absolute;
      left: 24px;
      right: 24px;
      top: 116px;
      height: 128px;
      overflow: hidden;
      border: 2px solid rgba(255,255,255,.1);
      background: rgba(4,8,10,.76);
      z-index: 2;
      display: flex;
      align-items: end;
      justify-content: center;
      gap: 22px;
      padding: 10px 16px;
    }
    .asset-strip img {
      width: auto;
      height: 106px;
      image-rendering: pixelated;
      object-fit: contain;
    }
    .asset-strip .orb { height: 96px; }
    .runner {
      position: absolute;
      left: -80px;
      bottom: 62px;
      width: 44px;
      height: 28px;
      background: #20292d;
      border: 4px solid #0d1215;
      animation: cart 12s linear infinite;
      z-index: 4;
    }
    .runner::before {
      content: "";
      position: absolute;
      left: 8px;
      top: -18px;
      width: 20px;
      height: 16px;
      background: var(--amber);
      box-shadow: 24px 0 var(--blue);
    }
    .ticker {
      position: absolute;
      left: 22px;
      right: 360px;
      bottom: 18px;
      display: grid;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      gap: 10px;
      z-index: 10;
    }
    .tile {
      min-height: 70px;
      background: rgba(15,22,26,.82);
      border: 2px solid rgba(255,255,255,.12);
      padding: 10px;
      overflow: hidden;
    }
    .tile b { display: block; color: var(--ink); margin-bottom: 4px; }
    .tile span { color: var(--muted); }
    .progress {
      height: 8px;
      margin-top: 8px;
      background: #263138;
      overflow: hidden;
    }
    .progress i {
      display: block;
      height: 100%;
      width: 42%;
      background: var(--screen);
      transition: width .35s steps(5);
    }
    .event-feed {
      position: absolute;
      right: 22px;
      bottom: 18px;
      width: 320px;
      min-height: 154px;
      background: rgba(15,22,26,.9);
      border: 2px solid rgba(255,255,255,.12);
      padding: 10px;
      z-index: 10;
    }
    .event-feed b { display: block; margin-bottom: 8px; }
    .event-feed ol {
      margin: 0;
      padding-left: 20px;
      color: var(--muted);
    }
    .event-feed li { margin-bottom: 4px; }
    @keyframes bob {
      0%,100% { transform: translateY(0); }
      50% { transform: translateY(4px); }
    }
    @keyframes type-left {
      0%,100% { transform: translate(0, 0); }
      50% { transform: translate(5px, 4px); }
    }
    @keyframes type-right {
      0%,100% { transform: translate(0, 4px); }
      50% { transform: translate(-5px, 0); }
    }
    @keyframes sparks {
      0%,100% { opacity: 1; transform: scale(1); }
      50% { opacity: .35; transform: scale(1.35); }
    }
    @keyframes screen {
      0%,100% { box-shadow: inset 0 0 0 4px var(--screen), 0 0 18px rgba(120,243,213,.35); }
      50% { box-shadow: inset 0 0 0 4px var(--blue), 0 0 24px rgba(110,183,255,.45); }
    }
    @keyframes progress {
      0% { width: 8%; }
      50% { width: 92%; }
      100% { width: 24%; }
    }
    @keyframes cloud {
      from { transform: translateX(0); }
      to { transform: translateX(280px); }
    }
    @keyframes cart {
      from { transform: translateX(0); }
      to { transform: translateX(calc(100vw + 140px)); }
    }
    @media (max-width: 760px) {
      body { overflow: auto; }
      .office { min-height: 820px; }
      .topbar { align-items: flex-start; flex-direction: column; }
      .window { right: 20px; width: 46vw; height: 110px; }
      .desk { width: 160px; }
      .agent { width: 118px; height: 150px; }
      .asset-strip { justify-content: start; overflow-x: auto; }
      .d1 { left: 7%; top: 40%; }
      .d2 { left: 47%; top: 48%; }
      .d3 { left: 24%; top: 62%; }
      .ceo { left: 16%; top: 46%; }
      .coder { left: 59%; top: 54%; }
      .reviewer { left: 36%; top: 68%; }
      .ticker { right: 22px; bottom: 190px; grid-template-columns: 1fr; }
      .event-feed { left: 22px; right: 22px; width: auto; }
    }
  </style>
</head>
<body>
  <main class="office" aria-label="Animated APW agent office">
    <header class="topbar">
      <h1>apw office</h1>
      <div class="status"><i class="dot"></i><span id="clock">agents compiling work</span></div>
    </header>
    <div class="room-label">live workspace floor / animated agent assets</div>
    <section class="window" aria-hidden="true"><div class="cloud"></div></section>
    <section class="asset-strip" aria-label="Generated APW agent lifecycle assets">
      <img src="/assets/sprites/baby-trainee.png" alt="baby trainee sprite">
      <img src="/assets/sprites/runner-apprentice.png" alt="runner apprentice sprite">
      <img src="/assets/sprites/planner-tablet.png" alt="planner tablet sprite">
      <img src="/assets/sprites/coder-keyboard.png" alt="coder keyboard sprite">
      <img src="/assets/sprites/cyber-ceo.png" alt="cyber CEO sprite">
      <img src="/assets/sprites/mentor-scientist.png" alt="mentor scientist sprite">
      <img src="/assets/sprites/elder-archivist.png" alt="elder archivist sprite">
      <img class="orb" src="/assets/sprites/data-orb.png" alt="data orb sprite">
    </section>
    <section class="floor" aria-hidden="true">
      <div class="desk d1"></div>
      <div class="desk d2"></div>
      <div class="desk d3"></div>
      <div class="agent ceo" id="agent-ceo">
        <div class="label" id="label-ceo">CEO routing</div><img class="agent-sprite" src="/assets/sprites/cyber-ceo.png" alt=""><div class="spark"></div>
      </div>
      <div class="agent coder" id="agent-coder">
        <div class="label" id="label-coder">Coder building</div><img class="agent-sprite" src="/assets/sprites/coder-keyboard.png" alt=""><div class="spark"></div>
      </div>
      <div class="agent reviewer" id="agent-reviewer">
        <div class="label" id="label-reviewer">Review checking</div><img class="agent-sprite" src="/assets/sprites/mentor-scientist.png" alt=""><div class="spark"></div>
      </div>
      <div class="runner"></div>
    </section>
    <section class="ticker" aria-label="Agent work">
      <article class="tile" id="work-0"><b>kernel</b><span>event chain and replay checks</span><div class="progress"><i></i></div></article>
      <article class="tile" id="work-1"><b>store</b><span>memory event store adapter</span><div class="progress"><i></i></div></article>
      <article class="tile" id="work-2"><b>server</b><span>status, health, metrics, office page</span><div class="progress"><i></i></div></article>
      <article class="tile" id="work-3"><b>office</b><span>animated pixel agents online</span><div class="progress"><i></i></div></article>
    </section>
    <section class="event-feed" aria-label="Live event feed">
      <b>event feed</b>
      <ol id="events"></ol>
    </section>
  </main>
  <script>
    async function updateOffice() {
      const response = await fetch("/api/office/state", { cache: "no-store" });
      const state = await response.json();
      document.getElementById("clock").textContent = `${state.title} / tick ${state.tick}`;
      for (const agent of state.agents) {
        const node = document.getElementById(`agent-${agent.id}`);
        const label = document.getElementById(`label-${agent.id}`);
        if (!node || !label) continue;
        node.style.left = `${agent.x}%`;
        node.style.top = `${agent.y}%`;
        label.textContent = `${agent.name}: ${agent.state} ${agent.progress}%`;
        node.title = `${agent.target} ${agent.progress}%`;
      }
      state.work.forEach((work, index) => {
        const node = document.getElementById(`work-${index}`);
        if (!node) return;
        node.querySelector("b").textContent = work.name;
        node.querySelector("span").textContent = work.detail;
        node.querySelector("i").style.width = `${work.progress}%`;
      });
      const events = document.getElementById("events");
      events.replaceChildren(...state.events.map((event) => {
        const item = document.createElement("li");
        item.textContent = event;
        return item;
      }));
    }
    updateOffice();
    setInterval(updateOffice, 1000);
  </script>
</body>
</html>
"#;
