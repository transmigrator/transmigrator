// lib.rs

use wasm_bindgen::prelude::*;

mod browser;
mod crypto;
mod file_manager;
mod network;

use browser::Browser;
use file_manager::FileManager;
use network::ProxyMesh;

#[wasm_bindgen]
pub struct Transmigrator {
    browser: Browser,
    network: ProxyMesh,
    file_manager: FileManager,
}

#[wasm_bindgen]
impl Transmigrator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            browser: Browser::new(),
            network: ProxyMesh::new(),
            file_manager: FileManager::new(),
        }
    }

    pub fn run(&mut self) {
        // Main execution logic
        // For now, we'll just log a message
        web_sys::console::log_1(&"Transmigrator is running".into());
    }
}

// This is the entry point of the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    // Any initialization code can go here
    web_sys::console::log_1(&"Transmigrator initialized".into());
}
