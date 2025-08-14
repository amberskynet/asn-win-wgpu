use std::sync::Arc;

use asn_gui_core::{AsnGuiWindowConfig, TAsnRenderManager, TAsnWindowManager};
use asn_logger::{error, info, trace, warn};

use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::{WinitWindow, data::LOG_MODULE_NAME, runner_dataset::RunnerDataset};

impl<R> ApplicationHandler for RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // This method is called when the event loop is about to wait for new events.
        // You can use this to request a redraw if your application needs continuous rendering.
        if let Some(window) = &self.window {
            // trace(LOG_MODULE_NAME, &format!("about_to_wait"));
            window.request_redraw();
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let conf = AsnGuiWindowConfig::default();

            let w = self.new_window(event_loop, &conf).unwrap();
            let arc_w = Arc::new(w);

            self.r.init(arc_w.clone()).unwrap();

            self.window = Some(arc_w);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                trace(LOG_MODULE_NAME, &format!("CloseRequested event"));
                self.handle_close(event_loop);
            }
            WindowEvent::RedrawRequested => {
                // trace(LOG_MODULE_NAME, &format!("RedrawRequested event"));
                self.handle_redraw();
            }
            WindowEvent::Resized(size) => {
                trace(LOG_MODULE_NAME, &format!("Resized event: {size:?}"));
                self.handle_resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                trace(LOG_MODULE_NAME, &format!("KeyboardInput event: {event:?}"));
                self.handle_keyboard_input(event_loop, event);
            }
            WindowEvent::Focused(focused) => {
                trace(LOG_MODULE_NAME, &format!("Window focus changed: {id:?}"));
                trace(LOG_MODULE_NAME, &format!("Window focus changed: {focused}"));
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                trace(
                    LOG_MODULE_NAME,
                    &format!("Scale factor changed: {scale_factor}"),
                );
            }
            _ => {
                trace(LOG_MODULE_NAME, &format!("Window {id:?} event: {event:?}"));
            }
        }
    }
}

impl<R> RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    /// Handles application close
    fn handle_close(&mut self, event_loop: &ActiveEventLoop) {
        info(LOG_MODULE_NAME, "Application close requested");
        event_loop.exit();
    }

    /// Handles window resize events
    fn handle_resize(&mut self, width: u32, height: u32) {
        trace(
            LOG_MODULE_NAME,
            &format!("Resizing window to {width}x{height}"),
        );

        if let Err(resize_error) = self.r.resize(width, height) {
            error(LOG_MODULE_NAME, &format!("Resize failed: {resize_error}"));
        } else {
            info(
                LOG_MODULE_NAME,
                &format!("Window resized successfully to {width}x{height}"),
            );
        }
    }

    fn handle_redraw(&mut self) {
        match &self.window {
            Some(w) => {
                w.request_redraw();
            }
            None => {
                error(LOG_MODULE_NAME, &format!("window is None"));
                return;
            }
        };

        let fcx = match self.r.begin_frame() {
            Ok(fcx) => fcx,
            Err(e) => {
                error(
                    LOG_MODULE_NAME,
                    &format!("handle_redraw begin_frame failed: {e}"),
                );
                return;
            }
        };

        match self.r.end_frame(fcx) {
            Ok(_) => {}
            Err(e) => {
                error(
                    LOG_MODULE_NAME,
                    &format!("handle_redraw end_frame failed: {e}"),
                );
                return;
            }
        }
    }

    /// Handles keyboard input events
    fn handle_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: winit::event::KeyEvent,
    ) {
        use winit::event::ElementState;

        if event.state == ElementState::Pressed {
            match event.logical_key.as_ref() {
                winit::keyboard::Key::Character("Escape") => {
                    info(LOG_MODULE_NAME, "Escape key pressed - closing application");
                    self.handle_close(event_loop);
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F11) => {
                    info(LOG_MODULE_NAME, "F11 key pressed - toggling fullscreen");
                    // TODO: Implement fullscreen toggle
                    warn(LOG_MODULE_NAME, "Fullscreen toggle not yet implemented");
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F1) => {
                    info(LOG_MODULE_NAME, "F1 key pressed - showing help");
                    // TODO: Implement help system
                }
                winit::keyboard::Key::Character("r") | winit::keyboard::Key::Character("R") => {
                    info(LOG_MODULE_NAME, "R key pressed...");
                }
                _ => {
                    trace(
                        LOG_MODULE_NAME,
                        &format!("Key pressed: {:?}", event.logical_key),
                    );
                }
            }
        }
    }
}
