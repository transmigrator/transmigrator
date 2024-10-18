use reqwest::Error;
use std::sync::Mutex;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Function;
use wasm_bindgen_futures::spawn_local;

lazy_static! {
    static ref PROXIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub async fn fetch_proxies_util(url: &str, callback: Function) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(Error::new(reqwest::ErrorKind::Request, Some("Failed to fetch proxies".to_string())));
    }
    let proxies = response.text().await?;
    let mut proxies_vec = PROXIES.lock().unwrap();
    *proxies_vec = proxies.lines().map(|line| line.to_string()).collect();
    let js_proxies = JsValue::from_str(&serde_json::to_string(&*proxies_vec).unwrap());
    callback.call1(&JsValue::NULL, &js_proxies).unwrap();
    Ok(())
}

#[wasm_bindgen]
pub async fn fetch_proxies(url: &str, callback: Function) {
    let url = url.to_string();
    spawn_local(async move {
        match fetch_proxies_util(&url, callback).await {
            Ok(_) => {
                log::info!("Fetched proxies successfully");
            }
            Err(err) => {
                log::error!("Failed to fetch proxies: {:?}", err);
            }
        }
    });
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
    clear_proxies();
}
