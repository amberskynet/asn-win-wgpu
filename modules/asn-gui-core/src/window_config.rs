#[derive(Debug, Clone)]
pub struct AsnGuiWindowConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for AsnGuiWindowConfig {
    fn default() -> Self {
        Self {
            window_title: "ASN WGPU Application".to_string(),
            window_width: 800,
            window_height: 600,
        }
    }
}

impl AsnGuiWindowConfig {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            window_title: title.into(),
            window_width: width,
            window_height: height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AsnGuiWindowConfig::default();
        assert_eq!(config.window_title, "ASN WGPU Application");
        assert_eq!(config.window_width, 800);
        assert_eq!(config.window_height, 600);
    }

    #[test]
    fn test_custom_config() {
        let config = AsnGuiWindowConfig {
            window_title: "Test App".to_string(),
            window_width: 1024,
            window_height: 768,
        };

        assert_eq!(config.window_title, "Test App");
        assert_eq!(config.window_width, 1024);
        assert_eq!(config.window_height, 768);
    }
}
