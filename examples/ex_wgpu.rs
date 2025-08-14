extern crate asn_gui_core;
extern crate asn_logger;
extern crate asn_wgpu;
extern crate asn_winit;

mod log_utils;

use asn_logger::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_wgpu";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    m_info!("hello from main()");

    let r = asn_wgpu::get_manager();

    asn_winit::run(r)
}
