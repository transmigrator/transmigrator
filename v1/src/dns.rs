use reqwest::Client;
use serde_json::Value;

pub async fn resolve_dns(domain: &str) -> Result<String, Error> {
    let client = Client::new();
    let url = format!("https://dns.google/resolve?name={}", domain);
    let response = client.get(&url).send().await?.json::<Value>().await?;
    let ip = response["Answer"][0]["data"].as_str().unwrap().to_string();
    Ok(ip)
}