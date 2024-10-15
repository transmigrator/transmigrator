// browser/mod.rs

use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub struct Browser {
    address_bar: String,
}

#[wasm_bindgen]
impl Browser {
    pub fn new() -> Self {
        Self {
            address_bar: String::new(),
        }
    }

    pub fn set_address(&mut self, address: String) {
        self.address_bar = address;
    }

    pub fn navigate(&self) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // For now, just display the address in the body
        body.set_inner_html(&format!("Navigating to: {}", self.address_bar));

        Ok(())
    }

    pub fn detect_command(&self) -> bool {
        self.address_bar.starts_with('/')
    }

    pub fn execute_command(&self) -> Result<(), JsValue> {
        if self.detect_command() {
            let window = window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let body = document.body().expect("document should have a body");
            body.set_inner_html(&format!("Executing command: {}", self.address_bar));
        }
        Ok(())
    }
}