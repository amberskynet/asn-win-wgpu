extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_winit;

mod log_utils;
use std::sync::Arc;

use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use asn_winit::WinitWindow;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_winit";

pub struct DummyRenderManager {}

impl TAsnRenderManager for DummyRenderManager {
    type FrameContext = ();
    type Window = WinitWindow;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>> {
        info(LOG_MODULE_NAME, "begin_frame()");
        Ok(())
    }

    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>> {
        let _ = fcx;
        info(LOG_MODULE_NAME, "end_frame()");
        Ok(())
    }

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let _ = w;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    info(LOG_MODULE_NAME, "hello from main()");

    let r = DummyRenderManager {};

    asn_winit::run(r)
}
