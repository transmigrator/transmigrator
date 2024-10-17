mod proxy;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize the console log
    console_log::init_with_level(log::Level::Debug).unwrap();
    Ok(())
}

#[wasm_bindgen]
pub fn fetch_proxies(url: String) {
    spawn_local(async move {
        match utils::fetch_proxies(&url).await {
            Ok(proxies) => {
                log::info!("Fetched proxies: {:?}", proxies);
                // Handle the fetched proxies (e.g., store them in ProxyMesh)
            }
            Err(err) => {
                log::error!("Failed to fetch proxies: {:?}", err);
            }
        }
    });
}
