//! # ASN Win WGPU
//!
//! A modern window system implementation using Winit + WGPU for the Amberskynet project.
//!
//! ## Features
//!
//! - Cross-platform window management with Winit
//! - Hardware-accelerated rendering with WGPU
//! - Modular architecture with separate components
//! - WebAssembly support for web deployment
//! - Texture rendering with PNG/JPEG support
//! - Async/await support for modern Rust
//!
//! ## Quick Start
//!
//! ```rust
//! use asn_win_wgpu::run;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     run()
//! }
//! ```
//!
//! ## Custom Configuration
//!
//! ```rust
//! use asn_win_wgpu::{run_with_config, asn_win_config::AppConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AppConfig {
//!         window_title: "My Custom App".to_string(),
//!         window_width: 1024,
//!         window_height: 768,
//!     };
//!
//!     run_with_config(config)
//! }
//! ```

mod app;
mod data;

extern crate asn_logger;
extern crate asn_wgpu;
extern crate winit;

use app::App;
use asn_gui_core::AsnGuiWindowConfig;
use asn_logger::{error, info};
use data::LOG_MODULE_NAME;
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

/// Runs the application with a custom application instance
///
/// Allows you to use a custom application instance instead of the default one.
///
/// # Arguments
///
/// * `app` - Application instance to run
///
/// # Returns
///
/// Returns `Ok(())` on successful completion, or an error if the
/// application fails to start or encounters an unrecoverable error.
fn run_with_app(app: &mut impl ApplicationHandler) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().map_err(|e| format!("Failed to create event loop: {e}"))?;

    event_loop.set_control_flow(ControlFlow::Poll);
    let result = event_loop.run_app(app);

    match result {
        Ok(_) => {
            info(LOG_MODULE_NAME, "Application exited successfully");
            Ok(())
        }
        Err(e) => {
            error(LOG_MODULE_NAME, &e.to_string());
            Err(Box::new(std::io::Error::other(format!(
                "Application error: {e}"
            ))))
        }
    }
}

pub fn run(config: &AsnGuiWindowConfig) -> Result<(), Box<dyn std::error::Error>> {
    info(LOG_MODULE_NAME, "Starting ASN WGPU application");

    let mut app = App::new(config);

    run_with_app(&mut app)
}
