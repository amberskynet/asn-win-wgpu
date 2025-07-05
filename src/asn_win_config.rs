/// Configuration for the application
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub vsync: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "ASN WGPU Application".to_string(),
            window_width: 800,
            window_height: 600,
            vsync: true,
        }
    }
}

/// Creates a default application configuration
pub fn default_config() -> AppConfig {
    AppConfig::default()
}

/// Creates a custom application configuration
pub fn custom_config(title: impl Into<String>, width: u32, height: u32, vsync: bool) -> AppConfig {
    AppConfig {
        window_title: title.into(),
        window_width: width,
        window_height: height,
        vsync,
    }
}
