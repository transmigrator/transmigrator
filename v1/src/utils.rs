use reqwest::Error;

pub async fn fetch_proxies(url: &str) -> Result<Vec<String>, Error> {
    let response = reqwest::get(url).await?;
    let proxies = response.text().await?;
    Ok(proxies.lines().map(|line| line.to_string()).collect())
}