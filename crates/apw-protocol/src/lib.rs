//! apw-protocol — kanonische Wire-Types für das ForgeFabrik Agent OS.
//!
//! Diese Crate enthält **nur** serialisierbare Datenstrukturen.
//! Sie hat KEINE `std::fs`-Abhängigkeiten, KEINE Netzwerk-IO,
//! KEINE asynchrone Runtime, und KEINE `serde_json::Value`-Instanziierung.
//!
//! ## Governance (aus Design-Spec)
//!
//! - **Time-Policy:** `std::time::SystemTime` ist **verboten**.
//!   Autoritative Zeit ist ein `Tick`, geliefert von einem `ClockSource`.
//! - **Determinismus-Policy:** Replay-authoritativer State darf nur
//!   `BTreeMap`/`BTreeSet` enthalten — niemals `HashMap`/`HashSet`.
//! - **Serialisierungs-Policy:** Keine `f32`/`f64` in kanonischen Pfaden.
//!   Alle numerischen Werte sind Integer, Fixed-Point (`u32` als Milli-Promille),
//!   oder string-kodierte Rationals.
//! - **Versionierungs-Policy:** Jeder versionierte Typ trägt
//!   `schema_version: u32` auf der Hülltyp-Ebene, nicht auf dem Payload-Enum.

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic,
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::single_char_add_str,
    clippy::needless_collect,
    clippy::if_same_then_else,
    clippy::useless_vec,
    clippy::redundant_closure_for_method_calls,
    clippy::iter_cloned_collect,
    clippy::for_kv_map,
)]

pub fn name() -> &'static str {
    "apw-protocol"
}

// ===========================================================================
// Identity
// ===========================================================================

/// Eindeutiger Bezeichner für einen Agenten.
#[derive(
    Clone, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
pub struct AgentId(pub String);

/// Eindeutiger Bezeichner für ein Lease (Auftrag/Ausführungsberechtigung).
#[derive(
    Clone, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
pub struct LeaseId(pub String);

/// Eindeutiger Bezeichner für einen Akteur, der ein Event ausgelöst hat.
#[derive(
    Clone, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
pub struct ActorId(pub String);

// ===========================================================================
// Time — Tick ist die autoritative Zeit im Kernel
// ===========================================================================

/// Autoritative Zeit im Kernel: ein monotin vorwärts zählender Zähler.
#[derive(
    Clone, Copy, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
pub struct Tick(pub u64);

/// Read-Only-Clock: "Welcher Tick ist es gerade?"
pub trait ClockSource: Send + Sync {
    fn tick(&self) -> Tick;
}

/// Veränderbare Uhr für Simulationen — **nicht** für Replay oder Wanduhr.
pub trait SimulationClock: ClockSource {
    fn advance(&mut self, n: u64);
}

// ===========================================================================
// Role
// ===========================================================================

#[derive(
    Clone, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Implementation,
    Research,
    Review,
    Planning,
    Ceo,
    Sandbox,
    Economist,
    ReplayAgent,
    TrustAgent,
    Custom(String),
}

// ===========================================================================
// Expression
// ===========================================================================

#[derive(
    Clone, Debug,
    PartialEq, Eq, Hash,
    serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Expression {
    Idle,
    Working,
    Thinking,
    Blocked,
    Walking,
    Seated,
    Sleeping,
    Custom(String),
}

// ===========================================================================
// Status
// ===========================================================================

#[derive(
    Clone, Copy, Debug,
    PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Warn,
    Alert,
    Idle,
}

// ===========================================================================
// Capability + Authority Map (Capability-Policy)
// ===========================================================================

#[derive(
    Clone, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Capability {
    PromoteAgent { from_floor: u8, to_floor: u8 },
    AllocateLease { task_id: String, ttl_ticks: u64 },
    SubmitSpriteProposal { pack: String, frame: String },
    ModifyAuthorityMap,
    RunTowerAdmin,
    ReplayChain,
}

#[derive(
    Clone, Debug,
    PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DenialReason {
    MissingCapability,
    ExpiredLease,
    InvalidScope,
    AuthorityMapRejected,
    ReplayOnlyOperation,
    Other(String),
}

