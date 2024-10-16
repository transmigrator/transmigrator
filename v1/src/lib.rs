// lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, Transmigrator v1!".to_string()
}