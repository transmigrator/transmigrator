use wasm_bindgen::prelude::*;
use web_sys::{Request, Response, ResponseInit, Headers, FetchEvent};

#[wasm_bindgen]
pub fn handle_fetch(event: FetchEvent) {
    let request = event.request();
    let url = request.url();

    // Modify the request to route through the proxy
    let proxy_url = format!("http://your-proxy-server:port?target={}", url);
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
    for header in request.headers().entries() {
        let header = header.unwrap();
        headers.append(&header[0], &header[1]).unwrap();
    }
    init.headers(&headers.into());

    let modified_request = Request::new_with_str_and_init(&proxy_url, &init).unwrap();

    let response_promise = web_sys::window().unwrap().fetch_with_request(&modified_request);
    let response = wasm_bindgen_futures::JsFuture::from(response_promise);

    event.respond_with(response);
}