/// Authority Map: `BTreeMap` (nicht `HashMap`) für deterministische Iteration.
pub type AuthorityMap = std::collections::BTreeMap<ActorId, std::collections::BTreeSet<Capability>>;

// ===========================================================================
// Events
// ===========================================================================

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EventEnvelope {
    pub schema_version: u32,
    pub tick: Tick,
    pub actor: ActorId,
    pub event_hash: Option<[u8; 32]>,
    pub prev_event_hash: Option<[u8; 32]>,
    pub payload: Event,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    AgentExpressionChanged {
        agent_id: AgentId,
        expression: Expression,
    },
    AgentPromoted {
        agent_id: AgentId,
        from_floor: u8,
        to_floor: u8,
        cost: u64,
    },
    ItemPurchased {
        agent_id: AgentId,
        item_id: String,
        cost: u64,
    },
    SubsystemStatusChanged {
        room_id: String,
        status: Status,
        heat: u8,
    },
    LeaseAcquired {
        agent_id: AgentId,
        lease_id: LeaseId,
    },
    LeaseCompleted {
        agent_id: AgentId,
        lease_id: LeaseId,
        success: bool,
    },
    CapabilityDenied {
        actor: ActorId,
        capability: Capability,
        reason: DenialReason,
    },
}

// ===========================================================================
// State types
// ===========================================================================

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AgentState {
    pub agent_id: AgentId,
    pub role: Role,
    pub floor: u8,
    pub desk_id: u8,
    pub has_lease: bool,
    pub blocked: bool,
    pub reputation_milli: u32,
    pub wallet: u64,
    pub current_expression: Expression,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SubsystemPayload {
    Algebra { chain_valid: bool, chain_length: u64 },
    Scheduler { queue_size: u64, interval_ticks: u64 },
    Economy { bids: u64, multiplier_milli: u32 },
    Sandbox { live_pids: u64 },
    GenericKv { entries: std::collections::BTreeMap<String, KvValue> },
}

#[derive(
    Clone, Debug,
    serde::Serialize, serde::Deserialize,
    PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum KvValue {
    Int(i64),
    Uint(u64),
    Bool(bool),
    Text(String),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BidRequest {
    pub agent_id: AgentId,
    pub target_floor: u8,
    pub max_price: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BidResult {
    pub accepted: bool,
    pub new_floor: u8,
    pub price_paid: u64,
}
// ===========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_protocol_compiles() {
        assert_eq!(name(), "apw-protocol");
    }

    #[test]
    fn canonical_json_roundtrip() {
        let env = EventEnvelope {
            schema_version: 1,
            tick: Tick(100),
            actor: ActorId("kernel".into()),
            event_hash: None,
            prev_event_hash: None,
            payload: Event::AgentExpressionChanged {
                agent_id: AgentId("agent-1".into()),
                expression: Expression::Working,
            },
        };
        let json = serde_json::to_vec(&env).unwrap();
        let back: EventEnvelope = serde_json::from_slice(&json).unwrap();
        assert_eq!(env, back);
    }

    #[test]
    fn authority_map_iteration_is_sorted() {
        let mut map: AuthorityMap = std::collections::BTreeMap::new();
        map.insert(
            ActorId("z-actor".into()),
            std::collections::BTreeSet::from([Capability::RunTowerAdmin]),
        );
        map.insert(
            ActorId("a-actor".into()),
            std::collections::BTreeSet::from([Capability::ReplayChain]),
        );
        let keys: Vec<_> = map.keys().cloned().collect();
        assert_eq!(keys[0], ActorId("a-actor".into()));
    }

    #[test]
    fn schema_version_present() {
        let env = EventEnvelope {
            schema_version: 1,
            tick: Tick(0),
            actor: ActorId("test".into()),
            event_hash: None,
            prev_event_hash: None,
            payload: Event::LeaseAcquired {
                agent_id: AgentId("a".into()),
                lease_id: LeaseId("l".into()),
            },
        };
        assert_eq!(env.schema_version, 1);
    }
}
