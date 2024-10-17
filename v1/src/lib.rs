mod proxy;
mod utils;
mod packet;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use proxy::ProxyMesh;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::utils::fetch_proxies as fetch_proxies_util;
use js_sys::Function;
use packet::Packet;

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
            Ok(_) => {
                log::info!("Fetched proxies successfully");
                let mut proxy_mesh = ProxyMesh::new();
                proxy_mesh.construct_chains();
                *PROXY_MESH.lock().unwrap() = Some(proxy_mesh);
                // Call the JavaScript callback function to notify that proxies are fetched
                callback.call0(&JsValue::NULL).unwrap();
            }
            Err(err) => {
                log::error!("Failed to fetch proxies: {:?}", err);
            }
        }
    });
}

#[wasm_bindgen]
pub fn send_packet(data: Vec<u8>, key: Vec<u8>) {
    let mut packet = Packet::new(data, key);
    if let Some(ref mut proxy_mesh) = *PROXY_MESH.lock().unwrap() {
        proxy_mesh.send_packet(&mut packet);
    } else {
        log::error!("ProxyMesh is not initialized");
    }
}
