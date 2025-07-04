mod app;
mod data;

extern crate asn_logger;
extern crate asn_wgpu;
extern crate winit;

use app::{App, AppConfig};
use asn_logger::{error, info};
use data::LOG_MODULE_NAME;
use winit::event_loop::{ControlFlow, EventLoop};

/// Runs the application with default configuration
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    run_with_config(AppConfig::default())
}

/// Runs the application with custom configuration
pub fn run_with_config(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    info(LOG_MODULE_NAME, "Starting ASN WGPU application");

    let event_loop = EventLoop::new()
        .map_err(|e| format!("Failed to create event loop: {e}"))?;

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
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Application error: {e}"),
            )))
        }
    }
}

/// Creates a default application configuration
pub fn default_config() -> AppConfig {
    AppConfig::default()
}

/// Creates a custom application configuration
pub fn custom_config(
    title: impl Into<String>,
    width: u32,
    height: u32,
    vsync: bool,
) -> AppConfig {
    AppConfig {
        window_title: title.into(),
        window_width: width,
        window_height: height,
        vsync,
    }
}
