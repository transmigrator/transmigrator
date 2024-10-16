// browser/interface.rs

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};
use crate::browser::tabs::Tab;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create and display the loading screen
    let loading_screen = create_loading_screen(&document)?;
    body.append_child(&loading_screen)?;

    // Remove loading screen after 3 seconds
    let closure = Closure::wrap(Box::new(move || {
        loading_screen.remove();
        // Create the minimalistic interface
        create_interface(&document).unwrap();
    }) as Box<dyn Fn()>);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000)?;
    closure.forget();

    Ok(())
}

fn create_loading_screen(document: &Document) -> Result<Element, JsValue> {
    let loading_screen = document.create_element("div")?;
    loading_screen.set_inner_html(r#"<svg>...</svg>"#); // Replace with your SVG content
    loading_screen.set_attribute("style", "height: 100vh; width: 100vw; background: black; display: flex; justify-content: center; align-items: center;")?;
    Ok(loading_screen)
}

fn create_interface(document: &Document) -> Result<(), JsValue> {
    let body = document.body().unwrap();
    body.set_attribute("style", "background: black; color: white; font-family: sans-serif;")?;

    // Create address bar
    let address_bar = document.create_element("input")?;
    address_bar.set_attribute("type", "text")?;
    address_bar.set_attribute("placeholder", "Enter URL or IP")?;
    address_bar.set_attribute("style", "width: 80%; padding: 10px; margin: 20px; border: none; background: #333; color: white;")?;
    body.append_child(&address_bar)?;

    // Create open tab button
    let open_tab_button = document.create_element("button")?;
    open_tab_button.set_inner_html("Open Tab");
    open_tab_button.set_attribute("style", "padding: 10px 20px; margin: 20px; border: none; background: #333; color: white; cursor: pointer;")?;
    body.append_child(&open_tab_button)?;

    // Create close tab button
    let close_tab_button = document.create_element("button")?;
    close_tab_button.set_inner_html("Close Tab");
    close_tab_button.set_attribute("style", "padding: 10px 20px; margin: 20px; border: none; background: #333; color: white; cursor: pointer;")?;
    body.append_child(&close_tab_button)?;

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
