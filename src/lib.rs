use wasm_bindgen::prelude::*;

/// Hàm xuất ra WASM, nhận input và trả lời
#[wasm_bindgen]
pub fn chat(message: &str) -> String {
    format!("LifeTalk AI Agent trả lời: {}", message)
}
