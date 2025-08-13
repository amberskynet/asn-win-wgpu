extern crate asn_gui_core;
extern crate asn_logger;

mod data;
mod runner_dataset;

use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use data::LOG_MODULE_NAME;
use winit::event_loop::ControlFlow;

// do some re-export
pub use winit;

use crate::runner_dataset::RunnerDataset;

pub type WinitWindow = winit::window::Window;

pub fn run<R>(r: R) -> Result<(), Box<dyn std::error::Error>>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    info(LOG_MODULE_NAME, "run()");

    let mut runner = RunnerDataset::new(r);

    let event_loop = winit::event_loop::EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))?;

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(&mut runner);

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
