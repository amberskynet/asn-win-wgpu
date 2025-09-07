use asn_logger::*;
pub const LOG_MODULE_NAME: &str = "DummyGuiHandler";

pub struct DummyGuiHandler {}

use asn_gui_core::TAsnGuiHandler;
use asn_wgpu::{WgpuGuiHandler, render_manager};

impl TAsnGuiHandler for DummyGuiHandler {
    fn init(&mut self, gcx: &Self::GraphContext) {
        let _ = gcx;
        m_info!("init");
    }

    fn update(&mut self, gcx: &Self::GraphContext) {
        let _ = gcx;
        // m_info!("update");
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        let _ = fcx;
        // m_info!("draw");
    }

    type GraphContext = render_manager::WgpuGraphContext;
    type FrameContext = render_manager::WgpuFrameContext;
}

pub fn get_handler() -> impl WgpuGuiHandler {
    DummyGuiHandler {}
}
