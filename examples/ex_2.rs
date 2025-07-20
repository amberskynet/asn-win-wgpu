extern crate asn_logger;
extern crate asn_win_wgpu;

mod log_utils;
use log_utils::setup_log;

use asn_win_wgpu::{asn_win_config::AppConfig, run_with_config};


/// Advanced example demonstrating custom configuration and error handling
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    setup_log();
    

    // Create a custom configuration for a high-resolution display
    let config = AppConfig {
        window_title: "ASN WGPU - Advanced Example".to_string(),
        window_width: 1920,
        window_height: 1080,
        vsync: true, // Enable VSync for smooth rendering
    };

    // Run the application with custom configuration
    match run_with_config(config) {
        Ok(_) => {
            println!("Application completed successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Application failed: {}", e);
            Err(e)
        }
    }
}
