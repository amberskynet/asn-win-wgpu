extern crate asn_gui_core;
extern crate asn_logger;

mod data;

use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use data::LOG_MODULE_NAME;
use winit::event_loop::ControlFlow;

mod app_state;
mod winit_utils;

// do some re-export
pub use winit;

use crate::app_state::new_state;

pub type WinitWindow = winit::window::Window;

pub trait WinitRenderManager: TAsnRenderManager<Window = WinitWindow> {}

pub fn run<R>(r: R) -> Result<(), Box<dyn std::error::Error>>
where
    R: WinitRenderManager,
{
    m_info!("run()");

    let mut runner = new_state(r);

    let event_loop = winit::event_loop::EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))?;

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut runner);

    match result {
        Ok(_) => {
            m_info!("Application exited successfully");
            Ok(())
        }
        Err(e) => {
            asn_logger::m_error!("Application error: {}", e);
            Err(Box::new(std::io::Error::other(format!(
                "Application error: {e}"
            ))))
        }
    }
}
