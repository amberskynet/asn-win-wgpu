use asn_gui_core::AsnGuiWindowConfig;
use asn_logger::m_error;

use crate::data::LOG_MODULE_NAME;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

pub fn new_window(
    event_loop: &ActiveEventLoop,
    conf: &AsnGuiWindowConfig,
) -> Result<Window, Box<dyn std::error::Error>> {
    // Проверка на допустимые размеры окна
    if conf.window_width <= 0 || conf.window_height <= 0 {
        let error_msg = format!(
            "Invalid window dimensions: {}x{}",
            conf.window_width, conf.window_height
        );
        m_error!("{}", error_msg);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            error_msg
        ).into());
    }

    let window_attributes = WindowAttributes::default()
        .with_title(&conf.window_title)
        .with_inner_size(winit::dpi::LogicalSize::new(
            conf.window_width,
            conf.window_height,
        ))
        .with_resizable(true)
        .with_decorations(true);

    event_loop
        .create_window(window_attributes)
        .map_err(|e| {
            let error_msg = format!("Failed to create window: {e}");
            m_error!("{}", error_msg);
            std::io::Error::other(error_msg).into()
        })
}
