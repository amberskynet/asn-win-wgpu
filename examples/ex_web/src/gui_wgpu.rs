use asn_logger::*;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
const LOG_MODULE_NAME: &str = "gui_wgpu";

mod dummy_gui {
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
            m_info!("update");
        }

        fn draw(&mut self, fcx: &mut Self::FrameContext) {
            let _ = fcx;
            m_info!("draw");
        }

        type GraphContext = render_manager::WgpuGraphContext;
        type FrameContext = render_manager::WgpuFrameContext;
    }

    pub fn get_handler() -> impl WgpuGuiHandler {
        DummyGuiHandler {}
    }
}

#[allow(dead_code)]
pub fn run_gui() {
    let h = dummy_gui::get_handler();

    let h_safe = Arc::new(Mutex::new(h));

    let r = asn_wgpu::get_manager(h_safe);

    match asn_winit::run(r) {
        Ok(_) => {
            m_info!("Application finished successfully");
        }
        Err(e) => {
            m_error!("Application failed with error: {}", e);
        }
    }
}
