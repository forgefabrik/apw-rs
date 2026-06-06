//! Heuristic feature candidate extraction from text blobs.
//!
//! For markdown: scan for `- [ ] ...`, `## Feature: ...`, `### ` sections.
//! For rust source: scan for `///` doc lines, `pub fn/struct/enum/trait` items.
//!
//! This is intentionally simple — the goal is "raise a hand" on potentially
//! missing features, not deduplicate or rank. The human stays in the loop:
//! `feature-harvester diff` shows what's not in the registry, and the human
//! decides what to add.

use crate::{CandidateKind, FeatureCandidate};

pub fn is_interesting_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".md") {
        // skip generated / vendored / lockfiles / huge CHANGELOGs
        if lower.contains("changelog") || lower.contains("license") || lower.contains("/.git/") {
            return false;
        }
        return true;
    }
    if lower.ends_with(".rs") {
        // skip build outputs, target, vendored
        if lower.starts_with("target/") || lower.contains("/.git/") {
            return false;
        }
        return true;
    }
    false
}

pub fn kind_for_path(path: &str) -> CandidateKind {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".md") {
        CandidateKind::MarkdownItem
    } else if lower.ends_with(".rs") {
        CandidateKind::PublicItem
    } else {
        CandidateKind::MarkdownItem
    }
}

pub fn extract_candidates(path: &str, text: &str, kind: CandidateKind) -> Vec<FeatureCandidate> {
    match kind {
        CandidateKind::MarkdownItem => extract_markdown(path, text),
        CandidateKind::PublicItem | CandidateKind::RustdocItem => extract_rust(path, text),
    }
}

fn extract_markdown(path: &str, text: &str) -> Vec<FeatureCandidate> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        // Task-list item: - [ ] Feature: foo  or  - [x] **foo**
        if let Some(rest) = trimmed
            .strip_prefix("- [ ]")
            .or_else(|| trimmed.strip_prefix("- [x]"))
            .or_else(|| trimmed.strip_prefix("- [X]"))
        {
            let name = rest.trim().trim_start_matches(':').trim();
            if name.is_empty() || name.len() > 200 {
                continue;
            }
            // skip pure meta
            if name.starts_with("http") || name.starts_with("[![") {
                continue;
            }
            if let Some(c) =
                build_candidate(path, name, (i + 1) as u64, CandidateKind::MarkdownItem)
            {
                out.push(c);
            }
        } else if let Some(rest) = trimmed.strip_prefix("## Feature:") {
            let name = rest.trim();
            if !name.is_empty() {
                if let Some(c) =
                    build_candidate(path, name, (i + 1) as u64, CandidateKind::MarkdownItem)
                {
                    out.push(c);
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("### ") {
            // sub-section heading — include as a candidate only if it looks like
            // a feature noun (starts with capital, <= 80 chars, no colon).
            let name = rest.trim();
            if !name.is_empty()
                && name.len() <= 120
                && name
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                    .unwrap_or(false)
                && !name.contains(':')
            {
                if let Some(c) =
                    build_candidate(path, name, (i + 1) as u64, CandidateKind::MarkdownItem)
                {
                    out.push(c);
                }
            }
        }
    }
    out
}

fn extract_rust(path: &str, text: &str) -> Vec<FeatureCandidate> {
    let mut out = Vec::new();
    let mut current_doc: Vec<(u64, String)> = Vec::new();
    let mut in_pub_item: Option<(u64, String, &'static str)> = None;
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("///") {
            current_doc.push(((i + 1) as u64, rest.trim().to_string()));
        } else if let Some(rest) = trimmed.strip_prefix("pub ") {
            // We have a `pub` line — capture the kind.
            let kind = if rest.starts_with("fn ") {
                "fn"
            } else if rest.starts_with("struct ") {
                "struct"
            } else if rest.starts_with("enum ") {
                "enum"
            } else if rest.starts_with("trait ") {
                "trait"
            } else if rest.starts_with("mod ") {
                "mod"
            } else if rest.starts_with("type ") {
                "type"
            } else if rest.starts_with("const ") || rest.starts_with("static ") {
                "const"
            } else {
                ""
            };
            if kind.is_empty() {
                in_pub_item = None;
            } else {
                // First token after `pub <kind> ` is the item name
                let name = rest
                    .trim_start_matches(kind)
                    .trim_start()
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap_or("")
                    .to_string();
                in_pub_item = Some(((i + 1) as u64, name, kind));
            }
        } else if trimmed.starts_with("//") {
            // ignore other comments
        } else {
            // blank/non-doc/non-pub line: emit a candidate if we have one
            if let Some((line_no, name, kind)) = in_pub_item.take() {
                // pick a doc summary: first non-empty `///` line, or "<kind> name"
                let summary = current_doc
                    .iter()
                    .map(|(_, t)| t.as_str())
                    .find(|t| !t.is_empty())
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| format!("{kind} {name}"));
                if !name.is_empty() {
                    if let Some(c) = build_candidate_pub(path, &name, &summary, line_no) {
                        out.push(c);
                    }
                }
            }
            current_doc.clear();
        }
    }
    // tail
    if let Some((line_no, name, kind)) = in_pub_item.take() {
        let summary = current_doc
            .iter()
            .map(|(_, t)| t.as_str())
            .find(|t| !t.is_empty())
            .map(|t| t.to_string())
            .unwrap_or_else(|| format!("{kind} {name}"));
        if !name.is_empty() {
            if let Some(c) = build_candidate_pub(path, &name, &summary, line_no) {
                out.push(c);
            }
        }
    }
    out
}

