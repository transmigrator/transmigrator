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
    // Create a new HTTPS client
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use mockito::mock;

    #[test]
    fn test_resolve_dns_success() {
        let _m = mock("GET", "/dns-query?name=example.com")
            .with_header("content-type", "application/dns-json")
            .with_body(r#"{"Answer":[{"data":"93.184.216.34"}]}"#)
            .create();

        let rt = Runtime::new().unwrap();
        let result = rt.block_on(resolve_dns("example.com")).unwrap();
        assert_eq!(result, "93.184.216.34");
    }

    #[test]
    fn test_resolve_dns_invalid_response() {
        let _m = mock("GET", "/dns-query?name=example.com")
            .with_header("content-type", "application/dns-json")
            .with_body(r#"{}"#)
            .create();

        let rt = Runtime::new().unwrap();
        let result = rt.block_on(resolve_dns("example.com"));
        assert!(matches!(result, Err(DnsError::InvalidResponse)));
    }

    #[test]
    fn test_resolve_dns_network_error() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(resolve_dns("invalid_domain"));
        assert!(matches!(result, Err(DnsError::Network(_))));
    }
}
