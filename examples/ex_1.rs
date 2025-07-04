extern crate asn_logger;
extern crate asn_node_quad;
extern crate asn_win_wgpu;

use asn_logger::{init_log, AsnLogConfig, AsnLogLevel};
use asn_node_quad::AsnNodeQuad;
use asn_win_wgpu::{run, run_with_config, custom_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Run with default configuration
    // run()?;

    // Example 2: Run with custom configuration
    let config = custom_config(
        "My Custom WGPU App",
        1024,
        768,
        true, // vsync enabled
    );
    
    run_with_config(config)
}
