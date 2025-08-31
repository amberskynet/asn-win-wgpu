use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

extern crate asn_logger;

use asn_logger::*;

pub const LOG_MODULE_NAME: &str = "ex_web";

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    // Простая инициализация для web
    console::log_1(&"ASN Web App initialized".into());
    
    // Здесь можно добавить базовую логику без проблемных зависимостей
    spawn_local(async move {
        console::log_1(&"Async task started".into());
        // Простая задержка
        std::thread::sleep(std::time::Duration::from_millis(100));
        console::log_1(&"Async task completed".into());
    });

    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ASN Web!", name)
}

#[wasm_bindgen]
pub fn get_version() -> String {
    "ASN Web WGPU v0.1.0".to_string()
}
