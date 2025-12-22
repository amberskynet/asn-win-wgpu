//! Интеграционные тесты для asn-gui-core модуля

use asn_gui_core::{AsnGuiWindowConfig, TAsnGuiElement, TAsnGuiHandler, TAsnRenderManager};

/// Мок-объект для тестирования GUI элементов
struct MockGuiElement {
    updated: bool,
    drawn: bool,
}

impl MockGuiElement {
    fn new() -> Self {
        MockGuiElement {
            updated: false,
            drawn: false,
        }
    }
}

struct MockGraphContext;
struct MockFrameContext;

impl TAsnGuiElement for MockGuiElement {
    type GraphContext = MockGraphContext;
    type FrameContext = MockFrameContext;

    fn update(&mut self, _gcx: &Self::GraphContext) {
        self.updated = true;
    }

    fn draw(&mut self, _fcx: &mut Self::FrameContext) {
        self.drawn = true;
    }
}

/// Мок-объект для тестирования GUI обработчиков
struct MockGuiHandler {
    element: MockGuiElement,
}

impl MockGuiHandler {
    fn new() -> Self {
        MockGuiHandler {
            element: MockGuiElement::new(),
        }
    }
}

impl TAsnGuiHandler for MockGuiHandler {
    type GraphContext = MockGraphContext;
    type FrameContext = MockFrameContext;

    fn init(&mut self, _gcx: &Self::GraphContext) {
        // Инициализация
    }

    fn update(&mut self, _gcx: &Self::GraphContext) {
        self.element.update(_gcx);
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        self.element.draw(fcx);
    }
}

/// Мок-окно для тестирования
struct MockWindow;

impl TAsnRenderManager for MockWindow {
    type Window = MockWindow;

    fn init(&mut self, _w: std::sync::Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn resize(&mut self, _width: u32, _height: u32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[test]
fn test_gui_element_lifecycle() {
    let mut element = MockGuiElement::new();
    let gcx = MockGraphContext;
    let mut fcx = MockFrameContext;

    // Проверяем начальное состояние
    assert!(!element.updated);
    assert!(!element.drawn);

    // Вызываем update
    element.update(&gcx);
    assert!(element.updated);
    assert!(!element.drawn);

    // Вызываем draw
    element.draw(&mut fcx);
    assert!(element.updated);
    assert!(element.drawn);
}

#[test]
fn test_gui_handler_lifecycle() {
    let mut handler = MockGuiHandler::new();
    let gcx = MockGraphContext;
    let mut fcx = MockFrameContext;

    // Проверяем начальное состояние
    assert!(!handler.element.updated);
    assert!(!handler.element.drawn);

    // Инициализируем
    handler.init(&gcx);

    // Обновляем
    handler.update(&gcx);
    assert!(handler.element.updated);
    assert!(!handler.element.drawn);

    // Отрисовываем
    handler.draw(&mut fcx);
    assert!(handler.element.updated);
    assert!(handler.element.drawn);
}

#[test]
fn test_render_manager_basic_operations() {
    let mut manager = MockWindow;
    let window = std::sync::Arc::new(MockWindow);

    // Тестируем инициализацию
    assert!(manager.init(window).is_ok());

    // Тестируем изменение размера
    assert!(manager.resize(800, 600).is_ok());

    // Тестируем обновление
    assert!(manager.update().is_ok());

    // Тестируем отрисовку
    assert!(manager.draw().is_ok());
}

#[test]
fn test_window_config_defaults() {
    let config = AsnGuiWindowConfig::default();

    // Проверяем значения по умолчанию
    assert_eq!(config.window_title, "ASN WGPU Application");
    assert_eq!(config.window_width, 800);
    assert_eq!(config.window_height, 600);
}

#[test]
fn test_window_config_builder_pattern() {
    let config = AsnGuiWindowConfig {
        window_title: "Test Window".to_string(),
        window_width: 1024,
        window_height: 768,
    };

    assert_eq!(config.window_title, "Test Window");
    assert_eq!(config.window_width, 1024);
    assert_eq!(config.window_height, 768);
}

#[tokio::test]
async fn test_async_gui_operations() {
    use tokio::time::{Duration, sleep};

    let mut handler = MockGuiHandler::new();
    let gcx = MockGraphContext;
    let mut fcx = MockFrameContext;

    // Имитируем асинхронные операции GUI
    handler.init(&gcx);
    sleep(Duration::from_millis(1)).await;

    handler.update(&gcx);
    sleep(Duration::from_millis(1)).await;

    handler.draw(&mut fcx);

    // Проверяем, что все операции выполнены
    assert!(handler.element.updated);
    assert!(handler.element.drawn);
}

#[test]
fn test_multiple_gui_elements() {
    let mut elements = vec![
        MockGuiElement::new(),
        MockGuiElement::new(),
        MockGuiElement::new(),
    ];

    let gcx = MockGraphContext;
    let mut fcx = MockFrameContext;

    // Обновляем все элементы
    for element in &mut elements {
        element.update(&gcx);
    }

    // Проверяем, что все обновлены
    for element in &elements {
        assert!(element.updated);
        assert!(!element.drawn);
    }

    // Отрисовываем все элементы
    for element in &mut elements {
        element.draw(&mut fcx);
    }

    // Проверяем, что все отрисованы
    for element in &elements {
        assert!(element.updated);
        assert!(element.drawn);
    }
}
