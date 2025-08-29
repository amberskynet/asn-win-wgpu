//! Модуль для обработки клавиатурных событий в приложении
//!
//! Этот модуль содержит функции для обработки различных клавиш клавиатуры,
//! таких как Escape, F11 (переключение полноэкранного режима), F1 (помощь) и другие.

use asn_logger::log::*;
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};

/// Обрабатывает событие нажатия клавиши
///
/// # Аргументы
/// * `event_loop` - активный цикл событий приложения
/// * `event` - событие клавиатуры
///
/// # Поддерживаемые клавиши
/// * Escape - закрытие приложения
/// * F11 - переключение полноэкранного режима
/// * F1 - показ справки
/// * R/r - перезагрузка (пока без реализации)
pub fn handle_key_event(event_loop: &ActiveEventLoop, event: KeyEvent) {
    if event.state == ElementState::Pressed {
        match event.logical_key.as_ref() {
            Key::Character("Escape") => {
                handle_escape_key(event_loop);
            }
            Key::Named(NamedKey::F11) => {
                handle_f11_key();
            }
            Key::Named(NamedKey::F1) => {
                handle_f1_key();
            }
            Key::Character("r") | Key::Character("R") => {
                handle_r_key();
            }
            _ => {
                // Логируем нажатие других клавиш для отладки
                #[cfg(debug_assertions)]
                trace!("Key pressed: {:?}", event.logical_key);
            }
        }
    }
}

/// Обрабатывает нажатие клавиши Escape (закрытие приложения)
fn handle_escape_key(event_loop: &ActiveEventLoop) {
    info!("Escape key pressed - closing application");
    event_loop.exit();
}

/// Обрабатывает нажатие клавиши F11 (переключение полноэкранного режима)
fn handle_f11_key() {
    info!("F11 key pressed - toggling fullscreen");
    // TODO: Реализовать переключение полноэкранного режима
    info!("Fullscreen toggle not yet implemented");
}

/// Обрабатывает нажатие клавиши F1 (показ справки)
fn handle_f1_key() {
    info!("F1 key pressed - showing help");
    // TODO: Реализовать систему помощи
}

/// Обрабатывает нажатие клавиши R (перезагрузка)
fn handle_r_key() {
    info!("R key pressed...");
    // TODO: Реализовать функциональность перезагрузки
}
