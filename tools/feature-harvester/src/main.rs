//! feature-harvester — propose Feature candidates from a GitHub repository.
//!
//! Subcommands:
//!   scan <owner/repo>     walk a GitHub repo, extract Feature candidates from
//!                         markdown / rustdoc / API surfaces, write JSON
//!   diff <candidates.json>    show only candidates not already in the registry

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;
use feature_schema::Registry;
use serde::{Deserialize, Serialize};

mod github;
mod parse;

use github::GitHubClient;

#[derive(Parser, Debug)]
#[command(
    name = "feature-harvester",
    about = "Propose Feature candidates by scanning a GitHub repository",
    version
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
    /// GitHub personal access token (optional; raises rate limit from 60 to 5000/h)
    #[arg(long, global = true)]
    token: Option<String>,
    /// Path to features.registry.json (for `diff`)
    #[arg(long, global = true, default_value = "features.registry.json")]
    registry: String,
}

#[derive(Parser, Debug)]
enum Cmd {
    /// Scan a GitHub repository and write candidate features to a JSON file.
    Scan {
        /// Repository in `owner/repo` form.
        repo: String,
        /// Output path for the candidates JSON (default: ./candidates-<repo>.json)
        #[arg(long)]
        output: Option<PathBuf>,
        /// Limit the number of files scanned (for smoke tests).
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Show candidates not already present in the registry.
    Diff {
        /// Path to a candidates JSON file (from `scan`).
        candidates: PathBuf,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCandidate {
    pub suggested_id: String,
    pub name: String,
    pub source_path: String,
    pub source_line: Option<u64>,
    pub rationale: String,
    pub kind: CandidateKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CandidateKind {
    /// Pulled from a markdown bullet / heading.
    MarkdownItem,
    /// Pulled from a `///` rustdoc line.
    RustdocItem,
    /// Pulled from a public `pub` item (function/struct/enum/trait).
    PublicItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateFile {
    pub repo: String,
    pub generated_at: String,
    pub candidates: Vec<FeatureCandidate>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to start tokio runtime: {e:#}");
            return ExitCode::from(2);
        }
    };
    match rt.block_on(run(args)) {
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            eprintln!("{e:#}");
            ExitCode::from(2)
        }
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    match args.cmd {
        Cmd::Scan {
            repo,
            output,
            limit,
        } => {
            scan(args.token, &repo, output.as_deref(), limit).await?;
        }
        Cmd::Diff { candidates } => {
            diff(&args.registry, &candidates).await?;
        }
    }
    Ok(())
}

async fn scan(
    token: Option<String>,
    repo: &str,
    output: Option<&Path>,
    limit: Option<usize>,
) -> anyhow::Result<()> {
    let client = GitHubClient::new(token);
    eprintln!("scanning {repo} ...");
    let tree = client.list_tree(repo, "HEAD").await?;
    eprintln!("tree: {} entries", tree.len());

    // Filter to interesting paths.
    let interesting: Vec<&github::TreeEntry> = tree
        .iter()
        .filter(|e| e.is_file() && parse::is_interesting_path(&e.path))
        .collect();
    eprintln!("interesting files: {}", interesting.len());

    let to_scan: Vec<&github::TreeEntry> = match limit {
        Some(n) => interesting.into_iter().take(n).collect(),
        None => interesting,
    };

    let mut all_candidates: Vec<FeatureCandidate> = Vec::new();
    let total = to_scan.len();
    for (i, entry) in to_scan.iter().enumerate() {
        if (i + 1) % 10 == 0 || i + 1 == total {
            eprintln!("  [{}/{}] {}", i + 1, total, entry.path);
        }
        let blob = match client.get_blob(repo, &entry.sha).await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("    skip {}: {e:#}", entry.path);
                continue;
            }
        };
        let text = match std::str::from_utf8(&blob) {
            Ok(s) => s,
            Err(_) => continue, // binary
        };
        let kind = parse::kind_for_path(&entry.path);
        let cands = parse::extract_candidates(&entry.path, text, kind);
        all_candidates.extend(cands);
    }

    let out_path = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(format!("candidates-{}.json", repo.replace('/', "-"))));
    let file = CandidateFile {
        repo: repo.to_string(),
        generated_at: now_iso8601(),
        candidates: all_candidates,
    };
    let json = serde_json::to_string_pretty(&file)?;
    std::fs::write(&out_path, json).with_context(|| format!("writing {}", out_path.display()))?;
    eprintln!(
        "wrote {} ({} candidates) to {}",
        out_path.display(),
        file.candidates.len(),
        out_path.display()
    );
    Ok(())
}

async fn diff(registry_path: &str, candidates_path: &Path) -> anyhow::Result<()> {
    let reg = Registry::load(Path::new(registry_path))?;
    let text = std::fs::read_to_string(candidates_path)
        .with_context(|| format!("reading {}", candidates_path.display()))?;
    let file: CandidateFile = serde_json::from_slice(text.as_bytes())
        .with_context(|| format!("parsing {}", candidates_path.display()))?;
    let known: std::collections::BTreeSet<&str> =
        reg.features.iter().map(|f| f.id.as_str()).collect();
    let new_only: Vec<&FeatureCandidate> = file
        .candidates
        .iter()
        .filter(|c| {
            // canonicalize suggested_id to registry id form (lowercase dotted)
            !known.contains(c.suggested_id.as_str())
        })
        .collect();
    println!(
        "{} candidates, {} new (not in registry):",
        file.candidates.len(),
        new_only.len()
    );
    for c in &new_only {
        println!(
            "  [{}] {}  ({}:{})",
            c.kind.as_str(),
            c.suggested_id,
            c.source_path,
            c.source_line
                .map(|n| n.to_string())
                .unwrap_or_else(|| "-".into())
        );
        println!("    {}", c.name);
    }
    Ok(())
}

impl FeatureCandidate {
    // nothing extra
}

impl CandidateKind {
    fn as_str(&self) -> &'static str {
        match self {
            CandidateKind::MarkdownItem => "markdown-item",
            CandidateKind::RustdocItem => "rustdoc-item",
            CandidateKind::PublicItem => "public-item",
        }
    }
}

fn now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // We don't pull in chrono; emit a simple UTC ISO-8601 form.
    // 86400 * 365.25 ~= 31557600
    let days = (secs / 86400) as i64;
    let mut year = 1970i64;
    let mut remaining = days;
    loop {
        let leap = is_leap(year);
        let year_days = if leap { 366 } else { 365 };
        if remaining < year_days {
            break;
        }
        remaining -= year_days;
        year += 1;
    }
    let leap = is_leap(year);
    let months = [
        31,
        28 + if leap { 1 } else { 0 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1u32;
    for &m in &months {
        if remaining < m {
            break;
        }
        remaining -= m;
        month += 1;
    }
    let day = remaining + 1;
    let secs_today = (secs % 86400) as u32;
    let hh = secs_today / 3600;
    let mm = (secs_today % 3600) / 60;
    let ss = secs_today % 60;
    format!("{year:04}-{month:02}-{day:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
