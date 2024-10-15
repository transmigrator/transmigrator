// browser/interface.rs

use wasm_bindgen::prelude::*;
use super::tab::Tab;

#[wasm_bindgen]
pub struct BrowserInterface {
    address_bar: String,
    tabs: Vec<Tab>,
}

#[wasm_bindgen]
impl BrowserInterface {
    pub fn new() -> Self {
        BrowserInterface {
            address_bar: String::new(),
            tabs: vec![Tab::new(0)],
        }
    }

    pub fn navigate(&mut self, url: &str) {
        // TODO: Implement navigation logic
        self.address_bar = url.to_string();
    }

    pub fn open_tab(&mut self) {
        let new_id = self.tabs.len();
        self.tabs.push(Tab::new(new_id));
    }

    pub fn close_tab(&mut self, tab_id: usize) {
        self.tabs.retain(|tab| tab.id != tab_id);
    }

    pub fn execute_command(&self, command: &str) -> String {
        // TODO: Implement command execution logic
        format!("Executing command: {}", command)
    }
}