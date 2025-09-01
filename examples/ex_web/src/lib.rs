use asn_logger::log::info;
mod log_utils;

pub const LOG_MODULE_NAME: &str = "ex_web";

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[wasm_bindgen]
pub fn init_web_app() -> Result<(), JsValue> {
    log_utils::setup_log();

    // Простая инициализация для web
    info!("ASN Web App initialized");

    // Здесь можно добавить базовую логику без проблемных зависимостей
    spawn_local(async move {
        console::log_1(&"Async task started".into());

        // Простая асинхронная задача без задержки
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
