use clap::{Parser, Subcommand};

/// apw-rs CLI — Tower-Admin, Replay und Status.
#[derive(Parser)]
#[command(name = "apw", version, about = "ForgeFabrik Agent OS CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Pixel-Office TUI starten
    Office {
        /// Server-URL (default: http://localhost:8080)
        #[arg(short, long, default_value = "http://localhost:8080")]
        server: String,
    },
    /// Replay-Log analysieren
    Replay {
        /// Pfad zur Event-DB
        path: String,
        /// Ab Tick (optional)
        #[arg(short, long)]
        from: Option<u64>,
    },
    /// Server-Status abfragen
    Status {
        /// Server-URL
        #[arg(short, long, default_value = "http://localhost:8080")]
        server: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Office { server } => {
            println!("Office starten — server={server}");
            // TODO: apw-office::run(server)
            Ok(())
        }
        Command::Replay { path, from } => {
            println!("Replay — path={path}, from={from:?}");
            // TODO: apw-kernel::replay::run(path, from)
            Ok(())
        }
        Command::Status { server } => {
            println!("Status — server={server}");
            // TODO: HTTP-GET /status
            Ok(())
        }
    }
}

