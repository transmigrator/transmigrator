// browser/interface.rs

use wasm_bindgen::prelude::*;
use web_sys::{Document, Window};
use super::tab::Tab;

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
    // Implement address bar, tab management, and file manager initialization here
    Ok(())
}

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
