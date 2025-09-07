#[allow(unused_imports)]
use js_sys::Date;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Платформо-независимая обертка для времени
#[derive(Debug, Clone, Copy)]
pub struct Instant {
    timestamp: f64,
}

impl Instant {
    /// Создает новый экземпляр времени
    pub fn now() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            // Для WebAssembly используем Date.now()
            Instant {
                timestamp: Date::now(),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Для нативной версии используем std::time::Instant
            use std::time::SystemTime;
            Instant {
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis() as f64,
            }
        }
    }

    /// Возвращает разницу во времени в миллисекундах
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let ms = self.timestamp - earlier.timestamp;
        Duration { ms }
    }
}

/// Платформо-независимая обертка для продолжительности
#[derive(Debug, Clone, Copy)]
pub struct Duration {
    ms: f64,
}

#[allow(dead_code)]
impl Duration {
    /// Возвращает продолжительность в миллисекундах
    pub fn as_millis(&self) -> u128 {
        self.ms as u128
    }

    /// Возвращает продолжительность в секундах
    pub fn as_secs_f64(&self) -> f64 {
        self.ms / 1000.0
    }
}
