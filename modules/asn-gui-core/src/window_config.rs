#[derive(Debug, Clone)]
pub struct AsnGuiWindowConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

/// Validates that a value is greater than zero
fn validate_positive<T: PartialOrd + std::fmt::Display + From<u8>>(
    value: T,
    name: &str,
) -> Result<(), String> {
    if value <= T::from(0) {
        return Err(format!("{} ({}) must be positive", name, value));
    }
    Ok(())
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

/// Builder для создания конфигурации окна с гибкими настройками
pub struct WindowConfigBuilder {
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

impl WindowConfigBuilder {
    /// Создает новый builder с настройками по умолчанию
    pub fn new() -> Self {
        Self {
            title: None,
            width: None,
            height: None,
        }
    }

    /// Устанавливает заголовок окна
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Устанавливает ширину окна
    ///
    /// # Panics
    /// Паникует если ширина равна 0 или превышает 16384 пикселей
    pub fn width(mut self, width: u32) -> Self {
        validate_positive(width, "window width").expect("Invalid window width");
        if width > 16384 {
            panic!("Window width {} exceeds maximum allowed size 16384", width);
        }
        self.width = Some(width);
        self
    }

    /// Устанавливает высоту окна
    ///
    /// # Panics
    /// Паникует если высота равна 0 или превышает 16384 пикселей
    pub fn height(mut self, height: u32) -> Self {
        validate_positive(height, "window height").expect("Invalid window height");
        if height > 16384 {
            panic!(
                "Window height {} exceeds maximum allowed size 16384",
                height
            );
        }
        self.height = Some(height);
        self
    }

    /// Устанавливает размеры окна (ширина и высота)
    ///
    /// # Panics
    /// Паникует если размеры равны 0 или превышают 16384 пикселей
    pub fn size(self, width: u32, height: u32) -> Self {
        self.width(width).height(height)
    }

    /// Создает конфигурацию окна из настроек builder'а
    pub fn build(self) -> AsnGuiWindowConfig {
        AsnGuiWindowConfig {
            window_title: self
                .title
                .unwrap_or_else(|| "ASN WGPU Application".to_string()),
            window_width: self.width.unwrap_or(800),
            window_height: self.height.unwrap_or(600),
        }
    }
}

impl AsnGuiWindowConfig {
    /// Создает новый builder для пошаговой настройки
    pub fn builder() -> WindowConfigBuilder {
        WindowConfigBuilder::new()
    }

    /// Создает конфигурацию с базовыми параметрами (устаревший метод, используйте builder())
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

    #[test]
    fn test_builder_default() {
        let config = WindowConfigBuilder::new().build();
        assert_eq!(config.window_title, "ASN WGPU Application");
        assert_eq!(config.window_width, 800);
        assert_eq!(config.window_height, 600);
    }

    #[test]
    fn test_builder_custom_title() {
        let config = WindowConfigBuilder::new().title("My Custom App").build();
        assert_eq!(config.window_title, "My Custom App");
        assert_eq!(config.window_width, 800); // default
        assert_eq!(config.window_height, 600); // default
    }

    #[test]
    fn test_builder_custom_size() {
        let config = WindowConfigBuilder::new().size(1920, 1080).build();
        assert_eq!(config.window_title, "ASN WGPU Application"); // default
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
    }

    #[test]
    fn test_builder_full_custom() {
        let config = WindowConfigBuilder::new()
            .title("Gaming App")
            .width(2560)
            .height(1440)
            .build();
        assert_eq!(config.window_title, "Gaming App");
        assert_eq!(config.window_width, 2560);
        assert_eq!(config.window_height, 1440);
    }

    #[test]
    #[should_panic(expected = "window width")]
    fn test_builder_zero_width_panics() {
        WindowConfigBuilder::new().width(0);
    }

    #[test]
    #[should_panic(expected = "window height")]
    fn test_builder_zero_height_panics() {
        WindowConfigBuilder::new().height(0);
    }

    #[test]
    #[should_panic(expected = "exceeds maximum")]
    fn test_builder_width_too_large_panics() {
        WindowConfigBuilder::new().width(20000);
    }

    #[test]
    fn test_builder_chaining() {
        let config = AsnGuiWindowConfig::builder()
            .title("Chained Builder")
            .size(1280, 720)
            .build();
        assert_eq!(config.window_title, "Chained Builder");
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 720);
    }
}
