use clap::{Parser, Subcommand};
use std::net::{SocketAddr, ToSocketAddrs};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Office { server } => {
            let bind = socket_addr_from_url(&server)?;
            println!("Office server listening on http://{bind}");
            apw_server::serve(apw_server::ServerConfig { bind }).await
        },
        Command::Replay { path, from } => {
            println!("Replay — path={path}, from={from:?}");
            // TODO: apw-kernel::replay::run(path, from)
            Ok(())
        },
        Command::Status { server } => {
            let url = format!("{}/status", server.trim_end_matches('/'));
            let body = reqwest::get(&url).await?.error_for_status()?.text().await?;
            println!("Status — server={server} {body}");
            Ok(())
        },
    }
}

fn socket_addr_from_url(server: &str) -> anyhow::Result<SocketAddr> {
    let url = reqwest::Url::parse(server)?;
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("server URL must include a host"))?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| anyhow::anyhow!("server URL must include a port"))?;
    let host = if host == "localhost" {
        "127.0.0.1"
    } else {
        host
    };

    (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not resolve {host}:{port}"))
}
