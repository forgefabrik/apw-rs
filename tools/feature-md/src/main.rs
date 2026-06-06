use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;

mod render;

use feature_schema::Registry;

#[derive(Parser, Debug)]
#[command(
    name = "feature-md",
    about = "Render features.registry.json to Markdown",
    version
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Parser, Debug)]
enum Cmd {
    /// Render the registry to a Markdown file (default: docs/FEATURES.md).
    Render {
        /// Path to features.registry.json (default: ./features.registry.json)
        #[arg(long, default_value = "features.registry.json")]
        registry: String,
        /// Output Markdown path (default: ./docs/FEATURES.md)
        #[arg(long, default_value = "docs/FEATURES.md")]
        output: String,
        /// Exit 0 if output is up-to-date, 1 if stale (don't write).
        #[arg(long)]
        check: bool,
    },
}

fn main() -> ExitCode {
    let args = Args::parse();
    match args.cmd {
        Cmd::Render {
            registry,
            output,
            check,
        } => render(&registry, &output, check),
    }
}

fn render(reg_path: &str, out_path: &str, check: bool) -> ExitCode {
    let text = match std::fs::read_to_string(reg_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed to read {reg_path}: {e}");
            return ExitCode::from(2);
        }
    };
    let registry: Registry =
        match serde_json::from_str(&text).with_context(|| format!("parsing {reg_path}")) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("failed to parse {reg_path}: {e:#}");
                return ExitCode::from(2);
            }
        };
    let markdown = render::to_markdown(&registry);
    if check {
        let existing = std::fs::read_to_string(out_path).unwrap_or_default();
        if existing == markdown {
            println!("{out_path} is up-to-date");
            return ExitCode::from(0);
        }
        eprintln!("{out_path} is STALE; run: cargo run -p feature-md -- render");
        return ExitCode::from(1);
    }
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if let Err(e) = std::fs::write(out_path, &markdown) {
        eprintln!("failed to write {out_path}: {e}");
        return ExitCode::from(2);
    }
    println!(
        "wrote {out_path} ({} features, {} sources)",
        registry.features.len(),
        registry.sources.len()
    );
    ExitCode::from(0)
}
