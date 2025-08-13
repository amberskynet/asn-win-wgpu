extern crate asn_gui_core;
extern crate asn_logger;

mod data;
mod winit_app;

use asn_gui_core::*;
use asn_logger::*;
use data::LOG_MODULE_NAME;
use winit::event_loop::ControlFlow;

use crate::winit_app::WinitApp;

pub fn run(config: &AsnGuiWindowConfig) -> Result<(), Box<dyn std::error::Error>> {
    let _ = config;
    info(LOG_MODULE_NAME, "run()");

    let mut app = WinitApp::new();

    let event_loop = winit::event_loop::EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))?;

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut app);

    match result {
        Ok(_) => {
            info(LOG_MODULE_NAME, "Application exited successfully");
            Ok(())
        }
        Err(e) => {
            error(LOG_MODULE_NAME, &e.to_string());
            Err(Box::new(std::io::Error::other(format!(
                "Application error: {e}"
            ))))
        }
    }
}
