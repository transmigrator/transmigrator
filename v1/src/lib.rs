use wasm_bindgen::prelude::*;
use reqwest::Client;
use std::sync::Mutex;

#[wasm_bindgen]
pub struct ProxyChain {
    proxies: Vec<String>,
}

#[wasm_bindgen]
impl ProxyChain {
    pub fn new(proxies: Vec<String>) -> ProxyChain {
        ProxyChain { proxies }
    }

    pub fn add_proxy(&mut self, proxy: String) {
        self.proxies.push(proxy);
    }

    pub fn get_proxies(&self) -> Vec<String> {
        self.proxies.clone()
    }
}

#[wasm_bindgen]
pub struct ProxyMesh {
    chains: Vec<ProxyChain>,
}

#[wasm_bindgen]
impl ProxyMesh {
    pub fn new() -> ProxyMesh {
        ProxyMesh { chains: Vec::new() }
    }

    pub fn add_chain(&mut self, chain: ProxyChain) {
        self.chains.push(chain);
    }

    pub fn get_chains(&self) -> Vec<ProxyChain> {
        self.chains.clone()
    }
}

lazy_static::lazy_static! {
    static ref PROXIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[wasm_bindgen]
pub async fn fetch_proxies(url: String) -> Result<(), JsValue> {
    let client = Client::new();
    let response = client.get(&url).send().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    let text = response.text().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
    let proxies: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    let mut proxies_guard = PROXIES.lock().unwrap();
    *proxies_guard = proxies;
    Ok(())
}
