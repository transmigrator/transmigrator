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
    // Create a new HTTP client
    let client = Client::new();
    
    // Construct the DNS query URL
    let url = format!("https://cloudflare-dns.com/dns-query?name={}", domain);
    
    // Send the DNS query request
    let response = client
        .get(&url)
        .header("Accept", "application/dns-json")
        .send()
        .await?
        .json::<Value>()
        .await?;
    
    // Parse the DNS response
    if let Some(answers) = response["Answer"].as_array() {
        for answer in answers {
            if let Some(ip) = answer["data"].as_str() {
                return Ok(ip.to_string());
            }
        }
    }
    
    // Return an error if the response is invalid
    Err(DnsError::InvalidResponse)
}
