//! apw-server
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, clippy::all, clippy::pedantic)]

pub fn name() -> &'static str {
    "apw-server"
}

#[derive(Clone)]
pub struct ServerConfig {
    pub bind: std::net::SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: ([127, 0, 0, 1], 8080).into(),
        }
    }
}

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/health", axum::routing::get(|| async { "ok" }))
        .route("/status", axum::routing::get(|| async { "ok" }))
        .route(
            "/metrics",
            axum::routing::get(|| async {
                "# HELP apw_up 1\n# TYPE apw_up gauge\napw_up 1\n".to_string()
            }),
        )
}

pub async fn serve(config: ServerConfig) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(config.bind).await?;
    axum::serve(listener, routes()).await?;
    Ok(())
}
