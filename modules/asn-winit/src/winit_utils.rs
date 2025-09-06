use asn_gui_core::AsnGuiWindowConfig;
#[allow(unused_imports)]
use asn_logger::{m_error, m_info};

use crate::data::LOG_MODULE_NAME;
use crate::error::{AsnWinitError, window_creation_error};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

/// Создает новое окно с заданной конфигурацией
///
/// # Аргументы
/// * `event_loop` - активный цикл событий
/// * `conf` - конфигурация окна
///
/// # Возвращает
/// Результат создания окна или ошибку AsnWinitError
pub fn new_window(
    event_loop: &ActiveEventLoop,
    conf: &AsnGuiWindowConfig,
) -> Result<Window, AsnWinitError> {
    #[allow(unused_mut)]
    let mut window_attributes = WindowAttributes::default()
        .with_title(&conf.window_title)
        .with_inner_size(winit::dpi::LogicalSize::new(
            conf.window_width,
            conf.window_height,
        ))
        .with_resizable(true)
        .with_decorations(true);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::UnwrapThrowExt;
        use winit::platform::web::WindowAttributesExtWebSys;

        const CANVAS_ID: &str = "asn-canvas";

        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
        let html_canvas_element = canvas.unchecked_into();
        window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        m_info!("window_attributes: {:?}", window_attributes);
    }

    let window = match event_loop.create_window(window_attributes) {
        Ok(window) => window,
        Err(e) => {
            m_error!("Failed to create window: {e}");
            return Err(window_creation_error(format!(
                "RunnerDataset:init_window error: {e}"
            )));
        }
    };

    Ok(window)
}
