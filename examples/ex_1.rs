extern crate asn_logger;
extern crate asn_win_wgpu;

mod log_utils;
use log_utils::setup_log;

use asn_gui_core::AsnGuiWindowConfig;
use asn_win_wgpu::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    // Example 2: Run with custom configuration
    let config = AsnGuiWindowConfig::new("My Custom WGPU App", 1024, 768);

    run(&config)
}
