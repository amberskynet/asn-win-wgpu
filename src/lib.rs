extern crate asn_logger;

use asn_logger::*;

use crate::log_utils::setup_log;
mod log_utils;

pub const LOG_MODULE_NAME: &str = "asn_web_wgpu";

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    setup_log();

    // Простая инициализация для web без проблемных зависимостей
    console::log_1(&"ASN Web WGPU App initialized".into());
    console::log_1(&"Web app is ready!".into());

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init_web_app() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();
    println!("ASN Web WGPU App initialized (native mode)");
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ASN Web WGPU!", name)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ASN Web WGPU!", name)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_version() -> String {
    "ASN Web WGPU v0.1.0".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_version() -> String {
    "ASN Web WGPU v0.1.0".to_string()
}
