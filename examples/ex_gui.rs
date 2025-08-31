extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;

mod log_utils;
mod map_utils;

use asn_core::{cgmath::Vector3, transform_set::TransformSet};
use map_utils::generate_random_map;
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

const LOG_MODULE_NAME: &str = "ex_gui";

const LOOP_MILLIS: u64 = 500;

pub struct GuiList {
    m: WgpuMap,
    map_width: u32,
    map_height: u32,
    tiles_width: u32,
    tiles_height: u32,
}

impl GuiList {
    pub fn new(gcx: &render_manager::WgpuGraphContext) -> Self {
        // let map_tiles_bytes = include_bytes!("tiles_64_95.png");
        // let tiles_width = 64;
        // let tiles_height = 95;

        let map_tiles_bytes = include_bytes!("tiles_16_12.png");
        let tiles_width = 16;
        let tiles_height = 12;

        let map_width = 2;
        let map_height = 2;

        // Генерируем случайные значения для карты
        let mut map = generate_random_map(map_width, map_height, map_width * map_height - 1);

        map[0] = 1;
        map[1] = 2;
        map[2] = 3;
        map[3] = 4;

        println!("map: {:?}", map);

        let tiles_params = wgpu_map::MapTilesParams {
            map_tiles_bytes,
            tiles_width,
            tiles_height,
        };
        let map_params = wgpu_map::MapParams {
            map_width,
            map_height,
            tile_indices: map.as_slice(),
        };
        let m = get_map(gcx, &tiles_params, &map_params);

        let s = TransformSet {
            pos: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rot: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vector3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
        };

        // // Создаем и устанавливаем MVP-матрицу
        let mvp_matrix = s.matrix_calculated();
        m.update_mvp_matrix(gcx, mvp_matrix.into());

        GuiList {
            m,
            map_width,
            map_height,
            tiles_width,
            tiles_height,
        }
    }

    /// Обновляет карту случайными значениями
    pub fn update_map(&mut self) {
        let map = generate_random_map(
            self.map_width,
            self.map_height,
            self.tiles_width * self.tiles_height - 1,
        );
        self.m.update_map(map.as_slice());
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

        g.update_map();

        m_info!("update")
    }
}

use asn_gui_core::{TAsnGuiElement, TAsnGuiHandler};
use asn_wgpu::render_manager;

impl TAsnGuiHandler for MyGuiHandler {
    type GraphContext = render_manager::WgpuGraphContext;
    type FrameContext = render_manager::WgpuFrameContext;

    fn init(&mut self, gcx: &Self::GraphContext) {
        let gui_list = GuiList::new(gcx);
        self.gui_list = Some(gui_list);
        m_info!("init");
    }

    fn update(&mut self, gcx: &Self::GraphContext) {
        self.gui_list.as_mut().unwrap().m.update(gcx);
    }

    // fn handle_keyboard_input(
    //     &mut self,
    //     gcx: &Self::GraphContext,
    //     input: &winit::event::KeyboardInput,
    // ) {
    //     if let Some(ref gui_list) = self.gui_list {
    //         // Обрабатываем нажатия клавиш для изменения масштаба
    //         if let winit::event::ElementState::Pressed = input.state {
    //             match input.virtual_keycode {
    //                 Some(winit::event::VirtualKeyCode::Plus)
    //                 | Some(winit::event::VirtualKeyCode::Equals) => {
    //                     // Увеличиваем масштаб
    //                     self.scale_factor *= 1.1;
    //                     gui_list.m.apply_uniform_scaling(gcx, self.scale_factor);
    //                     m_info!("Увеличен масштаб до: {}", self.scale_factor);
    //                 }
    //                 Some(winit::event::VirtualKeyCode::Minus)
    //                 | Some(winit::event::VirtualKeyCode::Underline) => {
    //                     // Уменьшаем масштаб
    //                     self.scale_factor *= 0.9;
    //                     gui_list.m.apply_uniform_scaling(gcx, self.scale_factor);
    //                     m_info!("Уменьшен масштаб до: {}", self.scale_factor);
    //                 }
    //                 Some(winit::event::VirtualKeyCode::Key0) => {
    //                     // Сброс масштаба
    //                     self.scale_factor = 1.0;
    //                     gui_list.m.apply_uniform_scaling(gcx, self.scale_factor);
    //                     m_info!("Масштаб сброшен до: {}", self.scale_factor);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }

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
                // h.update_map();
            }
            thread::sleep(Duration::from_millis(LOOP_MILLIS));
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
