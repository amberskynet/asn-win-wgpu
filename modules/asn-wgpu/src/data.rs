pub const LOG_MODULE_NAME: &str = "asn-wgpu";
/// Константы для настройки рендеринга
pub const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};

/// Минимальный размер окна
pub const MIN_WINDOW_SIZE: u32 = 1;

/// Максимальный размер окна (для валидации)
pub const MAX_WINDOW_SIZE: u32 = 16384;

/// Настройки для создания GPU устройства
pub const DEVICE_LABEL: &str = "ASN WGPU Device";
pub const SHADER_LABEL: &str = "ASN Shader";
pub const PIPELINE_LAYOUT_LABEL: &str = "ASN Render Pipeline Layout";
pub const PIPELINE_LABEL: &str = "ASN Render Pipeline";
pub const ENCODER_LABEL: &str = "ASN Render Encoder";
pub const RENDER_PASS_LABEL: &str = "ASN Render Pass"; 
