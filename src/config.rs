//! Configuration module for ASN applications
//!
//! This module provides flexible configuration options for ASN applications
//! using the builder pattern. You can configure window settings, rendering options,
//! and other application parameters.
//!
//! ## Basic Configuration
//!
//! ```rust,no_run
//! use asn_win_wgpu::AppConfig;
//!
//! let config = AppConfig::default();
//! // Uses default window title "ASN WGPU Application" and size 800x600
//! ```
//!
//! ## Custom Configuration
//!
//! ```rust,no_run
//! use asn_win_wgpu::AppConfig;
//!
//! let config = AppConfig::builder()
//!     .window(|w| w
//!         .title("My Custom App")
//!         .size(1920, 1080)
//!     )
//!     .vsync(false)
//!     .power_preference(asn_wgpu::wgpu::PowerPreference::HighPerformance)
//!     .build();
//! ```
//!
//! ## Window Configuration
//!
//! ```rust,no_run
//! use asn_gui_core::WindowConfigBuilder;
//!
//! let window_config = WindowConfigBuilder::new()
//!     .title("Gaming App")
//!     .width(2560)
//!     .height(1440)
//!     .build();
//!
//! // Or using size() method
//! let window_config2 = WindowConfigBuilder::new()
//!     .title("Productivity App")
//!     .size(1440, 900)
//!     .build();
//! ```

use asn_gui_core::AsnGuiWindowConfig;

/// Main application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Window configuration
    pub window: AsnGuiWindowConfig,
    /// Enable vsync
    pub vsync: bool,
    /// Power preference for GPU
    pub power_preference: asn_wgpu::wgpu::PowerPreference,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: AsnGuiWindowConfig::default(),
            vsync: true,
            power_preference: asn_wgpu::wgpu::PowerPreference::default(),
        }
    }
}

/// Builder for application configuration
pub struct AppConfigBuilder {
    window_builder: asn_gui_core::WindowConfigBuilder,
    vsync: Option<bool>,
    power_preference: Option<asn_wgpu::wgpu::PowerPreference>,
}

impl AppConfigBuilder {
    /// Creates a new configuration builder with default settings
    ///
    /// # Example
    /// ```rust,no_run
    /// use asn_win_wgpu::AppConfig;
    ///
    /// let builder = AppConfig::builder();
    /// ```
    pub fn new() -> Self {
        Self {
            window_builder: asn_gui_core::WindowConfigBuilder::new(),
            vsync: None,
            power_preference: None,
        }
    }

    /// Configures the window settings
    pub fn window<F>(mut self, f: F) -> Self
    where
        F: FnOnce(asn_gui_core::WindowConfigBuilder) -> asn_gui_core::WindowConfigBuilder,
    {
        self.window_builder = f(self.window_builder);
        self
    }

    /// Sets vsync on/off
    pub fn vsync(mut self, enabled: bool) -> Self {
        self.vsync = Some(enabled);
        self
    }

    /// Sets GPU power preference
    pub fn power_preference(mut self, preference: asn_wgpu::wgpu::PowerPreference) -> Self {
        self.power_preference = Some(preference);
        self
    }

    /// Builds the final configuration
    pub fn build(self) -> AppConfig {
        AppConfig {
            window: self.window_builder.build(),
            vsync: self.vsync.unwrap_or(true),
            power_preference: self
                .power_preference
                .unwrap_or(asn_wgpu::wgpu::PowerPreference::default()),
        }
    }
}

impl AppConfig {
    /// Creates a new configuration builder
    pub fn builder() -> AppConfigBuilder {
        AppConfigBuilder::new()
    }
}
