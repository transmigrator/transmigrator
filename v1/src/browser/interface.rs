// browser/interface.rs

use wasm_bindgen::prelude::*;
use web_sys::{Document, Window, HtmlInputElement, HtmlElement};
use super::tabs::Tab;
use crate::browser::tabs::Tab;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // Display loading screen
    let body = document.body().expect("document should have a body");
    body.set_inner_html("<div id='loading' style='background-color: black; height: 100vh; display: flex; align-items: center; justify-content: center;'>
                            <img src='path/to/loading.svg' alt='Loading...' />
                         </div>");

    // Remove loading screen after 3 seconds
    let closure = Closure::wrap(Box::new(move || {
        let loading_div = document.get_element_by_id("loading").unwrap();
        loading_div.remove();
        // Initialize the main interface here
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

pub struct Browser {
    tabs: Vec<Tab>,
    current_tab: usize,
}

impl Browser {
    pub fn new() -> Self {
        Browser {
            tabs: Vec::new(),
            current_tab: 0,
        }
    }

    pub fn open_tab(&mut self, url: String) {
        let id = self.tabs.len();
        let tab = Tab::new(id, url);
        self.tabs.push(tab);
        self.current_tab = id;
    }

    pub fn close_tab(&mut self, id: usize) {
        self.tabs.retain(|tab| tab.id != id);
        if self.current_tab == id {
            self.current_tab = if self.tabs.is_empty() { 0 } else { self.tabs.len() - 1 };
        }
    }

    pub fn switch_tab(&mut self, id: usize) {
        if (id < self.tabs.len()) {
            self.current_tab = id;
        }
    }

    pub fn get_current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.current_tab)
    }

    pub fn render(&self) {
        if let Some(tab) = self.get_current_tab() {
            // Render the current tab's content
            println!("Rendering tab with URL: {}", tab.url);
        } else {
            println!("No tabs open.");
        }
    }
}
