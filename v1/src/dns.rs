use reqwest::Client;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnsError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Invalid DNS response")]
    InvalidResponse,
}

pub async fn resolve_dns(domain: &str) -> Result<String, DnsError> {
    let client = Client::new();
    let url = format!("https://cloudflare-dns.com/dns-query?name={}", domain);
    let response = client
        .get(&url)
        .header("Accept", "application/dns-json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(ip) = response["Answer"][0]["data"].as_str() {
        Ok(ip.to_string())
    } else {
        Err(DnsError::InvalidResponse)
    }
}
