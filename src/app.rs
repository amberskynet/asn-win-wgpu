use crate::data::LOG_MODULE_NAME;
use std::sync::Arc;

use asn_logger::{error, info, trace};
use asn_wgpu::State;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

/// Main application struct that handles window events and rendering
#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    /// Called when the application is resumed (e.g., when a window is created)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .expect("Failed to create window"),
        );

        self.state = Some(
            pollster::block_on(State::new(Arc::clone(&window)))
        );
    }

    /// Handles window events like close, redraw, resize, etc.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info(LOG_MODULE_NAME, "The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
            }
            WindowEvent::Resized(size) => {
                self.handle_resize(size.width, size.height);
            }
            _ => {
                trace(LOG_MODULE_NAME, &format!("Window {id:?} event: {event:?}"));
            }
        }
    }
}

impl App {
    /// Handles the redraw event by rendering the current state
    fn handle_redraw(&mut self) {
        let Some(state) = self.state.as_mut() else {
            error(LOG_MODULE_NAME, "Cannot render: state is not initialized");
            return;
        };

        if let Err(render_error) = state.render() {
            error(LOG_MODULE_NAME, &format!("Render failed: {render_error}"));
            
            // Try to restore the surface if rendering failed
            if let Err(restore_error) = state.restore() {
                error(LOG_MODULE_NAME, &format!("Surface restore failed: {restore_error}"));
            }
        }
    }

    /// Handles window resize events
    fn handle_resize(&mut self, width: u32, height: u32) {
        let Some(state) = self.state.as_mut() else {
            error(LOG_MODULE_NAME, "Cannot resize: state is not initialized");
            return;
        };

        state.resize(width, height);
    }
}
