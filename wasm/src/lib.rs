use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    log::info!("WASM module initialized");
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! (from Rust WASM)")
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
