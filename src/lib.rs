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
//!         vsync: true,
//!     };
//!     
//!     run_with_config(config)
//! }
//! ```

mod app;
pub mod asn_win_config;
mod data;

extern crate asn_logger;
extern crate asn_wgpu;
extern crate winit;

use app::App;
use asn_logger::{error, info};
use asn_win_config::AppConfig;
use data::LOG_MODULE_NAME;
use winit::event_loop::{ControlFlow, EventLoop};

/// Runs the application with default configuration
///
/// This is the simplest way to start the application. It uses default
/// window settings and configuration.
///
/// # Returns
///
/// Returns `Ok(())` on successful completion, or an error if the
/// application fails to start or encounters an unrecoverable error.
///
/// # Example
///
/// ```rust
/// use asn_win_wgpu::run;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     run()
/// }
/// ```
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    run_with_config(AppConfig::default())
}

/// Runs the application with custom configuration
///
/// Allows you to customize window properties, rendering settings,
/// and other application parameters.
///
/// # Arguments
///
/// * `config` - Application configuration containing window settings
///
/// # Returns
///
/// Returns `Ok(())` on successful completion, or an error if the
/// application fails to start or encounters an unrecoverable error.
///
/// # Example
///
/// ```rust
/// use asn_win_wgpu::{run_with_config, asn_win_config::AppConfig};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = AppConfig {
///         window_title: "My App".to_string(),
///         window_width: 1024,
///         window_height: 768,
///         vsync: true,
///     };
///     
///     run_with_config(config)
/// }
/// ```
pub fn run_with_config(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    info(LOG_MODULE_NAME, "Starting ASN WGPU application");

    let event_loop = EventLoop::new().map_err(|e| format!("Failed to create event loop: {e}"))?;

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::with_config(config);
    let result = event_loop.run_app(&mut app);

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
