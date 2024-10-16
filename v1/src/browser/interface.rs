// browser/interface.rs

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlInputElement};
use crate::file_manager::FileManager;

#[wasm_bindgen]
pub struct Browser {
    file_manager: FileManager,
}

#[wasm_bindgen]
impl Browser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Browser {
        Browser {
            file_manager: FileManager::new(),
        }
    }

    pub fn start(&self) {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        // Create a full-height div with a black background
        let loading_div = document.create_element("div").unwrap();
        loading_div.set_attribute("style", "height: 100vh; background-color: black; display: flex; justify-content: center; align-items: center;").unwrap();

        // Create an SVG element
        let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap();
        svg.set_attribute("width", "100").unwrap();
        svg.set_attribute("height", "100").unwrap();
        svg.set_inner_html("<circle cx='50' cy='50' r='40' stroke='white' stroke-width='3' fill='none' />");

        loading_div.append_child(&svg).unwrap();
        body.append_child(&loading_div).unwrap();

        // Remove the loading screen after 3 seconds
        let closure = Closure::wrap(Box::new(move || {
            loading_div.remove();
        }) as Box<dyn Fn()>);

        window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000).unwrap();
        closure.forget();
    }

    pub fn select_file(&self) -> Result<(), JsValue> {
        self.file_manager.select_file()
    }

    pub fn download_file(&self, data: &str, filename: &str) -> Result<(), JsValue> {
        self.file_manager.download_file(data, filename)
    }

    pub fn upload_file(&self) -> Result<(), JsValue> {
        self.file_manager.upload_file()
    }
}
