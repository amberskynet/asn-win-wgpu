extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_winit;

mod log_utils;
use std::sync::Arc;

use asn_gui_core::{TAsnRenderManager, TAsnSurface};
use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_winit";

pub struct DummyRenderManager {}
impl TAsnSurface for DummyRenderManager {
    type AsnWindow = ();

    fn init(&mut self, w: Arc<Self::AsnWindow>) -> Result<(), Box<dyn std::error::Error>> {
        let _ = w;
        info(LOG_MODULE_NAME, "init()");
        Ok(())
    }
}

impl TAsnRenderManager for DummyRenderManager {
    type FrameContext = ();

    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>> {
        info(LOG_MODULE_NAME, "begin_frame()");
        Ok(())
    }

    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>> {
        let _ = fcx;
        info(LOG_MODULE_NAME, "end_frame()");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    info(LOG_MODULE_NAME, "hello from main()");

    let r = DummyRenderManager {};

    asn_winit::run(r)
}
