use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Headers, Response};
use js_sys::{Promise, Array};
use std::sync::{Arc, Mutex};
use crate::proxy::{ProxyManager, ProxyChain, ProxyMesh}; // Import the ProxyManager, ProxyChain, and ProxyMesh
use rand::Rng;
use aes_gcm::Aes256Gcm; // Or any other cryptographic library
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};

// Initialize the ProxyManager
lazy_static::lazy_static! {
    static ref PROXY_MANAGER: Arc<ProxyManager> = Arc::new(ProxyManager::new());
}

// Entry point for the WebAssembly module
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize the proxy list (this should be dynamically fetched)
    let proxies = fetch_proxies();
    PROXY_MANAGER.refresh_proxies(proxies);

    Ok(())
}

#[wasm_bindgen]
pub async fn handle_fetch(url: String, method: String, headers: Vec<(String, String)>, body: Vec<u8>) -> Result<JsValue, JsValue> {
    // Refresh proxies if needed
    let proxies = fetch_proxies();
    PROXY_MANAGER.check_and_refetch_proxies(proxies);

    let packet_size = 1280;
    let packet_count = (body.len() + packet_size - 1) / packet_size;

    let proxy_mesh = PROXY_MANAGER.create_proxy_mesh(packet_count);

    // Encrypt the body
    let key = generate_random_key();
    let nonce = generate_random_nonce();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
    let encrypted_body = cipher.encrypt(GenericArray::from_slice(&nonce), body.as_ref()).expect("encryption failure!");

    // Split the encrypted body into 1280-byte packets
    let packets = split_into_packets(&encrypted_body, packet_size);

    for (i, packet) in packets.iter().enumerate() {
        if let Some(proxy_chain) = proxy_mesh.get_chains().get(i) {
            let proxy_url = format!("{}?target={}", proxy_chain.get_proxies()[0], url);
            let mut init = web_sys::RequestInit::new();
            init.method(&method);
            init.mode(web_sys::RequestMode::Cors);
            init.credentials(web_sys::RequestCredentials::SameOrigin);
            init.redirect(web_sys::RequestRedirect::Follow);

            let headers_obj = web_sys::Headers::new().unwrap();
            for (key, value) in &headers {
                headers_obj.append(&key, &value).unwrap();
            }
            init.headers(&headers_obj.into());

            let modified_request = Request::new_with_str_and_init(&proxy_url, &init).unwrap();
            modified_request.body(Some(&js_sys::Uint8Array::from(packet.as_slice())));

            let response_promise = web_sys::window().unwrap().fetch_with_request(&modified_request);
            let future = JsFuture::from(response_promise);

            wasm_bindgen_futures::spawn_local(async move {
                match future.await {
                    Ok(response) => {
                        // Handle the response
                    },
                    Err(err) => {
                        web_sys::console::error_1(&err);
                    }
                }
            });
        }
    }

    Ok(JsValue::from_str("Request sent through proxy chain"))
}

fn generate_random_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..32).map(|_| rng.gen()).collect()
}

fn generate_random_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..12).map(|_| rng.gen()).collect()
}

fn split_into_packets(data: &[u8], packet_size: usize) -> Vec<Vec<u8>> {
    data.chunks(packet_size).map(|chunk| chunk.to_vec()).collect()
}

fn fetch_proxies() -> Vec<String> {
    // This function should dynamically fetch the list of proxies
}
