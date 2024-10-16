// browser/interface.rs

use wasm_bindgen::prelude::*;
use web_sys::{Document, Window, HtmlInputElement, HtmlElement};
use super::tabs::Tab;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // Display loading screen
    display_loading_screen(&document)?;

    // Initialize the browser interface
    init_browser_interface(&document)?;

    Ok(())
}

fn display_loading_screen(document: &Document) -> Result<(), JsValue> {
    let body = document.body().expect("document should have a body");
    let loading_screen = document.create_element("div")?;
    loading_screen.set_inner_html("<svg>...</svg>");
    loading_screen.set_attribute("style", "height: 100%; background: black;")?;
    body.append_child(&loading_screen)?;

    // Remove loading screen after 3 seconds
    let closure = Closure::wrap(Box::new(move || {
        loading_screen.remove();
    }) as Box<dyn Fn()>);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000)?;
    closure.forget();

    Ok(())
}

fn init_browser_interface(document: &Document) -> Result<(), JsValue> {
    // Create address bar
    let address_bar = document.create_element("input")?;
    address_bar.set_attribute("type", "text")?;
    address_bar.set_attribute("placeholder", "Enter URL or IP")?;
    document.body().unwrap().append_child(&address_bar)?;

    // Add event listener for address bar
    let address_bar_clone = address_bar.clone();
    let closure = Closure::wrap(Box::new(move || {
        let input: HtmlInputElement = address_bar_clone.dyn_into().unwrap();
        let value = input.value();
        if is_command(&value) {
            execute_command(&value);
        } else {
            load_url(&value);
        }
    }) as Box<dyn Fn()>);
    address_bar.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn is_command(input: &str) -> bool {
    // Implement command detection logic
    input.starts_with("cmd:")
}

fn execute_command(command: &str) {
    // Implement command execution logic
    web_sys::console::log_1(&format!("Executing command: {}", command).into());
}

fn load_url(url: &str) {
    // Implement URL loading logic
    web_sys::console::log_1(&format!("Loading URL: {}", url).into());
}

#[wasm_bindgen]
pub struct BrowserInterface {
    address_bar: String,
    tabs: Vec<Tab>,
}
