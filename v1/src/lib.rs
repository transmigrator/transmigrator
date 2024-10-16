// lib.rs


pub mod browser;
pub mod crypto;
pub mod file_manager;
pub mod network;

use browser::interface::{start, Browser};
use file_manager::FileManager;
use network::ProxyMesh;

pub struct Transmigrator {
    browser: Browser,
    network: ProxyMesh,
    file_manager: FileManager,
}

impl Transmigrator {
    pub fn new() -> Self {
        Self {
            browser: Browser::new(),
            network: ProxyMesh::new(),
            file_manager: FileManager::new(),
        }
    }

    pub fn run(&mut self) {
        // Main execution logic
        web_sys::console::log_1(&"Transmigrator is running".into());
    }
}

pub fn main() -> Result<(), JsValue> {
    start();
    Ok(())
}
