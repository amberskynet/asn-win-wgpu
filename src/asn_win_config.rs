/// Configuration for the application
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "ASN WGPU Application".to_string(),
            window_width: 800,
            window_height: 600,
        }
    }
}

/// Creates a default application configuration
pub fn default_config() -> AppConfig {
    AppConfig::default()
}

/// Creates a custom application configuration
pub fn custom_config(title: impl Into<String>, width: u32, height: u32) -> AppConfig {
    AppConfig {
        window_title: title.into(),
        window_width: width,
        window_height: height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.window_title, "ASN WGPU Application");
        assert_eq!(config.window_width, 800);
        assert_eq!(config.window_height, 600);
    }

    #[test]
    fn test_custom_config() {
        let config = AppConfig {
            window_title: "Test App".to_string(),
            window_width: 1024,
            window_height: 768,
        };

        assert_eq!(config.window_title, "Test App");
        assert_eq!(config.window_width, 1024);
        assert_eq!(config.window_height, 768);
    }

    #[test]
    fn test_custom_config_builder() {
        let config = custom_config("Builder Test", 1920, 1080);

        assert_eq!(config.window_title, "Builder Test");
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
    }
}
