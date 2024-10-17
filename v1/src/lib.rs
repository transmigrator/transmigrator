mod proxy;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use proxy::ProxyMesh;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::utils::fetch_proxies as fetch_proxies_util;
use js_sys::Function;

lazy_static! {
    static ref PROXY_MESH: Mutex<Option<ProxyMesh>> = Mutex::new(None);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize the console log
    console_log::init_with_level(log::Level::Debug).unwrap();
    Ok(())
}

#[wasm_bindgen]
pub fn fetch_proxies(url: &str, callback: Function) {
    let url = url.to_string();
    spawn_local(async move {
        match fetch_proxies_util(&url).await {
            Ok(proxies) => {
                log::info!("Fetched proxies: {:?}", proxies);
                let proxy_mesh = ProxyMesh::new(proxies);
                *PROXY_MESH.lock().unwrap() = Some(proxy_mesh);
                // Example usage of ProxyMesh and ProxyChain
                if let Some(ref mut mesh) = *PROXY_MESH.lock().unwrap() {
                    let chain = mesh.get_next_chain();
                    log::info!("Created ProxyChain with proxies: {:?}", chain.get_proxies());
                }
                let this = JsValue::NULL;
                let _ = callback.call1(&this, &JsValue::from_str("Success"));
            }
            Err(err) => {
                log::error!("Failed to fetch proxies: {:?}", err);
                let this = JsValue::NULL;
                let _ = callback.call1(&this, &JsValue::from_str("Error"));
            }
        }
    });
}
