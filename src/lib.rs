//! ASN Win WGPU - Modern window system implementation
//!
//! This crate provides a high-level API for creating GPU-accelerated applications
//! with WGPU and Winit, featuring builder patterns for easy configuration.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use asn_win_wgpu::{run, AppConfig};
//! use my_handler::MyGuiHandler;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Simple run with defaults
//!     let handler = MyGuiHandler::new();
//!     run(handler)?;
//!
//!     // Or with custom config
//!     let config = AppConfig::builder()
//!         .window(|w| w.title("My App").size(1920, 1080))
//!         .vsync(false)
//!         .build();
//!
//!     run_with_config(handler, config)
//! }
//! ```

pub mod app;
pub mod config;
pub mod utils;

pub use app::*;
pub use config::*;
pub use utils::*;

// Re-exports for convenience
pub use asn_gui_core::AsnGuiWindowConfig;
pub use asn_wgpu;
pub use asn_winit;
