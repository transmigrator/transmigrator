use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

pub mod proxy;

#[wasm_bindgen]
#[derive(Clone)]
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

// Example function to fetch proxies (error handling added)
#[wasm_bindgen]
pub async fn fetch_proxies(url: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let request = Request::new_with_str(url)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str("Network response was not ok"));
    }

    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}
