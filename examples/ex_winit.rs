extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_winit;

mod log_utils;
use asn_gui_core::*;
use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_winit";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    info(LOG_MODULE_NAME, "hello from main()");

    let config = AsnGuiWindowConfig::new("My Custom WGPU App Ex Winit", 1024, 768);

    asn_winit::run(&config)
}
