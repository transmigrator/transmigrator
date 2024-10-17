use reqwest::Error;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROXIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub async fn fetch_proxies(url: &str) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    let proxies = response.text().await?;
    let mut proxies_vec = PROXIES.lock().unwrap();
    *proxies_vec = proxies.lines().map(|line| line.to_string()).collect();
    Ok(())
}

pub fn get_proxies() -> Vec<String> {
    let proxies_vec = PROXIES.lock().unwrap();
    proxies_vec.clone()
}

pub fn clear_proxies() {
    let mut proxies_vec = PROXIES.lock().unwrap();
    proxies_vec.clear();
}
