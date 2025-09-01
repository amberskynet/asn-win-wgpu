//! Модуль для обработки ошибок в asn-winit
//!
//! Этот модуль определяет пользовательские типы ошибок, которые могут возникнуть
//! при работе с модулем asn-winit.

use std::fmt;

/// Перечисление возможных ошибок в модуле asn-winit
#[derive(Debug)]
pub enum AsnWinitError {
    /// Ошибка создания цикла событий
    EventLoopCreationError(String),

    /// Ошибка создания окна
    WindowCreationError(String),

    /// Ошибка инициализации рендерера
    RendererInitializationError(String),

    /// Ошибка изменения размера окна
    WindowResizeError(String),

    /// Ошибка отрисовки
    RenderError(String),
}

impl fmt::Display for AsnWinitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsnWinitError::EventLoopCreationError(msg) => {
                write!(f, "Ошибка создания цикла событий: {}", msg)
            }
            AsnWinitError::WindowCreationError(msg) => {
                write!(f, "Ошибка создания окна: {}", msg)
            }
            AsnWinitError::RendererInitializationError(msg) => {
                write!(f, "Ошибка инициализации рендерера: {}", msg)
            }
            AsnWinitError::WindowResizeError(msg) => {
                write!(f, "Ошибка изменения размера окна: {}", msg)
            }
            AsnWinitError::RenderError(msg) => {
                write!(f, "Ошибка отрисовки: {}", msg)
            }
        }
    }
}

impl std::error::Error for AsnWinitError {}

#[allow(dead_code)]
/// Создает ошибку создания цикла событий
pub fn event_loop_creation_error(msg: impl Into<String>) -> AsnWinitError {
    AsnWinitError::EventLoopCreationError(msg.into())
}

#[allow(dead_code)]
/// Создает ошибку создания окна
pub fn window_creation_error(msg: impl Into<String>) -> AsnWinitError {
    AsnWinitError::WindowCreationError(msg.into())
}

#[allow(dead_code)]
/// Создает ошибку инициализации рендерера
pub fn renderer_initialization_error(msg: impl Into<String>) -> AsnWinitError {
    AsnWinitError::RendererInitializationError(msg.into())
}

#[allow(dead_code)]
/// Создает ошибку изменения размера окна
pub fn window_resize_error(msg: impl Into<String>) -> AsnWinitError {
    AsnWinitError::WindowResizeError(msg.into())
}

#[allow(dead_code)]
/// Создает ошибку отрисовки
pub fn render_error(msg: impl Into<String>) -> AsnWinitError {
    AsnWinitError::RenderError(msg.into())
}
