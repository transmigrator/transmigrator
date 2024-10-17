use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, Response, HtmlInputElement};
use std::sync::Mutex;

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
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let fetch_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(fetch_promise);

    let resp_value = future.await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text_promise = resp.text().unwrap();
    let text_future = JsFuture::from(text_promise);

    let text = text_future.await?.as_string().unwrap();
    let proxies: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    let mut proxies_guard = PROXIES.lock().unwrap();
    *proxies_guard = proxies;
    Ok(())
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let fetch_button = document.get_element_by_id("proxy-form").expect("should have #proxy-form on the page");

    let closure = Closure::wrap(Box::new(move || {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let url_input = document.get_element_by_id("url").expect("should have #url on the page");
        let url = url_input.dyn_into::<HtmlInputElement>().unwrap().value();

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = fetch_proxies(url).await {
                web_sys::console::log_1(&e);
            }
        });
    }) as Box<dyn FnMut()>);

    fetch_button.set_onsubmit(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}
