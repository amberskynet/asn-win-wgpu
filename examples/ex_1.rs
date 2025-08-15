extern crate asn_logger;
extern crate asn_win_wgpu;

mod log_utils;

use std::sync::{Arc, Mutex};

use log_utils::setup_log;

use asn_gui_core::{AsnGuiWindowConfig, TAsnGuiHandler};
use asn_win_wgpu::run;

struct MyAsnGuiHandler {}

impl TAsnGuiHandler for MyAsnGuiHandler {
    type FrameContext = ();

    fn update(&mut self) {
        println!("Update");
    }

    fn draw(&mut self, fcx: Self::FrameContext) {
        let _ = fcx;
        println!("Draw");
    }

    fn init(&mut self) {
        todo!()
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
