use crate::vertex::Vertex;

pub const LOG_MODULE_NAME: &str = "asn-wgpu";
/// Константы для настройки рендеринга
pub const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.1, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.2, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.3, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.4, 0.5],
    }, // E
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

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