fn build_candidate(
    path: &str,
    raw_name: &str,
    line: u64,
    kind: CandidateKind,
) -> Option<FeatureCandidate> {
    let cleaned = strip_markdown(raw_name);
    if cleaned.is_empty() {
        return None;
    }
    let suggested_id = slugify(&cleaned);
    if suggested_id.is_empty() || suggested_id.len() < 3 {
        return None;
    }
    Some(FeatureCandidate {
        suggested_id,
        name: cleaned,
        source_path: path.to_string(),
        source_line: Some(line),
        rationale: format!("Heuristic match in {path}:{line}"),
        kind,
    })
}

fn build_candidate_pub(
    path: &str,
    name: &str,
    summary: &str,
    line: u64,
) -> Option<FeatureCandidate> {
    let suggested_id = format!("pub.{}.{}", parent_module(path), name);
    Some(FeatureCandidate {
        suggested_id,
        name: format!("`{name}` — {summary}"),
        source_path: path.to_string(),
        source_line: Some(line),
        rationale: format!("Public item at {path}:{line}"),
        kind: CandidateKind::PublicItem,
    })
}

fn strip_markdown(s: &str) -> String {
    let mut out = s.to_string();
    // drop inline code/backticks
    if out.contains('`') {
        out = out.replace('`', "");
    }
    // strip leading "Feature:" / "Feature "
    if let Some(rest) = out.strip_prefix("Feature:") {
        out = rest.trim().to_string();
    } else if let Some(rest) = out.strip_prefix("Feature ") {
        out = rest.trim().to_string();
    }
    // strip surrounding ** ** (full wrap or leading/trailing)
    if out.starts_with("**") && out.ends_with("**") && out.len() > 4 {
        out = out[2..out.len() - 2].to_string();
    } else {
        if out.starts_with("**") {
            out = out[2..].to_string();
        }
        if out.ends_with("**") && out.len() > 2 {
            out = out[..out.len() - 2].to_string();
        }
    }
    if out.starts_with('[') {
        if let Some(end) = out.find("](") {
            if let Some(close) = out[end..].find(')') {
                out = out[1..end].to_string() + &out[end + close + 1..];
            }
        }
    }
    out.trim().to_string()
}

fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_dash = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if c.is_whitespace() || c == '-' || c == '_' || c == '.' {
            if !prev_dash && !out.is_empty() {
                out.push('-');
                prev_dash = true;
            }
        }
        // drop everything else
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

fn parent_module(path: &str) -> String {
    // foo/src/bar/baz.rs -> "bar.baz"
    let stem = path.rsplit_once('.').map(|(p, _)| p).unwrap_or(path);
    let parts: Vec<&str> = stem
        .split('/')
        .filter(|p| !p.is_empty() && *p != "src" && *p != "lib" && *p != "mod")
        .collect();
    if parts.is_empty() {
        "root".into()
    } else {
        parts.join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_basic() {
        assert_eq!(slugify("Hello World!"), "hello-world");
        assert_eq!(slugify("foo_bar.baz"), "foo-bar-baz");
        assert_eq!(slugify("   spaces   "), "spaces");
    }

    #[test]
    fn strip_markdown_inline() {
        assert_eq!(strip_markdown("**Foo**"), "Foo");
        assert_eq!(strip_markdown("[Foo](http://x) bar"), "Foo bar");
        assert_eq!(strip_markdown("`code` name"), "code name");
    }

    #[test]
    fn markdown_task_extraction() {
        let text = "# Title\n- [ ] Implement ZK proofs\n- [x] Token API\n";
        let cands = extract_markdown("README.md", text);
        assert_eq!(cands.len(), 2);
        assert_eq!(cands[0].suggested_id, "implement-zk-proofs");
        assert!(cands.iter().any(|c| c.suggested_id == "token-api"));
    }

    #[test]
    fn rust_pub_extraction() {
        let text = "/// summary line\npub fn foo() {}\n\npub struct Bar;\n";
        let cands = extract_rust("src/lib.rs", text);
        assert_eq!(cands.len(), 2);
        assert!(cands[0].name.contains("foo"));
    }

    #[test]
    fn interesting_paths() {
        assert!(is_interesting_path("README.md"));
        assert!(is_interesting_path("docs/intro.md"));
        assert!(!is_interesting_path("LICENSE.md"));
        assert!(!is_interesting_path("CHANGELOG.md"));
        assert!(is_interesting_path("src/lib.rs"));
        assert!(!is_interesting_path("target/debug/foo.rs"));
    }
}
