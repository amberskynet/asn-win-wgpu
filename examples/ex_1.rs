extern crate asn_logger;
extern crate asn_win_wgpu;

mod log_utils;

use std::sync::{Arc, Mutex};

use log_utils::setup_log;

use asn_gui_core::{AsnGuiElement, AsnGuiHandler, AsnGuiMap, AsnGuiWindowConfig};
use asn_win_wgpu::run;

struct MyAsnGuiHandler {
    m: Box<dyn AsnGuiMap>,
}

impl AsnGuiHandler for MyAsnGuiHandler {
    fn update(&mut self) {
        println!("Update");
    }

    fn draw(&mut self) {
        println!("Draw");
    }

    fn init(&mut self, f: &impl asn_gui_core::AsnGuiFabrica) {
        let map_width = 256;
        let map_height = 256;
        let map_tiles_bytes = include_bytes!("tiles.png");
        let m = f.get_map(map_tiles_bytes, map_width, map_height);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    let handler = MyAsnGuiHandler {};

    let h = Arc::new(Mutex::new(handler));

    // Example 2: Run with custom configuration
    let config = AsnGuiWindowConfig::new("My Custom WGPU App", 1024, 768);

    run(&config, h)
}
