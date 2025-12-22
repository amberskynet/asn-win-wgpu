//! High-level application API for ASN
//!
//! This module provides ergonomic functions for running GPU-accelerated applications
//! with minimal boilerplate code. It supports both simple and advanced configuration scenarios.
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use asn_win_wgpu::run;
//! use my_gui_handler::MyHandler;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let handler = MyHandler::new();
//!     run(handler)
//! }
//! ```
//!
//! ## Advanced Configuration
//!
//! ```rust,no_run
//! use asn_win_wgpu::{run_with_config, AppConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let handler = my_handler::create_handler();
//!     let config = AppConfig::builder()
//!         .window(|w| w
//!             .title("Advanced GPU App")
//!             .size(1920, 1080)
//!         )
//!         .vsync(false)
//!         .power_preference(asn_wgpu::wgpu::PowerPreference::HighPerformance)
//!         .build();
//!
//!     run_with_config(handler, config)
//! }
//! ```
//!
//! ## Builder Pattern
//!
//! ```rust,no_run
//! use asn_win_wgpu::App;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     App::new(my_handler::MyHandler::new())
//!         .with_config(|c| c
//!             .window(|w| w.title("Builder Pattern App"))
//!             .vsync(true)
//!         )
//!         .run()
//! }
//! ```

use asn_wgpu::WgpuGuiHandler;
use std::sync::{Arc, Mutex};

use crate::config::AppConfig;

/// Runs an application with default configuration
///
/// # Example
/// ```rust,no_run
/// use asn_win_wgpu::run;
/// use my_gui_handler::MyHandler;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let handler = MyHandler::new();
///     run(handler)
/// }
/// ```
pub fn run<H>(handler: H) -> Result<(), Box<dyn std::error::Error>>
where
    H: WgpuGuiHandler + 'static,
{
    run_with_config(handler, AppConfig::default())
}

/// Runs an application with custom configuration
///
/// # Example
/// ```rust,no_run
/// use asn_win_wgpu::{run_with_config, AppConfig};
/// use my_gui_handler::MyHandler;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let handler = MyHandler::new();
///     let config = AppConfig::builder()
///         .window(|w| w.title("My App").size(1920, 1080))
///         .vsync(false)
///         .build();
///
///     run_with_config(handler, config)
/// }
/// ```
pub fn run_with_config<H>(handler: H, _config: AppConfig) -> Result<(), Box<dyn std::error::Error>>
where
    H: WgpuGuiHandler + 'static,
{
    // For now, use default window config
    // TODO: Use the config parameter when window config is integrated

    let handler = Arc::new(Mutex::new(handler));
    let manager = asn_wgpu::get_manager(handler);

    asn_winit::run(manager).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

/// Creates an application builder for advanced configuration
///
/// # Example
/// ```rust,no_run
/// use asn_win_wgpu::App;
/// use my_gui_handler::MyHandler;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     App::new(MyHandler::new())
///         .with_config(|c| c
///             .window(|w| w.title("Advanced App").size(2560, 1440))
///             .vsync(true)
///         )
///         .run()
/// }
/// ```
pub struct App<H> {
    handler: H,
    config_builder: crate::config::AppConfigBuilder,
}

impl<H> App<H>
where
    H: WgpuGuiHandler + 'static,
{
    /// Creates a new application with the given handler
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            config_builder: crate::config::AppConfigBuilder::new(),
        }
    }

    /// Configures the application
    pub fn with_config<F>(mut self, f: F) -> Self
    where
        F: FnOnce(crate::config::AppConfigBuilder) -> crate::config::AppConfigBuilder,
    {
        self.config_builder = f(self.config_builder);
        self
    }

    /// Runs the application
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config_builder.build();
        run_with_config(self.handler, config)
    }
}

/// Utility function to create an application with a closure
///
/// # Example
/// ```rust,no_run
/// use asn_win_wgpu::run_app;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     run_app(|| {
///         // Create and return your handler here
///         my_gui_handler::MyHandler::new()
///     })
/// }
/// ```
pub fn run_app<F, H>(handler_fn: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce() -> H,
    H: WgpuGuiHandler + 'static,
{
    let handler = handler_fn();
    run(handler)
}
