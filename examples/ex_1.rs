extern crate asn_logger;
extern crate asn_win_wgpu;

mod log_utils;

use std::sync::Arc;

use log_utils::setup_log;

use asn_gui_core::{AsnGuiHandler, AsnGuiWindowConfig};
use asn_win_wgpu::run;

struct MyAsnGuiHandler {}

impl AsnGuiHandler for MyAsnGuiHandler {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    let h = Arc::new(MyAsnGuiHandler {});

    // Example 2: Run with custom configuration
    let config = AsnGuiWindowConfig::new("My Custom WGPU App", 1024, 768);

    run(&config, h)
}
