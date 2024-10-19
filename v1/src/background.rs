use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Headers, FetchEvent, ServiceWorkerGlobalScope, Response};
use js_sys::{Promise, Array};
use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;

struct ProxyChain {
    proxies: Vec<String>,
}

impl ProxyChain {
    pub fn new(proxies: Vec<String>) -> Self {
        ProxyChain { proxies }
    }

    pub fn get_proxies(&self) -> &Vec<String> {
        &self.proxies
    }
}

struct ProxyMesh {
    proxy_queue: Arc<Mutex<Vec<String>>>,
}

impl ProxyMesh {
    pub fn new(proxies: Vec<String>) -> Self {
        ProxyMesh {
            proxy_queue: Arc::new(Mutex::new(proxies)),
        }
    }

    pub fn get_next_chain(&self) -> Option<ProxyChain> {
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        if proxy_queue.len() < 3 {
            return None;
        }
        let mut proxies = vec![];
        for _ in 0..3 {
            if let Some(proxy) = proxy_queue.pop() {
                proxies.push(proxy);
            }
        }
        proxies.shuffle(&mut rand::thread_rng());
        Some(ProxyChain::new(proxies))
    }
}

// Entry point for the WebAssembly module
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Register the fetch event listener
    let global_scope = js_sys::global().unchecked_into::<ServiceWorkerGlobalScope>();
    let fetch_event_listener = Closure::wrap(Box::new(move |event: FetchEvent| {
        handle_fetch(event);
    }) as Box<dyn FnMut(_)>);

    global_scope.add_event_listener_with_callback("fetch", fetch_event_listener.as_ref().unchecked_ref())?;
    fetch_event_listener.forget(); // Prevent the closure from being garbage collected

    Ok(())
}

#[wasm_bindgen]
pub fn handle_fetch(event: FetchEvent) {
    let request = event.request();
    let url = request.url();

    // Example proxy list
    let proxies = vec![
        "http://proxy1:port".to_string(),
        "http://proxy2:port".to_string(),
        "http://proxy3:port".to_string(),
        "http://proxy4:port".to_string(),
    ];

    let proxy_mesh = ProxyMesh::new(proxies);
    if let Some(proxy_chain) = proxy_mesh.get_next_chain() {
        let proxy_url = format!("{}?target={}", proxy_chain.get_proxies()[0], url);
        let mut init = web_sys::RequestInit::new();
        init.method(request.method().as_str());
        init.mode(web_sys::RequestMode::Cors);
        init.credentials(web_sys::RequestCredentials::SameOrigin);
        init.redirect(web_sys::RequestRedirect::Follow);
        init.referrer(request.referrer().as_str());
        init.referrer_policy(request.referrer_policy().as_str());
        init.integrity(request.integrity().as_str());
        init.cache(request.cache().as_str());

        let headers = Headers::new().unwrap();
        let header_entries = js_sys::try_iter(&request.headers()).unwrap().unwrap();
        for header in header_entries {
            let header = header.unwrap();
            let header_array = Array::from(&header);
            headers.append(&header_array.get(0).as_string().unwrap(), &header_array.get(1).as_string().unwrap()).unwrap();
        }
        init.headers(&headers.into());

        let modified_request = Request::new_with_str_and_init(&proxy_url, &init).unwrap();

        let response_promise = web_sys::window().unwrap().fetch_with_request(&modified_request);
        let future = JsFuture::from(response_promise);

        wasm_bindgen_futures::spawn_local(async move {
            match future.await {
                Ok(response) => event.respond_with(Promise::resolve(&response)),
                Err(err) => {
                    let error_response = Response::error().unwrap();
                    event.respond_with(Promise::resolve(&error_response));
                    web_sys::console::error_1(&err);
                }
            }
        });
    } else {
        let error_response = Response::error().unwrap();
        event.respond_with(Promise::resolve(&error_response));
    }
}
