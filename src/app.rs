use crate::asn_win_config::AppConfig;
use crate::data::LOG_MODULE_NAME;
use std::sync::Arc;

use asn_logger::{error, info, trace, warn};
use asn_wgpu::State;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

/// Main application struct that handles window events and rendering
#[derive(Default)]
pub struct App {
    state: Option<State>,
    config: AppConfig,
    is_running: bool,
    frame_count: u64,
}

impl ApplicationHandler for App {
    /// Called when the application is resumed (e.g., when a window is created)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info(
            LOG_MODULE_NAME,
            "Application resumed - initializing window and state",
        );

        let window_attributes = winit::window::WindowAttributes::default()
            .with_title(&self.config.window_title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.window_width,
                self.config.window_height,
            ))
            .with_resizable(true)
            .with_decorations(true);

        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => Arc::new(window),
            Err(e) => {
                error(LOG_MODULE_NAME, &format!("Failed to create window: {e}"));
                event_loop.exit();
                return;
            }
        };

        let state = match pollster::block_on(State::new(Arc::clone(&window))) {
            Ok(state) => state,
            Err(e) => {
                error(LOG_MODULE_NAME, &format!("Failed to create GPU state: {e}"));
                event_loop.exit();
                return;
            }
        };
        
        self.state = Some(state);
        self.is_running = true;
        self.frame_count = 0;
        info(LOG_MODULE_NAME, "Application initialized successfully");
    }

    /// Handles window events like close, redraw, resize, etc.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if !self.is_running {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.handle_close(event_loop);
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
            }
            WindowEvent::Resized(size) => {
                self.handle_resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(event_loop, event);
            }
            WindowEvent::Focused(focused) => {
                trace(LOG_MODULE_NAME, &format!("Window focus changed: {focused}"));
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                trace(LOG_MODULE_NAME, &format!("Scale factor changed: {scale_factor}"));
            }
            _ => {
                trace(LOG_MODULE_NAME, &format!("Window {id:?} event: {event:?}"));
            }
        }
    }
}

#[allow(dead_code)]
impl App {
    /// Creates a new App with custom configuration
    pub fn with_config(config: AppConfig) -> Self {
        Self {
            state: None,
            config,
            is_running: false,
            frame_count: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Returns a reference to the current configuration
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Returns the current frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Handles application close
    fn handle_close(&mut self, event_loop: &ActiveEventLoop) {
        info(LOG_MODULE_NAME, "Application close requested");
        self.is_running = false;
        event_loop.exit();
    }

    /// Handles the redraw event by rendering the current state
    fn handle_redraw(&mut self) {
        let Some(state) = self.state.as_mut() else {
            error(LOG_MODULE_NAME, "Cannot render: state is not initialized");
            return;
        };

        self.frame_count += 1;

        // Start render pass
        match state.draw_start() {
            Ok(mut ctx) => {
                // Perform rendering
                if let Err(draw_error) = state.draw(&mut ctx) {
                    error(LOG_MODULE_NAME, &format!("Draw failed: {draw_error}"));
                    Self::try_restore(state);
                    return;
                }
                
                // End render pass
                if let Err(end_error) = state.draw_end(ctx) {
                    error(LOG_MODULE_NAME, &format!("Draw end failed: {end_error}"));
                    Self::try_restore(state);
                } else {
                    // Log frame rate every 60 frames
                    if self.frame_count % 60 == 0 {
                        trace(LOG_MODULE_NAME, &format!("Rendered frame {}", self.frame_count));
                    }
                }
            }
            Err(start_error) => {
                error(LOG_MODULE_NAME, &format!("Draw start failed: {start_error}"));
                Self::try_restore(state);
            }
        }
    }

    /// Handles render errors by attempting to restore the surface
    fn try_restore(state: &mut State) {
        warn(LOG_MODULE_NAME, "Attempting to restore surface after render error");
        
        if let Err(restore_error) = state.restore() {
            error(
                LOG_MODULE_NAME,
                &format!("Surface restore failed: {restore_error}"),
            );
        } else {
            info(LOG_MODULE_NAME, "Surface restored successfully after error");
        }
    }

    /// Handles window resize events
    fn handle_resize(&mut self, width: u32, height: u32) {
        let Some(state) = self.state.as_mut() else {
            error(LOG_MODULE_NAME, "Cannot resize: state is not initialized");
            return;
        };

        trace(
            LOG_MODULE_NAME,
            &format!("Resizing window to {width}x{height}"),
        );

        if let Err(resize_error) = state.resize(width, height) {
            error(LOG_MODULE_NAME, &format!("Resize failed: {resize_error}"));
        } else {
            info(LOG_MODULE_NAME, &format!("Window resized successfully to {width}x{height}"));
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
