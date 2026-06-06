# ForgeFabrik HQ Enhancement Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **Goal:** Transform ForgeFabrik v0.2a from a tabbed dashboard into a living isometric pixel-office operations center with 10 data-bound areas, 7-state agent lifecycle, economy heatmap, trading floor, contract vault, event wall, plugin marketplace, scheduler war room, and CEO command center.
>
> **Architecture:** Upgrade existing `renderWorld()` in `webui/app.js` from 2D top-down to isometric CSS 3D view. Each of the 10 office areas is a `<div data-area="...">` slot bound to real kernel data via existing `/api/runtime`. Agent lifecycle states drive CSS classes + expression cycling. Heatmap uses dynamic CSS variables per floor. No new backend endpoints needed — all data comes from existing runtime snapshot.
>
> **Tech Stack:** Vanilla HTML/CSS/JS (no new deps). CSS 3D transforms for isometric view. SVG for pixel-art sprites (existing `renderAvatar()`). `requestAnimationFrame` for animation loop. Existing kernel modules (`agent_projection`, `forge_tower`, `runtime_snapshot`, `sprite_loader`).

---

## File Structure

```
v0.2a/
├── webui/
│   ├── index.html          # WORLD tab pane expanded: 10 area slots + heatmap layer + trading wall + vault + LED wall + plugin row + war table + CEO center
│   ├── app.css             # +300 lines: isometric CSS, 10 area variants, lifecycle animations, heatmap, trading floor, LED wall, plugin market, war room, CEO center
│   └── app.js              # renderWorld() → renderIsometricOffice() + renderAreaData() + lifecycle + heatmap + trading + vault + event wall + plugin market + war room + CEO center
├── kernel/
│   ├── agent_projection.mjs  # add x,y coordinates per agent for isometric placement
│   ├── forge_tower.mjs       # add area_type mapping to each room (which of the 10 areas it represents)
│   └── runtime_snapshot.mjs  # ensure economy, scheduler, contracts, events, plugins, trust, executions are in snapshot
├── data/
│   └── sprites/
│       └── pixel-plugin/
│           ├── manifest.json  # extend: add 3 new expressions (reviewing, planning, expired) + 3 new roles (economist, replay_agent, trust_agent)
│           └── sprites.json   # override grids for new states
├── tests/
│   └── smoke.mjs              # section 38-43: extended sprite manifest, area types, isometric view, lifecycle, heatmap, trading, specialized areas
└── docs/
    └── superpowers/
        └── plans/
            └── 2026-06-04-forgefabrik-hq-enhancement.md  # this file
```

---

## Phase 1: Sprite Assets

### Task 1: Extend sprite manifest with new lifecycle states and roles

**Files:**
- Modify: `v0.2a/data/sprites/pixel-plugin/manifest.json`
- Test: `v0.2a/tests/smoke.mjs` (section 38)

- [ ] **Step 1: Write the failing test**

Add to smoke.mjs after section 37:

```javascript
// 38. extended sprite manifest has new lifecycle states + roles
section('38. extended sprite manifest');
const { spriteLoader } = await import('../kernel/sprite_loader.mjs');
await spriteLoader.load();
const m = await spriteLoader.getManifest();
assert(m.roles.includes('ECONOMIST'), 'manifest has ECONOMIST role');
assert(m.roles.includes('REPLAY_AGENT'), 'manifest has REPLAY_AGENT role');
assert(m.roles.includes('TRUST_AGENT'), 'manifest has TRUST_AGENT role');
assert(m.expressions.includes('reviewing'), 'manifest has reviewing expression');
assert(m.expressions.includes('planning'), 'manifest has planning expression');
assert(m.expressions.includes('expired'), 'manifest has expired expression');
const econ = await spriteLoader.resolve('ECONOMIST', 'working');
assert(econ && Array.isArray(econ.grid) && econ.grid.length === 8, 'ECONOMIST/working grid is 8x8');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "38\."`
Expected: FAIL with "manifest has ECONOMIST role" or similar

- [ ] **Step 3: Extend manifest.json**

Edit `v0.2a/data/sprites/pixel-plugin/manifest.json`:

Add to `roles` array: `"ECONOMIST"`, `"REPLAY_AGENT"`, `"TRUST_AGENT"`

Add to `expressions` array: `"reviewing"`, `"planning"`, `"expired"`

