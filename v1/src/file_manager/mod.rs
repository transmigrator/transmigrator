// file_manager/mod.rs

pub fn window() -> Option<web_sys::Window> {
    web_sys::window()
}
use web_sys::HtmlInputElement;
use web_sys::ProgressEvent;
use js_sys::Array;
use web_sys::FileReader;
use web_sys::Blob;
use web_sys::Url;
use std::fs::File;
use std::io::{Read, Write};

pub struct FileManager {
    selected_file: Option<HtmlInputElement>,
}

impl FileManager {
    pub fn new() -> FileManager {
        FileManager {
            selected_file: None,
        }
    }

    pub fn select_file(&mut self) -> Result<(), JsValue> {
        let document = window().unwrap().unwrap().document().unwrap();
        let input = document.create_element("input")?.dyn_into::<HtmlInputElement>()?;
        input.set_attribute("type", "file")?;
        input.set_attribute("style", "display: none;")?;
        document.body().unwrap().append_child(&input)?;
        input.click();
        self.selected_file = Some(input);
        Ok(())
    }

    pub fn download_file(&self, data: &str, filename: &str) -> Result<(), JsValue> {
        let document = window().ok_or("no global `window` exists")?.document().ok_or(JsValue::from_str("should have a document on window"))?;
        let a = document.create_element("a")?;
        let blob = Blob::new_with_str_sequence(&Array::of1(&JsValue::from_str(data)))?;
        let url = Url::create_object_url_with_blob(&blob)?;
        a.set_attribute("href", &url)?;
        a.set_attribute("download", filename)?;
        a.set_attribute("style", "display: none;")?;
        document.body().unwrap().append_child(&a)?;
        a.click();
        Url::revoke_object_url(&url)?;
        Ok(())
    }

    pub fn upload_file(&self) -> Result<(), JsValue> {
        if let Some(input) = &self.selected_file {
            let files = input.files().unwrap();
            if files.length() > 0 {
                let file = files.get(0).unwrap();
                let reader = FileReader::new()?;
                let onloadend = Closure::wrap(Box::new(move |event: web_sys::ProgressEvent| {
                    let reader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                    let result = reader.result().unwrap();
                    let text = result.as_string().unwrap();
                    web_sys::console::log_1(&text.into());
                }) as Box<dyn FnMut(_)>);
                reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                reader.read_as_text(&file)?;
                onloadend.forget();
            }
        }
        Ok(())
    }
}

impl FileManager {
    pub fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_file(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}
