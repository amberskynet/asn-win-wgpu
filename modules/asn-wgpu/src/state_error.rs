use thiserror::Error;

/// Ошибки, которые могут возникнуть при работе с GPU
#[derive(Debug, Error)]
pub enum StateError {
    #[error("Failed to create surface: {0}")]
    SurfaceCreation(String),
    #[error("No suitable GPU adapter found")]
    NoAdapter,
    #[error("Failed to create device: {0}")]
    DeviceCreation(String),
    #[error("Failed to get current texture: {0}")]
    TextureError(String),
    #[error("Invalid window size: {width}x{height}")]
    InvalidWindowSize { width: u32, height: u32 },
} 