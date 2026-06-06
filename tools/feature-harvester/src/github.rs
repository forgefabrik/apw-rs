//! Minimal async GitHub REST client for the feature-harvester tool.
//!
//! Only the endpoints we need:
//!   GET /repos/{owner}/{repo}/git/trees/{ref}?recursive=1
//!   GET /repos/{owner}/{repo}/git/blobs/{sha}
//!
//! Rate-limit aware: parses `X-RateLimit-Remaining` and surfaces 403/429 as
//! structured errors with the reset timestamp.

use anyhow::{anyhow, Context, Result};
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;

pub struct GitHubClient {
    http: Client,
    #[allow(dead_code)]
    token: Option<String>,
    base: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TreeEntry {
    pub path: String,
    #[allow(dead_code)]
    pub mode: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub sha: String,
    #[allow(dead_code)]
    pub size: Option<u64>,
    #[allow(dead_code)]
    pub url: String,
}

impl TreeEntry {
    pub fn is_file(&self) -> bool {
        self.kind == "blob"
    }
}

#[derive(Debug, Deserialize)]
struct TreeResponse {
    tree: Vec<TreeEntry>,
    truncated: bool,
}

impl GitHubClient {
    #[allow(dead_code)]
    pub fn new(token: Option<String>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("feature-harvester/0.1"),
        );
        if let Some(t) = &token {
            if let Ok(v) = HeaderValue::from_str(&format!("Bearer {t}")) {
                headers.insert("Authorization", v);
            }
        }
        let http = Client::builder()
            .default_headers(headers)
            .build()
            .expect("reqwest client builds");
        Self {
            http,
            token,
            base: "https://api.github.com".to_string(),
        }
    }

    pub async fn list_tree(&self, repo: &str, refname: &str) -> Result<Vec<TreeEntry>> {
        let url = format!(
            "{}/repos/{}/git/trees/{}?recursive=1",
            self.base, repo, refname
        );
        let resp = self.get(&url).await?;
        let parsed: TreeResponse = resp
            .error_for_status()
            .context("GitHub trees endpoint")?
            .json()
            .await
            .context("parsing trees response")?;
        if parsed.truncated {
            eprintln!(
                "warning: tree response was truncated ({}); set --limit to scan a subset",
                parsed.tree.len()
            );
        }
        Ok(parsed.tree)
    }

    pub async fn get_blob(&self, repo: &str, sha: &str) -> Result<Vec<u8>> {
        let url = format!("{}/repos/{}/git/blobs/{}", self.base, repo, sha);
        let resp = self.get(&url).await?;
        let status = resp.status();
        let headers = resp.headers().clone();
        let body: BlobResponse = resp
            .error_for_status()
            .context("GitHub blob endpoint")?
            .json()
            .await
            .context("parsing blob response")?;
        check_rate_limit(&headers, status);
        let bytes = body
            .content
            .decode()
            .ok_or_else(|| anyhow!("blob {sha} not base64"))?;
        Ok(bytes)
    }

    async fn get(&self, url: &str) -> Result<Response> {
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        let status = resp.status();
        let headers = resp.headers().clone();
        if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
            let reset = headers
                .get("x-ratelimit-reset")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0);
            let remaining = headers
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(-1);
            return Err(anyhow!(
                "GitHub returned {status} (rate limit remaining={remaining}, reset epoch={reset})"
            ));
        }
        Ok(resp)
    }
}

fn check_rate_limit(headers: &HeaderMap, status: StatusCode) {
    if status == StatusCode::OK {
        if let Some(remaining) = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<i64>().ok())
        {
            if remaining < 5 {
                eprintln!("warning: GitHub rate-limit remaining = {remaining} (consider --token)");
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct BlobResponse {
    content: BlobContent,
    #[allow(dead_code)]
    encoding: String,
}

#[derive(Debug, Deserialize)]
struct BlobContent(String);

impl BlobContent {
    fn decode(&self) -> Option<Vec<u8>> {
        base64::engine::general_purpose::STANDARD
            .decode(&self.0)
            .ok()
    }
}
