//! Canonical serializer
#![forbid(unsafe_code)]
use crate::{EventEnvelope, KvValue};
pub const BLAKE3_KEY: [u8;32] = *b"apw-rs-dev-key-placeholder-change-in-production";
pub fn blake3_hash(b: &[u8]) -> [u8;32] { blake3::Hasher::new_keyed(&BLAKE3_KEY).update(b).finalize().into() }
pub fn canonical_json(e: &EventEnvelope) -> Vec<u8> { serde_json::to_vec(e).unwrap() }
pub fn canonical_bytes(e: &EventEnvelope) -> Vec<u8> { canonical_json(e) }
impl KvValue { pub fn is_empty(&self) -> bool { match self { KvValue::Text(s) => s.is_empty(), KvValue::Bool(b) => !b, _ => false } } }
#[test] fn test() { assert!(KvValue::Text(String::new()).is_empty()); assert!(KvValue::Bool(false).is_empty()); }
