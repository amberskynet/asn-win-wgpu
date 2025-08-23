extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;
// extern crate wgpu_map;

mod log_utils;
use wgpu_map::{WgpuMap, get_map};

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_wgpu";

pub struct GuiList {
    m: WgpuMap,
}

impl GuiList {
    pub fn new(gcx: &render_manager::WgpuContext) -> Self {
        let map_tiles_bytes = include_bytes!("tiles.png");

        let m = get_map(gcx, map_tiles_bytes, 25, 25);
        GuiList { m }
    }
}

pub struct DummyGuiHandler {
    gui_list: Option<GuiList>,
}

use asn_gui_core::{TAsnGuiElement, TAsnGuiHandler};
use asn_wgpu::{WgpuGuiHandler, render_manager};

// как заполнять gui-компоненты до вызова init ?
// State -> Loaded/Unloaded
// Option -> Option<Element>
// FnOnce(GraphContext) -> new TAsnGuiHandler()
// Для примера сделаем решение с Option<Element>

impl TAsnGuiHandler for DummyGuiHandler {
    type GraphContext = render_manager::WgpuContext;
    type FrameContext = render_manager::WgpuFrameContext;

    fn init(&mut self, gcx: &Self::GraphContext) {
        let _ = gcx;
        let gui_list = GuiList::new(gcx);
        self.gui_list = Some(gui_list);
        m_info!("init");
    }

    fn update(&mut self) {
        m_info!("update");
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        self.gui_list.as_mut().unwrap().m.draw(fcx);
        m_info!("draw");
    }
}

pub fn get_handler() -> impl WgpuGuiHandler {
    DummyGuiHandler { gui_list: None }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let h = get_handler();

    let h_safe = Arc::new(Mutex::new(h));

    let r = asn_wgpu::get_manager(h_safe);

    asn_winit::run(r)?;

    for i in 0..2 {
        m_info!("update {i}");
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
