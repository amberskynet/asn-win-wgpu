use asn_gui_core::AsnGuiWindowConfig;
use asn_logger::m_error;

use crate::data::LOG_MODULE_NAME;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

pub fn new_window(
    event_loop: &ActiveEventLoop,
    conf: &AsnGuiWindowConfig,
) -> Result<Window, Box<dyn std::error::Error>> {
    let window_attributes = WindowAttributes::default()
        .with_title(&conf.window_title)
        .with_inner_size(winit::dpi::LogicalSize::new(
            conf.window_width,
            conf.window_height,
        ))
        .with_resizable(true)
        .with_decorations(true);

    let window = match event_loop.create_window(window_attributes) {
        Ok(window) => window,
        Err(e) => {
            m_error!("Failed to create window: {e}");
            return Err(Box::new(std::io::Error::other(format!(
                "RunnerDataset:init_window error: {e}"
            ))));
        }
    };

    Ok(window)
}
