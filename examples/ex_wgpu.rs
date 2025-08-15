extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;

mod log_utils;

use asn_gui_core::TAsnGuiHandler;
use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_wgpu";

pub struct DummyGuiHandler {}

impl TAsnGuiHandler for DummyGuiHandler {
    type GraphContext = ();
    type FrameContext = asn_wgpu::FrameContext;

    fn init(&mut self, gcx: Self::GraphContext) {
        let _ = gcx;
    }

    fn update(&mut self) {}

    fn draw(&mut self, fcx: Self::FrameContext) {
        let _ = fcx;
    }
}

fn get_dummy_gui_handler() -> impl TAsnGuiHandler<FrameContext = asn_wgpu::FrameContext> {
    return DummyGuiHandler {};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let r = asn_wgpu::get_manager();
    let h = get_dummy_gui_handler();

    asn_winit::run(r, h)
}
