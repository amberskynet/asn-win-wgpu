extern crate asn_gui_core;
extern crate asn_logger;

mod data;
mod error;
mod keyboard_handler;

use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use data::LOG_MODULE_NAME;
use winit::event_loop::ControlFlow;

mod app_state;
mod asn_winit_state;
mod winit_utils;

// do some re-export
pub use error::AsnWinitError;
pub use winit;

use app_state::new_state;
use error::event_loop_creation_error;

pub type WinitWindow = winit::window::Window;

pub trait WinitRenderManager: TAsnRenderManager<Window = WinitWindow> {}

// Blanket implementation
impl<T> WinitRenderManager for T where T: TAsnRenderManager<Window = WinitWindow> {}

pub fn run<R>(r: R) -> Result<(), AsnWinitError>
where
    R: WinitRenderManager,
{
    m_info!("run()");

    let mut runner = new_state(r);

    let event_loop = winit::event_loop::EventLoop::new()
        .map_err(|e| event_loop_creation_error(format!("Failed to create event loop: {e}")))?;

    let proxy = Some(event_loop.create_proxy());

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut runner);

    match result {
        Ok(_) => {
            m_info!("Application exited successfully");
            Ok(())
        }
        Err(e) => {
            asn_logger::m_error!("Application error: {}", e);
            Err(error::event_loop_creation_error(format!(
                "Application error: {e}"
            )))
        }
    }
}
