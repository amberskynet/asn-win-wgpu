mod log_utils;

use std::sync::Arc;

use asn_logger::{m_error, m_info};
use asn_winit::{WinitWindow, run};
use log_utils::setup_log;

use asn_gui_core::TAsnRenderManager;

pub const LOG_MODULE_NAME: &str = file!();

struct MyRenderManager {}

impl TAsnRenderManager for MyRenderManager {
    type Window = WinitWindow;

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let _ = w;
        m_info!("MyRenderManager::init()");
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let _ = height;
        let _ = width;
        m_info!("MyRenderManager::resize({}, {})", width, height);
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("MyRenderManager::draw()");
        Ok(())
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("MyRenderManager::update()");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    let handler = MyRenderManager {};

    if let Err(e) = run(handler) {
        m_error!("Error running application: {}", e);
    }

    Ok(())
}
