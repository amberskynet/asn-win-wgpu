use asn_gui_core::TAsnRenderManager;
use asn_logger::{m_info, m_trace};
use winit::event_loop::ActiveEventLoop;

use crate::WinitWindow;

const LOG_MODULE_NAME: &str = "window_event_handler";

impl<R> super::InitializationState<super::EmptyState<R>, super::ReadyState<R>>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    pub fn handle_close(&mut self, event_loop: &ActiveEventLoop) {
        m_info!("Application close requested");

        event_loop.exit();
    }

    /// Handles window resize events
    pub fn handle_resize(&mut self, width: u32, height: u32) {
        m_trace!("Resizing window to {width}x{height}");

        // if let Err(resize_error) = self.r.resize(width, height) {
        //     m_error!("Resize failed: {resize_error}");
        // } else {
        //     m_info!("Window resized successfully to {width}x{height}");
        // }
    }

    pub fn handle_redraw(&mut self) {
        // match &self.window {
        //     Some(w) => {
        //         w.request_redraw();
        //     }
        //     None => {
        //         m_error!("window is None");
        //         return;
        //     }
        // };

        // match self.r.draw() {
        //     Ok(_) => {}
        //     Err(err) => {
        //         m_error!("handle_redraw draw failed: {err}");
        //         return;
        //     }
        // };
    }

    /// Handles keyboard input events
    pub fn handle_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: winit::event::KeyEvent,
    ) {
        use winit::event::ElementState;

        if event.state == ElementState::Pressed {
            match event.logical_key.as_ref() {
                winit::keyboard::Key::Character("Escape") => {
                    m_info!("Escape key pressed - closing application");
                    self.handle_close(event_loop);
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F11) => {
                    m_info!("F11 key pressed - toggling fullscreen");
                    // TODO: Implement fullscreen toggle
                    m_info!("Fullscreen toggle not yet implemented");
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F1) => {
                    m_info!("F1 key pressed - showing help");
                    // TODO: Implement help system
                }
                winit::keyboard::Key::Character("r") | winit::keyboard::Key::Character("R") => {
                    m_info!("R key pressed...");
                }
                _ => {
                    m_trace!("Key pressed: {:?}", event.logical_key);
                }
            }
        }
    }
}
