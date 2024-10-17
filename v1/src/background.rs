use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&"Extension installed".into());
    Ok(())
}