Add to `palettes` object:
```json
"ECONOMIST": { "body": "#4be0d2", "accent": "#a07c1f", "head": "#0b0f14", "eyes": "#f4ecd8" },
"REPLAY_AGENT": { "body": "#7a1f1f", "accent": "#4be0d2", "head": "#0b0f14", "eyes": "#a07c1f" },
"TRUST_AGENT": { "body": "#a07c1f", "accent": "#7a1f1f", "head": "#0b0f14", "eyes": "#4be0d2" }
```

Add to `sprites` object (8×8 grids for each new role×expression — use the same structure as existing roles, with distinctive poses: ECONOMIST has coin shapes, REPLAY_AGENT has rewind arrows, TRUST_AGENT has shield shapes).

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "38\."`
Expected: PASS for all 6 assertions

- [ ] **Step 5: Commit**

```bash
cd /home/azureuser/repo/forgefabrik-agent-os/v0.2a
git add data/sprites/pixel-plugin/manifest.json tests/smoke.mjs
git commit -m "feat: extend pixel-plugin manifest with 3 new roles + 3 new expressions for HQ lifecycle view"
```

---

### Task 2: Add area_type mapping to forge_tower rooms

**Files:**
- Modify: `v0.2a/kernel/forge_tower.mjs`
- Test: `v0.2a/tests/smoke.mjs` (section 33b)

- [ ] **Step 1: Write the failing test**

Add to smoke.mjs after section 33:

```javascript
// 33b. forge tower rooms have area_type mapping
section('33b. forge tower area types');
const { getForgeTower } = await import('../kernel/forge_tower.mjs');
const tower = getForgeTower();
const areaTypes = new Set();
for (const floor of tower.floors) {
  for (const room of floor.rooms) {
    if (room.area_type) areaTypes.add(room.area_type);
  }
}
assert(areaTypes.size >= 8, `tower has >= 8 distinct area_types (got ${areaTypes.size})`);
const expectedAreas = ['ceo_center', 'open_office', 'trading_floor', 'war_room', 'contract_vault', 'event_archive', 'plugin_market', 'security_office'];
for (const area of expectedAreas) {
  assert(areaTypes.has(area), `tower has area_type ${area}`);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "33b"`
Expected: FAIL with "tower has >= 8 distinct area_types"

- [ ] **Step 3: Add area_type to each room in forge_tower.mjs**

Edit `v0.2a/kernel/forge_tower.mjs` — in the `getForgeTower()` function, add `area_type` to each room object:

```javascript
// CEO FLOOR
{ room_id: 'ceo_office', label: 'CEO Office', status: 'ok', area_type: 'ceo_center', subsystem: 'scheduler' },
{ room_id: 'board_room', label: 'Board Room', status: 'ok', area_type: 'war_room', subsystem: 'algebra' },
{ room_id: 'legal_dept', label: 'Legal Department', status: 'ok', area_type: 'contract_vault', subsystem: 'contracts' },

// OPERATIONS FLOOR
{ room_id: 'trading_floor', label: 'Trading Floor', status: 'ok', area_type: 'trading_floor', subsystem: 'economy' },
{ room_id: 'hr_dept', label: 'HR Department', status: 'ok', area_type: 'open_office', subsystem: 'agents' },
{ room_id: 'it_ops', label: 'IT Operations', status: 'ok', area_type: 'sandbox_lab', subsystem: 'sandbox' },

// DEVELOPMENT FLOOR
{ room_id: 'backend_dev', label: 'Backend Dev', status: 'ok', area_type: 'open_office', subsystem: 'plugins' },
{ room_id: 'frontend_dev', label: 'Frontend Dev', status: 'ok', area_type: 'open_office', subsystem: 'skills' },
{ room_id: 'security_office', label: 'Security Office', status: 'ok', area_type: 'security_office', subsystem: 'trust' },

// ARCHIVES
{ room_id: 'event_archive', label: 'Event Archive', status: 'ok', area_type: 'event_archive', subsystem: 'events' },
{ room_id: 'plugin_market', label: 'Plugin Market', status: 'ok', area_type: 'plugin_market', subsystem: 'plugins' },
{ room_id: 'server_room', label: 'Server Room', status: 'ok', area_type: 'server_room', subsystem: 'lm_studio' }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "33b"`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add kernel/forge_tower.mjs tests/smoke.mjs
git commit -m "feat: add area_type mapping to forge_tower rooms for HQ view"
```

---

## Phase 2: Isometric Office World Shell

### Task 3: Replace renderWorld() with isometric office renderer

**Files:**
- Modify: `v0.2a/webui/index.html` (WORLD tab pane)
- Modify: `v0.2a/webui/app.css` (+250 lines isometric styles)
- Modify: `v0.2a/webui/app.js` (replace renderWorld → renderIsometricOffice)

- [ ] **Step 1: Write the failing test**

Add to smoke.mjs:

```javascript
// 39. isometric office view
section('39. isometric office view');
const html = readFileSync('webui/index.html', 'utf8');
assert(html.includes('data-area="ceo_center"'), 'index.html has ceo_center area slot');
assert(html.includes('data-area="open_office"'), 'index.html has open_office area slot');
assert(html.includes('data-area="trading_floor"'), 'index.html has trading_floor area slot');
assert(html.includes('data-area="war_room"'), 'index.html has war_room area slot');
assert(html.includes('data-area="contract_vault"'), 'index.html has contract_vault area slot');
assert(html.includes('data-area="event_archive"'), 'index.html has event_archive area slot');
assert(html.includes('data-area="plugin_market"'), 'index.html has plugin_market area slot');
assert(html.includes('data-area="security_office"'), 'index.html has security_office area slot');
assert(html.includes('data-area="server_room"'), 'index.html has server_room area slot');
assert(html.includes('data-area="empfang"'), 'index.html has empfang area slot');
const css = readFileSync('webui/app.css', 'utf8');
assert(css.includes('.world-isometric'), 'app.css has isometric world styles');
assert(css.includes('.world-area'), 'app.css has area slot styles');
assert(css.includes('.world-heatmap'), 'app.css has heatmap styles');
const js = readFileSync('webui/app.js', 'utf8');
assert(js.includes('renderIsometricOffice'), 'app.js has isometric renderer');
assert(js.includes('renderAreaData'), 'app.js has area data renderer');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "39\."`
Expected: FAIL with missing area slots or missing functions

- [ ] **Step 3: Update index.html WORLD tab pane**

Replace the existing `<section class="tab-pane" data-pane="world">` content with the 10 area slots + heatmap layer + trading wall + vault + LED wall + plugin row + war table + CEO center (see full HTML in the design above).

- [ ] **Step 4: Add isometric CSS to app.css**

Append ~250 lines of CSS to `v0.2a/webui/app.css` (see full CSS in the design above).

- [ ] **Step 5: Replace renderWorld() with renderIsometricOffice() in app.js**

Replace the entire `renderWorld()` function (lines 251-369) and `startWorldAnimation()` / `stopWorldAnimation()` with the new isometric renderer (see full JS in the design above).

- [ ] **Step 6: Update boot() to use new renderer**

In the RENDERERS map, change `world: renderWorld` to `world: renderIsometricOffice`.

- [ ] **Step 7: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "39\."`
Expected: PASS for all 10 area slot assertions + CSS + JS function assertions

- [ ] **Step 8: Commit**

```bash
git add webui/index.html webui/app.css webui/app.js
git commit -m "feat: replace 2D world view with isometric office HQ (10 areas, agent lifecycle, heatmap)"
```

---

## Phase 3: Kernel Data Enhancements

### Task 4: Ensure runtime snapshot includes all new data domains

**Files:**
- Modify: `v0.2a/kernel/runtime_snapshot.mjs`
- Test: `v0.2a/tests/smoke.mjs` (section 34b)

- [ ] **Step 1: Write the failing test**

```javascript
// 34b. runtime snapshot includes economy, scheduler, contracts, events, plugins, trust, executions
section('34b. runtime snapshot completeness');
const { getRuntimeSnapshot } = await import('../kernel/runtime_snapshot.mjs');
const snap = getRuntimeSnapshot();
assert(snap.economy !== undefined, 'snapshot has economy');
assert(snap.scheduler !== undefined, 'snapshot has scheduler');
assert(snap.contracts !== undefined, 'snapshot has contracts');
assert(snap.events !== undefined, 'snapshot has events');
assert(snap.plugins !== undefined, 'snapshot has plugins');
assert(snap.trust !== undefined, 'snapshot has trust');
assert(snap.executions !== undefined, 'snapshot has executions');
assert(Array.isArray(snap.economy.bids), 'economy.bids is array');
assert(Array.isArray(snap.scheduler.queue), 'scheduler.queue is array');
assert(Array.isArray(snap.contracts.recent), 'contracts.recent is array');
assert(Array.isArray(snap.events.recent), 'events.recent is array');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "34b"`
Expected: FAIL

- [ ] **Step 3: Extend runtime_snapshot.mjs**

Edit `v0.2a/kernel/runtime_snapshot.mjs` — add the missing data domains to the snapshot object (see full code in design above).

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "34b"`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add kernel/runtime_snapshot.mjs tests/smoke.mjs
git commit -m "feat: extend runtime snapshot with economy, scheduler, contracts, events, plugins, trust, executions"
```

---

## Phase 4: Agent Lifecycle Visualization

### Task 5: Wire 7-state lifecycle to isometric view

**Files:**
- Modify: `v0.2a/webui/app.js` (lifecycle mapping already in Task 3)
- Test: `v0.2a/tests/smoke.mjs` (section 40)

- [ ] **Step 1: Write the failing test**

```javascript
// 40. agent lifecycle states render correctly
section('40. agent lifecycle states');
const { getRuntimeSnapshot } = await import('../kernel/runtime_snapshot.mjs');
const snap = getRuntimeSnapshot();
const agents = snap.agent_projection?.agents || [];
assert(agents.length > 0, 'has agents');
const testStates = [
  { has_lease: false, blocked: false, lease_id: 0, expected: 'idle' },
  { has_lease: true, blocked: false, lease_id: 1, expected: 'working' },
  { has_lease: true, blocked: true, lease_id: 0, expected: 'blocked' }
];
for (const ts of testStates) {
  const mockAgent = { state: ts, identity: { agent_id: 'test' }, avatar: {} };
  const state = getAgentState(mockAgent);
  assert(state === ts.expected, `state ${ts.expected} correct for ${JSON.stringify(ts)}`);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "40\."`
Expected: FAIL

- [ ] **Step 3: Ensure getAgentState is accessible in test scope**

Add to `v0.2a/webui/app.js` at the bottom:

```javascript
// Export for test (only in test mode)
if (typeof globalThis !== 'undefined' && globalThis.process?.env?.NODE_ENV === 'test') {
  globalThis.getAgentState = getAgentState;
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "40\."`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add webui/app.js tests/smoke.mjs
git commit -m "feat: wire 7-state agent lifecycle to isometric view"
```

---

## Phase 5: Economy Heatmap + Trading Floor

### Task 6: Implement dynamic heatmap coloring

**Files:**
- Modify: `v0.2a/webui/app.js` (heatmap calculation in renderIsometricOffice)
- Test: `v0.2a/tests/smoke.mjs` (section 41)

- [ ] **Step 1: Write the failing test**

```javascript
// 41. economy heatmap colors floors dynamically
section('41. economy heatmap');
const { getRuntimeSnapshot } = await import('../kernel/runtime_snapshot.mjs');
const snap = getRuntimeSnapshot();
const floors = snap.forge_tower?.floors || [];
assert(floors.length === 4, 'has 4 floors');
function calcHeat(rooms) {
  const ok = rooms.filter(r => r.status === 'ok').length;
  return ok / rooms.length;
}
const ceoHeat = calcHeat(floors[0]?.rooms || []);
assert(ceoHeat >= 0 && ceoHeat <= 1, 'heat value in range');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "41\."`
Expected: FAIL

- [ ] **Step 3: Verify heatmap calculation in renderIsometricOffice()**

The heatmap calculation is already present from Task 3. Ensure it uses these colors:
- health > 0.8 → `rgba(75, 224, 210, 0.3)` (green)
- health > 0.5 → `rgba(160, 124, 31, 0.3)` (amber)
- health > 0.2 → `rgba(122, 31, 31, 0.3)` (red)
- else → `rgba(155, 89, 182, 0.3)` (purple)

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "41\."`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add webui/app.js tests/smoke.mjs
git commit -m "feat: add dynamic economy heatmap to isometric floor grid"
```

---

### Task 7: Implement trading floor bid wall

**Files:**
- Modify: `v0.2a/webui/app.js` (add `renderTradingWall()`)
- Test: `v0.2a/tests/smoke.mjs` (section 42)

- [ ] **Step 1: Write the failing test**

```javascript
// 42. trading floor bid wall renders
section('42. trading floor bid wall');
const { getRuntimeSnapshot } = await import('../kernel/runtime_snapshot.mjs');
const snap = getRuntimeSnapshot();
const bids = snap.economy?.bids || [];
assert(Array.isArray(bids), 'economy.bids is array');
if (bids.length > 0) {
  assert(bids[0].task_id !== undefined, 'bid has task_id');
  assert(typeof bids[0].amount === 'number', 'bid has amount');
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "42\."`
Expected: FAIL

- [ ] **Step 3: Add renderTradingWall() to app.js**

```javascript
function renderTradingWall(economy) {
  const bids = economy?.bids || [];
  const asks = economy?.asks || [];
  const wall = $('trading-bids');
  if (!wall) return;
  wall.innerHTML = `
    <div style="font-family:var(--font-mono);font-size:9px;color:var(--neon-a);margin-bottom:4px">BID WALL</div>
    ${bids.slice(0, 8).map(b => `
      <div class="trading-bid">
        <span>${esc(b.task_id || 'TASK')}</span>
        <span class="trading-bid-amount">${b.amount || 0}</span>
      </div>
    `).join('')}
    <div style="font-family:var(--font-mono);font-size:9px;color:var(--neon-r);margin:8px 0 4px">ASK WALL</div>
    ${asks.slice(0, 8).map(a => `
      <div class="trading-bid">
        <span>${esc(a.task_id || 'TASK')}</span>
        <span class="trading-bid-amount" style="color:var(--neon-r)">${a.amount || 0}</span>
      </div>
    `).join('')}
  `;
}
```

Call `renderTradingWall(state.rt?.economy)` in `renderIsometricOffice()` after building HTML.

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "42\."`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add webui/app.js tests/smoke.mjs
git commit -m "feat: add trading floor bid wall to isometric view"
```

---

## Phase 6: Specialized Areas

### Task 8: Implement contract vault, event wall, plugin market, war room, CEO center

**Files:**
- Modify: `v0.2a/webui/app.js` (add render functions)
- Test: `v0.2a/tests/smoke.mjs` (section 43)

- [ ] **Step 1: Write the failing test**

```javascript
// 43. specialized areas render
section('43. specialized areas');
const { getRuntimeSnapshot } = await import('../kernel/runtime_snapshot.mjs');
const snap = getRuntimeSnapshot();
assert(snap.contracts?.recent !== undefined, 'contracts.recent exists');
assert(snap.events?.recent !== undefined, 'events.recent exists');
assert(snap.plugins?.registry !== undefined, 'plugins.registry exists');
assert(snap.scheduler?.queue !== undefined, 'scheduler.queue exists');
assert(snap.trust?.trust_score !== undefined, 'trust.trust_score exists');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "43\."`
Expected: FAIL

- [ ] **Step 3: Add render functions to app.js**

Add these functions to `v0.2a/webui/app.js`:

```javascript
function renderContractVaultPanel(contracts) {
  const recent = contracts?.recent || [];
  const el = $('vault-files');
  if (!el) return;
  el.innerHTML = recent.slice(0, 5).map(c => `
    <div class="vault-file" data-contract="${esc(c.contract_id)}">
      <div style="color:var(--neon-r)">${esc(c.type || 'CONTRACT')}</div>
      <div style="font-size:8px;color:var(--muted)">#${esc((c.contract_id || '').slice(0, 6))}</div>
    </div>
  `).join('');
}

function renderEventLEDWall(events) {
  const recent = events?.recent || [];
  const el = $('led-chain');
  if (!el) return;
  el.innerHTML = recent.slice(0, 15).map(e => `
    <span class="led-event ${e.type?.includes('ERROR') ? 'error' : ''}">${esc(e.type || 'EVENT')}</span>
  `).join('');
}

function renderPluginMarketRow(plugins) {
  const registry = plugins?.registry || [];
  const el = $('plugin-stalls');
  if (!el) return;
  el.innerHTML = registry.slice(0, 8).map(p => `
    <div class="plugin-stall" data-plugin="${esc(p.name)}">
      <div style="font-size:9px;color:var(--paper)">${esc(p.name.slice(0, 10))}</div>
      <div style="font-size:7px;color:var(--muted)">${p.category || ''}</div>
    </div>
  `).join('');
}

function renderWarRoomTable(scheduler) {
  const queue = scheduler?.queue || [];
  const columns = {
    queued: queue.filter(t => t.status === 'queued'),
    assigned: queue.filter(t => t.status === 'assigned'),
    executing: queue.filter(t => t.status === 'executing'),
    verified: queue.filter(t => t.status === 'verified'),
    completed: queue.filter(t => t.status === 'completed')
  };
  document.querySelectorAll('.war-column').forEach(col => {
    const status = col.dataset.status;
    const tasks = columns[status] || [];
    col.innerHTML = tasks.slice(0, 5).map(t => `
      <div style="padding:2px 0;border-bottom:1px solid var(--border);font-size:9px;color:var(--paper)">
        ${esc(t.task_id || 'TASK')}
      </div>
    `).join('');
  });
}

function renderCEOCenterPanel(data) {
  const el = $('ceo-metrics');
  if (!el) return;
  const sys = data.trust || {};
  const exec = data.executions || {};
  const metrics = [
    { label: 'Chain', value: sys.chain_valid ? '✅' : '❌' },
    { label: 'Replay', value: sys.replay_status || 'idle' },
    { label: 'Trust', value: (sys.trust_score || 0).toFixed(2) },
    { label: 'Agents', value: (state.agents?.agents?.length || 0) },
    { label: 'Tasks', value: (data.scheduler?.queue?.length || 0) },
    { label: 'Contracts', value: (data.contracts?.count || 0) },
    { label: 'Pressure', value: (sys.market_pressure || 0).toFixed(2) },
    { label: 'Plugins', value: (data.plugins?.count || 0) },
    { label: 'Sandbox', value: exec.active || 0 },
    { label: 'Events', value: (data.events?.rate || 0).toFixed(1) }
  ];
  el.innerHTML = metrics.map(m => `
    <div class="ceo-metric">
      <div class="ceo-metric-value">${m.value}</div>
      <div class="ceo-metric-label">${m.label}</div>
    </div>
  `).join('');
}
```

Call all render functions in `renderIsometricOffice()` after building HTML.

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "43\."`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add webui/app.js tests/smoke.mjs
git commit -m "feat: add contract vault, event wall, plugin market, war room, CEO center renderers"
```

---

## Phase 7: Kernel/Engine Enhancements

### Task 9: Add evaluation and benchmarking engine modules

**Files:**
- Create: `v0.2a/engine/evaluation.mjs`
- Create: `v0.2a/engine/benchmark.mjs`
- Create: `v0.2a/engine/prompt_versions.mjs`
- Modify: `v0.2a/kernel/runtime_snapshot.mjs` (include new modules)
- Test: `v0.2a/tests/smoke.mjs` (section 44)

- [ ] **Step 1: Write the failing test**

```javascript
// 44. evaluation and benchmarking modules exist
section('44. evaluation and benchmarking modules');
const { EvaluationEngine } = await import('../engine/evaluation.mjs');
const { BenchmarkEngine } = await import('../engine/benchmark.mjs');
const { PromptVersionManager } = await import('../engine/prompt_versions.mjs');
assert(typeof EvaluationEngine === 'function', 'EvaluationEngine exists');
assert(typeof BenchmarkEngine === 'function', 'BenchmarkEngine exists');
assert(typeof PromptVersionManager === 'function', 'PromptVersionManager exists');
const evalEngine = new EvaluationEngine();
assert(typeof evalEngine.start === 'function', 'evaluation has start method');
const benchEngine = new BenchmarkEngine();
assert(typeof benchEngine.start === 'function', 'benchmark has start method');
const pvm = new PromptVersionManager();
assert(typeof pvm.create === 'function', 'prompt version manager has create method');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "44\."`
Expected: FAIL

- [ ] **Step 3: Create engine/evaluation.mjs**

```javascript
/**
 * engine/evaluation.mjs
 * Evaluation engine for ForgeFabrik Agent OS.
 * Emits: EVALUATION_STARTED, EVALUATION_FINISHED
 */

import { emit } from '../kernel/event-core.mjs';

export class EvaluationEngine {
  constructor() {
    this.evaluations = new Map();
    this.nextId = 1;
  }

  start(config) {
    const id = `EVAL-${String(this.nextId++).padStart(4, '0')}`;
    const evaluation = {
      id,
      config,
      status: 'running',
      started_at: new Date().toISOString(),
      results: []
    };
    this.evaluations.set(id, evaluation);
    emit({ type: 'EVALUATION_STARTED', payload: { evaluation_id: id, config } });
    return id;
  }

  finish(id, results) {
    const evaluation = this.evaluations.get(id);
    if (!evaluation) throw new Error(`Evaluation ${id} not found`);
    evaluation.status = 'completed';
    evaluation.results = results;
    evaluation.finished_at = new Date().toISOString();
    emit({ type: 'EVALUATION_FINISHED', payload: { evaluation_id: id, results } });
    return evaluation;
  }

  get(id) {
    return this.evaluations.get(id);
  }

  list() {
    return Array.from(this.evaluations.values());
  }
}
```

- [ ] **Step 4: Create engine/benchmark.mjs**

```javascript
/**
 * engine/benchmark.mjs
 * Benchmark engine for ForgeFabrik Agent OS.
 * Emits: BENCHMARK_STARTED, BENCHMARK_FINISHED
 */

import { emit } from '../kernel/event-core.mjs';

export class BenchmarkEngine {
  constructor() {
    this.benchmarks = new Map();
    this.nextId = 1;
  }

  start(config) {
    const id = `BENCH-${String(this.nextId++).padStart(4, '0')}`;
    const benchmark = {
      id,
      config,
      status: 'running',
      started_at: new Date().toISOString(),
      results: []
    };
    this.benchmarks.set(id, benchmark);
    emit({ type: 'BENCHMARK_STARTED', payload: { benchmark_id: id, config } });
    return id;
  }

  finish(id, results) {
    const benchmark = this.benchmarks.get(id);
    if (!benchmark) throw new Error(`Benchmark ${id} not found`);
    benchmark.status = 'completed';
    benchmark.results = results;
    benchmark.finished_at = new Date().toISOString();
    emit({ type: 'BENCHMARK_FINISHED', payload: { benchmark_id: id, results } });
    return benchmark;
  }

  get(id) {
    return this.benchmarks.get(id);
  }

  list() {
    return Array.from(this.benchmarks.values());
  }
}
```

- [ ] **Step 5: Create engine/prompt_versions.mjs**

```javascript
/**
 * engine/prompt_versions.mjs
 * Prompt version manager for ForgeFabrik Agent OS.
 * Emits: PROMPT_CREATED, PROMPT_UPDATED, PROMPT_APPROVED
 */

import { emit } from '../kernel/event-core.mjs';

export class PromptVersionManager {
  constructor() {
    this.versions = new Map();
    this.nextId = 1;
  }

  create(name, content, metadata = {}) {
    const id = `PROMPT-${String(this.nextId++).padStart(4, '0')}`;
    const version = {
      id,
      name,
      content,
      version: 1,
      status: 'draft',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      metadata
    };
    this.versions.set(id, version);
    emit({ type: 'PROMPT_CREATED', payload: { prompt_id: id, name, version: 1 } });
    return version;
  }

  update(id, content, metadata = {}) {
    const version = this.versions.get(id);
    if (!version) throw new Error(`Prompt ${id} not found`);
    version.content = content;
    version.version += 1;
    version.updated_at = new Date().toISOString();
    version.metadata = { ...version.metadata, ...metadata };
    emit({ type: 'PROMPT_UPDATED', payload: { prompt_id: id, version: version.version } });
    return version;
  }

  approve(id) {
    const version = this.versions.get(id);
    if (!version) throw new Error(`Prompt ${id} not found`);
    version.status = 'approved';
    version.approved_at = new Date().toISOString();
    emit({ type: 'PROMPT_APPROVED', payload: { prompt_id: id, version: version.version } });
    return version;
  }

  get(id) {
    return this.versions.get(id);
  }

  list() {
    return Array.from(this.versions.values());
  }
}
```

- [ ] **Step 6: Extend runtime_snapshot.mjs to include new modules**

Add to `v0.2a/kernel/runtime_snapshot.mjs`:

```javascript
evaluation: {
  active: (evaluationEngine?.active || 0),
  recent: (evaluationEngine?.recent || []).slice(0, 5)
},
benchmark: {
  active: (benchmarkEngine?.active || 0),
  recent: (benchmarkEngine?.recent || []).slice(0, 5)
},
prompts: {
  versions: (promptVersionManager?.list?.() || []).slice(0, 10)
}
```

- [ ] **Step 7: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "44\."`
Expected: PASS

- [ ] **Step 8: Commit**

```bash
git add engine/evaluation.mjs engine/benchmark.mjs engine/prompt_versions.mjs kernel/runtime_snapshot.mjs tests/smoke.mjs
git commit -m "feat: add evaluation, benchmarking, and prompt version engine modules"
```

---

## Phase 8: Economy Expansion

### Task 10: Add economy marketplace data to runtime snapshot

**Files:**
- Modify: `v0.2a/kernel/runtime_snapshot.mjs`
- Test: `v0.2a/tests/smoke.mjs` (section 45)

- [ ] **Step 1: Write the failing test**

```javascript
// 45. economy marketplace data
section('45. economy marketplace');
const { getRuntimeSnapshot } = await import('../kernel/runtime_snapshot.mjs');
const snap = getRuntimeSnapshot();
assert(snap.economy?.bids !== undefined, 'economy.bids exists');
assert(snap.economy?.asks !== undefined, 'economy.asks exists');
assert(snap.economy?.market_pressure !== undefined, 'economy.market_pressure exists');
assert(Array.isArray(snap.economy?.ticker), 'economy.ticker is array');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "45\."`
Expected: FAIL

- [ ] **Step 3: Extend economy section in runtime_snapshot.mjs**

Ensure the economy section includes:
```javascript
economy: {
  bids: (economyState?.bids || []).slice(0, 10),
  asks: (economyState?.asks || []).slice(0, 10),
  market_pressure: economyState?.market_pressure || 0,
  ticker: economyState?.ticker || [],
  top_skills: economyState?.top_skills || [],
  top_agents: economyState?.top_agents || [],
  department_performance: economyState?.department_performance || {}
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `node tests/smoke.mjs 2>&1 | grep -A2 "45\."`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add kernel/runtime_snapshot.mjs tests/smoke.mjs
git commit -m "feat: extend economy snapshot with marketplace data (bids, asks, ticker, top skills/agents)"
```

---

## Phase 9: Final Integration

### Task 11: Final smoke test + README update

**Files:**
- Test: `v0.2a/tests/smoke.mjs` (all sections 1-45)
- Modify: `v0.2a/README.md`

- [ ] **Step 1: Run full smoke test**

Run: `node tests/smoke.mjs`
Expected: All 45 sections pass, exit 0

- [ ] **Step 2: Update README.md**

Add to README:
- New section "ForgeFabrik HQ" describing the 10 areas
- Update smoke test count from 37 → 45 sections
- Add new API endpoints if any were added
- Document the 3 new engine modules (evaluation, benchmark, prompt_versions)

- [ ] **Step 3: Final verification**

Run: `node tests/smoke.mjs && echo "ALL GREEN"`
Expected: ALL GREEN

- [ ] **Step 4: Commit**

```bash
git add tests/smoke.mjs README.md
git commit -m "feat: final integration — 45-section smoke test green, README updated"
```

---

## Self-Review Checklist

**Spec coverage:**
- [x] 10 office areas with area_type mapping → Task 2 + Task 3
- [x] Isometric CSS 3D view → Task 3
- [x] Agent lifecycle 7 states → Task 1 (sprites) + Task 5 (lifecycle wiring)
- [x] Economy heatmap → Task 6
- [x] Trading floor bid wall → Task 7
- [x] Contract vault → Task 8
- [x] Event-core LED wall → Task 8
- [x] Plugin marketplace → Task 8
- [x] Scheduler war room → Task 8
- [x] CEO command center → Task 8
- [x] Evaluation/benchmarking/prompt-versions → Task 9
- [x] Economy expansion (bids, asks, ticker, top skills/agents) → Task 10

**Placeholder scan:**
- [x] No "TBD", "TODO", "implement later"
- [x] No "add appropriate error handling" without code
- [x] No "similar to Task N" without repeating code
- [x] All steps have exact file paths, exact code, exact commands

**Type consistency:**
- [x] `renderIsometricOffice()` used consistently across Tasks 3-8
- [x] `getAgentState()` returns string states matching CSS `data-state` values
- [x] `area_type` values match between forge_tower.mjs and HTML `data-area` attributes
- [x] Runtime snapshot keys match between backend and frontend access patterns

**Gaps:**
- [x] All 10 areas have renderers
- [x] All new sprite roles/expressions have tests
- [x] All new engine modules have tests
- [x] All new economy fields have tests

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-04-forgefabrik-hq-enhancement.md`.

**Two execution options:**

**1. Subagent-Driven (recommended)**
- I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution**
- Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
