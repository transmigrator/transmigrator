use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Headers, FetchEvent, ServiceWorkerGlobalScope, Response};

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
    let header_entries = js_sys::try_iter(&request.headers()).unwrap().unwrap();
    for header in header_entries {
        let header = header.unwrap();
        let header_array = js_sys::Array::from(&header);
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
}
