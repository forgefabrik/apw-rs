//! Validation checks for features.registry.json.

use std::collections::BTreeSet;

use feature_schema::Registry;

#[derive(Debug, Clone)]
pub struct Issue {
    pub severity: Severity,
    pub check: &'static str,
    pub target: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warn,
}

#[derive(Debug, Default)]
pub struct Report {
    pub issues: Vec<Issue>,
}

impl Report {
    pub fn err(&mut self, check: &'static str, target: String, msg: String) {
        self.issues.push(Issue {
            severity: Severity::Error,
            check,
            target,
            message: msg,
        });
    }
    pub fn warn(&mut self, check: &'static str, target: String, msg: String) {
        self.issues.push(Issue {
            severity: Severity::Warn,
            check,
            target,
            message: msg,
        });
    }
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warn)
    }
}

const PHASES: &[&str] = &["phase-1", "phase-2"];
const MILESTONES: &[&str] = &["M0", "M1", "M2", "M3", "M4", "M5", "M6+"];
const STATUSES: &[&str] = &["done", "porting", "planned", "deferred", "not-adopted"];

pub fn run(r: &Registry) -> Report {
    let mut report = Report::default();
    check_top_level(r, &mut report);
    check_sources(r, &mut report);
    let feature_index = r.feature_index();
    let source_index = r.source_index();
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for f in &r.features {
        check_feature(f, &feature_index, &source_index, &mut report);
        if !seen_ids.insert(f.id.as_str()) {
            report.err(
                "duplicate-id",
                f.id.clone(),
                format!("feature id '{}' appears more than once", f.id),
            );
        }
    }
    report
}

fn check_top_level(r: &Registry, report: &mut Report) {
    if !is_semver(&r.schema_version) {
        report.err(
            "schema-version",
            "(root)".to_string(),
            format!("schema_version '{}' is not semver", r.schema_version),
        );
    }
    if r.registry_revision == 0 {
        report.warn(
            "registry-revision",
            "(root)".to_string(),
            "registry_revision is 0; bump it on first real commit".to_string(),
        );
    }
}

fn check_sources(r: &Registry, report: &mut Report) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for s in &r.sources {
        if !seen.insert(s.id.as_str()) {
            report.err(
                "duplicate-source",
                s.id.clone(),
                format!("source id '{}' appears more than once", s.id),
            );
        }
        if s.id.is_empty() {
            report.err(
                "empty-source-id",
                "(source)".to_string(),
                "source id is empty".to_string(),
            );
        }
    }
}

fn check_feature(
    f: &feature_schema::Feature,
    feature_index: &std::collections::BTreeMap<&str, &feature_schema::Feature>,
    source_index: &std::collections::BTreeMap<&str, &feature_schema::Source>,
    report: &mut Report,
) {
    let target = f.id.clone();
    if !is_feature_id(&f.id) {
        report.err(
            "id-pattern",
            target.clone(),
            format!("id '{}' must match ^\\d+\\.\\d+\\.\\d+$", f.id),
        );
    }
    if f.name.is_empty() {
        report.err("empty-name", target.clone(), "name is empty".to_string());
    }
    if !PHASES.contains(&f.phase.as_str()) {
        report.err(
            "bad-phase",
            target.clone(),
            format!("phase '{}' not in {PHASES:?}", f.phase),
        );
    }
    if !MILESTONES.contains(&f.milestone.as_str()) {
        report.err(
            "bad-milestone",
            target.clone(),
            format!("milestone '{}' not in {MILESTONES:?}", f.milestone),
        );
    }
    if !STATUSES.contains(&f.status.as_str()) {
        report.err(
            "bad-status",
            target.clone(),
            format!("status '{}' not in {STATUSES:?}", f.status),
        );
    }
    if let Some(sid) = &f.source_id {
        if !source_index.contains_key(sid.as_str()) {
            report.err(
                "dangling-source-id",
                target.clone(),
                format!("source_id '{}' does not match any source", sid),
            );
        }
    }
    if let Some(to) = &f.to {
        let crate_present = to.krate.as_deref().map(|s| !s.is_empty()).unwrap_or(false)
            || to.module.as_deref().map(|s| !s.is_empty()).unwrap_or(false)
            || to.path.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
        if !crate_present {
            report.err(
                "empty-to",
                target.clone(),
                "to.{crate,module,path} all empty".to_string(),
            );
        }
    }
    let mut seen_dep: BTreeSet<&str> = BTreeSet::new();
    for dep in &f.depends_on {
        if !seen_dep.insert(dep.as_str()) {
            report.err(
                "duplicate-dep",
                target.clone(),
                format!("depends_on lists '{}' twice", dep),
            );
        }
        if !feature_index.contains_key(dep.as_str()) {
            report.err(
                "dangling-dep",
                target.clone(),
                format!("depends_on '{}' does not exist", dep),
            );
        }
        if dep == &f.id {
            report.err(
                "self-dep",
                target.clone(),
                "feature depends on itself".to_string(),
            );
        }
    }
    if f.status == "done" {
        let in_future = matches!(f.milestone.as_str(), "M6+");
        if in_future && f.evidence.is_none() {
            report.warn(
                "done-without-evidence",
                target,
                format!(
                    "status=done with milestone={} and no evidence — confirm the proof is in the linked PR",
                    f.milestone
                ),
            );
        }
    }
}

fn is_semver(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

fn is_feature_id(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}
