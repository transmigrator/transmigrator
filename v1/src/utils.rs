use reqwest::Error;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROXIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub async fn fetch_proxies(url: &str) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(Error::new(reqwest::StatusCode::BAD_REQUEST, "Failed to fetch proxies"));
    }
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

pub fn clear_proxies_at_end_of_session() {
    // This function should be called at the end of the session to clear the proxies
    clear_proxies();
}
