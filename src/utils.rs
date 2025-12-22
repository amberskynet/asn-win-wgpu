//! Common utilities and helper functions
//!
//! This module contains shared utilities used across the ASN codebase
//! to reduce code duplication and improve maintainability.

use std::sync::{Arc, Mutex};

/// Safely locks a Mutex and returns the guard, converting lock errors to application errors
///
/// # Type Parameters
/// * `T` - The type stored in the Mutex
/// * `E` - The error type to convert lock errors to
///
/// # Arguments
/// * `mutex` - The mutex to lock
/// * `error_fn` - Function to create an error from the lock error string
///
/// # Returns
/// * `Result<MutexGuard<T>, E>` - The lock guard or an error
pub fn lock_mutex<T, E, F>(
    mutex: &Arc<Mutex<T>>,
    error_fn: F,
) -> Result<std::sync::MutexGuard<T>, E>
where
    F: FnOnce(String) -> E,
{
    mutex.lock().map_err(|e| error_fn(e.to_string()))
}

/// Safely locks a Mutex with a default error message
///
/// # Type Parameters
/// * `T` - The type stored in the Mutex
///
/// # Arguments
/// * `mutex` - The mutex to lock
///
/// # Returns
/// * `Result<MutexGuard<T>, String>` - The lock guard or an error message
pub fn lock_mutex_simple<T>(mutex: &Arc<Mutex<T>>) -> Result<std::sync::MutexGuard<T>, String> {
    lock_mutex(mutex, |msg| format!("Mutex lock failed: {}", msg))
}

/// Validates that a value is within a specified range
///
/// # Arguments
/// * `value` - The value to validate
/// * `min` - Minimum allowed value (inclusive)
/// * `max` - Maximum allowed value (inclusive)
/// * `name` - Name of the value for error messages
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, error message if invalid
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    name: &str,
) -> Result<(), String> {
    if value < min {
        return Err(format!("{} ({}) is below minimum ({})", name, value, min));
    }
    if value > max {
        return Err(format!("{} ({}) is above maximum ({})", name, value, max));
    }
    Ok(())
}

/// Validates that a value is greater than zero
///
/// # Arguments
/// * `value` - The value to validate
/// * `name` - Name of the value for error messages
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, error message if invalid
pub fn validate_positive<T: PartialOrd + std::fmt::Display + From<u8>>(
    value: T,
    name: &str,
) -> Result<(), String> {
    if value <= T::from(0) {
        return Err(format!("{} ({}) must be positive", name, value));
    }
    Ok(())
}

/// Safely converts an Option to a Result with a custom error message
///
/// # Arguments
/// * `option` - The Option to convert
/// * `error_msg` - Error message if None
///
/// # Returns
/// * `Result<T, String>` - The value or error message
pub fn option_to_result<T>(option: Option<T>, error_msg: &str) -> Result<T, String> {
    option.ok_or_else(|| error_msg.to_string())
}

/// Clamps a value to be within a specified range
///
/// # Arguments
/// * `value` - The value to clamp
/// * `min` - Minimum value
/// * `max` - Maximum value
///
/// # Returns
/// * The clamped value
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Performance timer for measuring execution time
pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    /// Creates a new timer and starts measuring
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    /// Gets the elapsed time since timer creation
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    /// Gets the elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }

    /// Resets the timer
    pub fn reset(&mut self) {
        self.start = std::time::Instant::now();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple RAII wrapper for cleanup operations
pub struct ScopeGuard<F: FnOnce()> {
    cleanup: Option<F>,
}

impl<F: FnOnce()> ScopeGuard<F> {
    /// Creates a new scope guard
    pub fn new(cleanup: F) -> Self {
        Self {
            cleanup: Some(cleanup),
        }
    }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}

/// Creates a scope guard that runs cleanup when it goes out of scope
pub fn defer<F: FnOnce()>(cleanup: F) -> ScopeGuard<F> {
    ScopeGuard::new(cleanup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_lock_mutex_simple() {
        let mutex = Arc::new(Mutex::new(42));
        let result = lock_mutex_simple(&mutex);
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 0, 10, "test").is_ok());
        assert!(validate_range(-1, 0, 10, "test").is_err());
        assert!(validate_range(15, 0, 10, "test").is_err());
    }

    #[test]
    fn test_validate_positive() {
        assert!(validate_positive(5, "test").is_ok());
        assert!(validate_positive(0, "test").is_err());
        assert!(validate_positive(-1, "test").is_err());
    }

    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(42), "error").unwrap(), 42);
        assert!(option_to_result(None::<i32>, "error").is_err());
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10.0);

        timer.reset();
        assert!(timer.elapsed_ms() < 1.0);
    }
}
