extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;

mod log_utils;

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_wgpu";

pub struct DummyGuiHandler {}

use asn_gui_core::TAsnGuiHandler;
use asn_wgpu::{WgpuGuiHandler, render_manager};

// как заполнять gui-компоненты до вызова init ?
// State -> Loaded/Unloaded
// Option -> Option<Element>
// FnOnce() -> TAsnGuiHandler

impl TAsnGuiHandler for DummyGuiHandler {
    type GraphContext = render_manager::WgpuContext;
    type FrameContext = render_manager::WgpuFrameContext;

    fn init(&mut self, gcx: &Self::GraphContext) {
        let _ = gcx;
        m_info!("init");
    }

    fn update(&mut self) {
        m_info!("update");
    }

    fn draw(&mut self, fcx: &Self::FrameContext) {
        let _ = fcx;
        m_info!("draw");
    }
}

pub fn get_handler() -> impl WgpuGuiHandler {
    DummyGuiHandler {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let h = DummyGuiHandler {};

    let h_safe = Arc::new(Mutex::new(h));

    let r = asn_wgpu::get_manager(h_safe);

    asn_winit::run(r)?;

    for i in 0..2 {
        m_info!("update {i}");
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
