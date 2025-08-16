extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;

mod log_utils;

use std::sync::{Arc, Mutex};

use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_wgpu";

mod dummy_gui {
    pub struct DummyGuiHandler {}

    use asn_gui_core::TAsnGuiHandler;

    impl TAsnGuiHandler for DummyGuiHandler {
        type GraphContext = asn_wgpu::GraphContext;
        type FrameContext = asn_wgpu::FrameContext;

        fn init(&mut self, gcx: Self::GraphContext) {
            let _ = gcx;
        }

        fn update(&mut self) {}

        fn draw(&mut self, fcx: Self::FrameContext) {
            let _ = fcx;
        }
    }

    pub fn get_handler()
    -> impl TAsnGuiHandler<GraphContext = asn_wgpu::GraphContext, FrameContext = asn_wgpu::FrameContext>
    {
        DummyGuiHandler {}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let h = dummy_gui::get_handler();

    let h_safe = Arc::new(Mutex::new(h));

    let r = asn_wgpu::get_manager(h_safe);

    asn_winit::run(r)
}
