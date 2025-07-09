use crate::asn_win_config::AppConfig;
use crate::data::LOG_MODULE_NAME;
use std::sync::Arc;

use asn_logger::{error, info, trace};
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
            ));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );

        let state = pollster::block_on(State::new(Arc::clone(&window)))
            .expect("Failed to create GPU state");
        self.state = Some(state);
        self.is_running = true;
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
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Returns a reference to the current configuration
    pub fn config(&self) -> &AppConfig {
        &self.config
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

        match state.draw_start() {
            Ok(mut ctx) => {
                if let Err(draw_error) = state.draw(&mut ctx) {
                    error(LOG_MODULE_NAME, &format!("Draw failed: {draw_error}"));
                    if let Err(restore_error) = state.restore() {
                        error(
                            LOG_MODULE_NAME,
                            &format!("Surface restore failed: {restore_error}"),
                        );
                    }
                    return;
                }
                if let Err(end_error) = state.draw_end(ctx) {
                    error(LOG_MODULE_NAME, &format!("Draw end failed: {end_error}"));
                    if let Err(restore_error) = state.restore() {
                        error(
                            LOG_MODULE_NAME,
                            &format!("Surface restore failed: {restore_error}"),
                        );
                    }
                }
            }
            Err(start_error) => {
                error(LOG_MODULE_NAME, &format!("Draw start failed: {start_error}"));
                if let Err(restore_error) = state.restore() {
                    error(
                        LOG_MODULE_NAME,
                        &format!("Surface restore failed: {restore_error}"),
                    );
                }
            }
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
