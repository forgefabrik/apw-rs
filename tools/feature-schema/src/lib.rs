//! Serde types that mirror features.registry.schema.json 1:1.
//!
//! Shared across `feature-guardian`, `feature-md`, `feature-graph`,
//! and `feature-harvester` so there is one source of truth for the
//! registry shape.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub schema_version: String,
    pub registry_revision: u64,
    #[serde(default)]
    pub generated_at: Option<String>,
    #[serde(default)]
    pub sources: Vec<Source>,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub repo: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub phase: String,
    pub milestone: String,
    pub status: String,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub from: Option<FromRef>,
    #[serde(default)]
    pub to: Option<ToRef>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub evidence: Option<Evidence>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromRef {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub line: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToRef {
    #[serde(rename = "crate", default)]
    pub krate: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    #[serde(default)]
    pub doc_url: Option<String>,
    #[serde(default)]
    pub doc_line: Option<u64>,
    #[serde(default)]
    pub rationale: Option<String>,
}

impl Registry {
    /// Load a registry from a JSON file.
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;
        let reg: Self = serde_json::from_slice(&bytes)?;
        Ok(reg)
    }

    pub fn feature_index(&self) -> BTreeMap<&str, &Feature> {
        self.features.iter().map(|f| (f.id.as_str(), f)).collect()
    }

    pub fn source_index(&self) -> BTreeMap<&str, &Source> {
        self.sources.iter().map(|s| (s.id.as_str(), s)).collect()
    }
}
