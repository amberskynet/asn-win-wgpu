use asn_logger::*;
pub const LOG_MODULE_NAME: &str = "DummyGuiHandler";

mod gui_handler_state;
mod map_utils;
use asn_gui_core::TAsnGuiHandler;
use asn_wgpu::{WgpuGuiHandler, render_manager};
use gui_handler_state::GuiHandlerState;

enum WebGuiHandler {
    Zero,
    Loaded(GuiHandlerState),
}

impl TAsnGuiHandler for WebGuiHandler {
    type GraphContext = render_manager::WgpuGraphContext;
    type FrameContext = render_manager::WgpuFrameContext;

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
}

pub fn get_handler() -> impl WgpuGuiHandler {
    WebGuiHandler::Zero
}
