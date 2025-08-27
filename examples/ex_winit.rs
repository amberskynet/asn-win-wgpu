extern crate asn_logger;

mod log_utils;
use std::{sync::Arc, thread::sleep, time::Duration};

use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use asn_winit::WinitWindow;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_winit";

pub struct DummyRenderManager {
    w: Option<Arc<WinitWindow>>,
}

impl TAsnRenderManager for DummyRenderManager {
    type Window = WinitWindow;

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("init");
        self.w = Some(w);
        Ok(())
    }

    fn resize(&mut self, width: u32, _height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let _ = width;
        m_info!("resize");
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("draw");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let r = DummyRenderManager { w: None };

    asn_winit::run(r)?;

    for i in 0..2 {
        m_info!("update {i}");
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
