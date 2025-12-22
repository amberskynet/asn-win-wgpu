//! Интеграционные тесты для asn-core модуля

use asn_core::{asn_event, loading_state::LoadingState, transform_set::TransformSet};
use cgmath::Vector3;

#[test]
fn test_transform_set_matrix_calculation() {
    let transform = TransformSet {
        pos: Vector3::new(1.0, 2.0, 3.0),
        rot: Vector3::new(0.0, 0.0, 0.0),
        scale: Vector3::new(2.0, 2.0, 2.0),
    };

    let matrix = transform.matrix_calculated();

    // Проверяем, что матрица является валидной 4x4 матрицей
    assert_eq!(matrix.x.x, 2.0);
    assert_eq!(matrix.y.y, 2.0);
    assert_eq!(matrix.z.z, 2.0);
    assert_eq!(matrix.w.w, 1.0);

    // Проверяем позицию
    assert_eq!(matrix.w.x, 1.0);
    assert_eq!(matrix.w.y, 2.0);
    assert_eq!(matrix.w.z, 3.0);
}

#[test]
fn test_loading_state_transitions() {
    let mut state = LoadingState::Zero;

    // Проверяем начальное состояние
    assert!(matches!(state, LoadingState::Zero));

    // Переходим в пустое состояние
    state = LoadingState::Empty("test data".to_string());
    assert!(matches!(state, LoadingState::Empty(ref data) if data == "test data"));

    // Переходим в загруженное состояние
    state = LoadingState::Loaded("loaded data".to_string());
    assert!(matches!(state, LoadingState::Loaded(ref data) if data == "loaded data"));
}

#[test]
fn test_open_gl_to_wgpu_matrix() {
    // Проверяем, что OPENGL_TO_WGPU_MATRIX определена и является корректной
    let matrix = asn_core::OPENGL_TO_WGPU_MATRIX;

    // Это должна быть корректная матрица преобразования
    assert_eq!(matrix.x.x, 1.0);
    assert_eq!(matrix.y.y, 1.0);
    assert_eq!(matrix.z.z, 0.5);
    assert_eq!(matrix.z.w, 0.0);
    assert_eq!(matrix.w.z, 0.5);
    assert_eq!(matrix.w.w, 1.0);
}

#[test]
fn test_event_system() {
    // Тестируем систему событий
    let event = asn_event::AsnEvent::WindowEvent(asn_event::AsnWindowEvent::CloseRequested);

    match event {
        asn_event::AsnEvent::WindowEvent(asn_event::AsnWindowEvent::CloseRequested) => {
            // Корректно обработано
        }
        _ => panic!("Unexpected event type"),
    }

    // Тестируем клавиатурное событие
    let keyboard_event =
        asn_event::AsnEvent::KeyboardEvent(asn_event::AsnKeyboardEvent::Pressed(42));
    match keyboard_event {
        asn_event::AsnEvent::KeyboardEvent(asn_event::AsnKeyboardEvent::Pressed(code)) => {
            assert_eq!(code, 42);
        }
        _ => panic!("Unexpected keyboard event type"),
    }
}

#[tokio::test]
async fn test_async_loading_state() {
    use std::time::Duration;
    use tokio::time::sleep;

    let mut state = LoadingState::Zero;

    // Имитируем асинхронную загрузку
    state = LoadingState::Empty("initial".to_string());
    sleep(Duration::from_millis(10)).await;

    state = LoadingState::Loaded("loaded data".to_string());
    sleep(Duration::from_millis(10)).await;

    assert!(matches!(state, LoadingState::Loaded(ref data) if data == "loaded data"));
}

#[test]
fn test_matrix_multiplication() {
    let transform1 = TransformSet {
        pos: Vector3::new(1.0, 0.0, 0.0),
        rot: Vector3::new(0.0, 0.0, 0.0),
        scale: Vector3::new(1.0, 1.0, 1.0),
    };

    let transform2 = TransformSet {
        pos: Vector3::new(0.0, 1.0, 0.0),
        rot: Vector3::new(0.0, 0.0, 0.0),
        scale: Vector3::new(2.0, 2.0, 2.0),
    };

    let matrix1 = transform1.matrix_calculated();
    let matrix2 = transform2.matrix_calculated();

    let combined = matrix1 * matrix2;

    // Проверяем, что комбинированная матрица корректна
    assert_eq!(combined.w.x, 1.0); // x из transform1
    assert_eq!(combined.w.y, 1.0); // y из transform2
    assert_eq!(combined.x.x, 2.0); // scale из transform2
    assert_eq!(combined.y.y, 2.0);
    assert_eq!(combined.z.z, 2.0);
}
