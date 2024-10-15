// browser/interface.rs

use wasm_bindgen::prelude::*;
use super::tab::Tab;

#[wasm_bindgen]
pub struct BrowserInterface {
    address_bar: String,
    tabs: Vec<Tab>,
    download_directory: String,
    proxy_list_path: Option<String>,
}

#[wasm_bindgen]
impl BrowserInterface {
    pub fn new() -> Self {
        BrowserInterface {
            address_bar: String::new(),
            tabs: vec![Tab::new(0)],
            download_directory: String::from("/downloads"), // Default download directory
            proxy_list_path: None,
        }
    }

    pub fn navigate(&mut self, input: &str) {
        if self.is_command(input) {
            let result = self.execute_command(input);
            // TODO: Display result in the current tab
        } else {
            // TODO: Implement actual navigation logic
            self.address_bar = input.to_string();
        }
    }

    pub fn open_tab(&mut self) {
        let new_id = self.tabs.len();
        self.tabs.push(Tab::new(new_id));
    }

    pub fn close_tab(&mut self, tab_id: usize) {
        self.tabs.retain(|tab| tab.id != tab_id);
    }

    fn is_command(&self, input: &str) -> bool {
        input.starts_with('/')
    }

    fn execute_command(&self, command: &str) -> String {
        // TODO: Implement more complex command execution logic
        match command {
            "/help" => String::from("Available commands: /help, /set_download_dir, /set_proxy_list"),
            _ => format!("Unknown command: {}", command),
        }
    }

    pub fn set_download_directory(&mut self, path: &str) {
        self.download_directory = path.to_string();
    }

    pub fn set_proxy_list(&mut self, path: &str) {
        self.proxy_list_path = Some(path.to_string());
    }

    // TODO: Implement methods for file upload and download
}
