# Event-Katalog — apw-rs

## Event-Format

struct EventEnvelope {
    schema_version: u32,
    tick: Tick,
    actor: ActorId,
    event_hash: Option<[u8; 32]>,
    prev_event_hash: Option<[u8; 32]>,
    payload: Event,
}

## Event-Tabelle

| Event | Felder | Auslöser | Konsument |
|---|---|---|---|
| AgentExpressionChanged | agent_id, expression | Agent-wechselt-Expression | Office |
| AgentPromoted | agent_id, from_floor, to_floor, cost | Auktion/Admin | Office, Scheduler |
| ItemPurchased | agent_id, item_id, cost | Economy | Buchhaltung |
| SubsystemStatusChanged | room_id, status, heat | Health-Check | Dashboard |
| LeaseAcquired | agent_id, lease_id | Lease-Vergabe | Scheduler |
| LeaseCompleted | agent_id, lease_id, success | Lease-Abschluss | Scheduler, Economy |
| CapabilityDenied | actor, capability, reason | Capability-Check | Audit, Office |

## Governance

- Schema Version: schema_version hochzählen bei Breaking Change
- Hash-Verkettung: Nur asynchron prüfbar (Algebra-Engine)
- Determinismus: Keine Floats, nur BTreeMap/BTreeSet

