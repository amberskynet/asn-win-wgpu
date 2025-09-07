use asn_logger::*;
use std::sync::{Arc, Mutex};

use crate::gui_handler;

#[allow(dead_code)]
const LOG_MODULE_NAME: &str = "gui_wgpu";

#[allow(dead_code)]
pub fn run_gui() {
    let h = gui_handler::get_handler();

    let h_safe = Arc::new(Mutex::new(h));

    let r = asn_wgpu::get_manager(h_safe);

    match asn_winit::run(r) {
        Ok(_) => {
            m_info!("Application finished successfully");
        }
        Err(e) => {
            m_error!("Application failed with error: {}", e);
        }
    }
}
