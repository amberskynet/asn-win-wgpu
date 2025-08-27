extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;

mod log_utils;
use wgpu_map::{WgpuMap, get_map};

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, sleep},
    time::Duration,
};

use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_gui";

pub struct GuiList {
    m: WgpuMap,
}

impl GuiList {
    pub fn new(gcx: &render_manager::WgpuGraphContext) -> Self {
        let map_tiles_bytes = include_bytes!("tiles.png");

        let m = get_map(gcx, map_tiles_bytes, 25, 25);
        GuiList { m }
    }
}

pub struct MyGuiHandler {
    gui_list: Option<GuiList>,
}

impl MyGuiHandler {
    fn update_map(&mut self) {
        let g = match self.gui_list.as_mut() {
            Some(g) => g,
            None => {
                return;
            }
        };

        g.m.fill_random();

        m_info!("update")
    }
}

use asn_gui_core::{TAsnGuiElement, TAsnGuiHandler};
use asn_wgpu::render_manager;

impl TAsnGuiHandler for MyGuiHandler {
    type GraphContext = render_manager::WgpuGraphContext;
    type FrameContext = render_manager::WgpuFrameContext;

    fn init(&mut self, gcx: &Self::GraphContext) {
        let _ = gcx;
        let gui_list = GuiList::new(gcx);
        self.gui_list = Some(gui_list);
        m_info!("init");
    }

    fn update(&mut self, gcx: &Self::GraphContext) {
        self.gui_list.as_mut().unwrap().m.update(gcx);
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        self.gui_list.as_mut().unwrap().m.draw(fcx);
        // m_info!("draw");
    }
}

pub fn get_handler() -> MyGuiHandler {
    MyGuiHandler { gui_list: None }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let is_running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&is_running);

    let h = get_handler();

    let h_safe = Arc::new(Mutex::new(h));

    let h_thread = h_safe.clone();

    let r = asn_wgpu::get_manager(h_safe);

    let handle = thread::spawn(move || {
        // Цикл обработки с отправкой результатов
        while running_clone.load(Ordering::Relaxed) {
            {
                let mut h = match h_thread.lock() {
                    Ok(h) => h,
                    Err(e) => {
                        m_error!("Error: {e}");
                        return;
                    }
                };
                h.update_map();
            }
            thread::sleep(Duration::from_millis(5));
        }
        m_info!("Exit from loop");
    });

    asn_winit::run(r)?;

    is_running.store(false, Ordering::Relaxed);
    handle.join().unwrap();

    for i in 0..2 {
        m_info!("wait {i}");
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
