use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;

mod checks;

use checks::Report;
use feature_schema::Registry;

#[derive(Parser, Debug)]
#[command(
    name = "feature-guardian",
    about = "Validate features.registry.json",
    version
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Parser, Debug)]
enum Cmd {
    /// Validate the registry against the project's checks.
    Check {
        /// Path to features.registry.json (default: ./features.registry.json)
        #[arg(long, default_value = "features.registry.json")]
        registry: String,
        /// Treat warnings as errors.
        #[arg(long)]
        strict: bool,
    },
}

fn main() -> ExitCode {
    let args = Args::parse();
    match args.cmd {
        Cmd::Check { registry, strict } => check(&registry, strict),
    }
}

fn check(path: &str, strict: bool) -> ExitCode {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed to read {path}: {e}");
            return ExitCode::from(2);
        }
    };
    let registry: Registry =
        match serde_json::from_str(&text).with_context(|| format!("parsing {path}")) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("failed to parse {path}: {e:#}");
                return ExitCode::from(2);
            }
        };
    let report: Report = checks::run(&registry);
    let n_feat = registry.features.len();
    let n_src = registry.sources.len();
    for i in &report.issues {
        let tag = match i.severity {
            checks::Severity::Error => "ERROR",
            checks::Severity::Warn => "WARN ",
        };
        let target = if i.target.is_empty() {
            "-".to_string()
        } else {
            i.target.clone()
        };
        eprintln!("{tag}  {:<24}  {:<14}  {}", i.check, target, i.message);
    }
    let n_err = report
        .issues
        .iter()
        .filter(|i| i.severity == checks::Severity::Error)
        .count();
    let n_warn = report
        .issues
        .iter()
        .filter(|i| i.severity == checks::Severity::Warn)
        .count();
    eprintln!();
    eprintln!("{path}: {n_feat} features, {n_src} sources, {n_err} errors, {n_warn} warnings");
    if report.has_errors() {
        ExitCode::from(1)
    } else if report.has_warnings() && strict {
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}
