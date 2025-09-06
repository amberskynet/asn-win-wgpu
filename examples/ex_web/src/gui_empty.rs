use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use asn_winit::WinitWindow;

#[allow(dead_code)]
const LOG_MODULE_NAME: &str = "gui_empty";

struct MyRenderManager {}

impl TAsnRenderManager for MyRenderManager {
    type Window = WinitWindow;

    fn init(&mut self, w: std::sync::Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let _ = w;
        m_info!("init");
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let _ = height;
        let _ = width;
        m_info!("resize");
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("draw");
        Ok(())
    }
}

pub fn run_gui() {
    let r = MyRenderManager {};

    match asn_winit::run(r) {
        Ok(_) => {
            m_info!("Application finished successfully");
        }
        Err(e) => {
            m_error!("Application failed with error: {}", e);
        }
    }
}
