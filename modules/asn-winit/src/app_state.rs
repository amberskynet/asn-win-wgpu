use std::{fmt, sync::Arc};

use asn_gui_core::AsnGuiWindowConfig;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop};

use crate::{RenderManager, winit_utils::new_window};

use asn_logger::log::*;

pub enum RenderManagerState<R> {
    Zero,
    Empty(R),
    Loaded(R),
}

impl<R> fmt::Display for RenderManagerState<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderManagerState::Zero => write!(f, "RenderManagerState::Zero"),
            RenderManagerState::Empty(_) => write!(f, "RenderManagerState::Empty"),
            RenderManagerState::Loaded(_) => write!(f, "RenderManagerState::Loaded"),
        }
    }
}

impl<R> RenderManagerState<R> {
    pub fn to_loaded(self) -> RenderManagerState<R> {
        match self {
            RenderManagerState::Empty(r) => RenderManagerState::Loaded(r),
            loaded @ RenderManagerState::Loaded(_) => loaded,
            RenderManagerState::Zero => panic!("State is zero"),
        }
    }
    pub fn load(&mut self) -> RenderManagerState<R> {
        let new_state = std::mem::replace(self, RenderManagerState::Zero);
        new_state.to_loaded()
    }
}

pub fn new_state<R>(r: R) -> RenderManagerState<R>
where
    R: RenderManager,
{
    RenderManagerState::Empty(r)
}

// TODO for future change <R> to FnOnce<R> (?)

impl<R> RenderManagerState<R>
where
    R: RenderManager,
{
    pub fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Empty(r) = self {
            let conf = AsnGuiWindowConfig::default();
            let w = new_window(event_loop, &conf).unwrap();
            r.init(Arc::new(w)).unwrap();

            let s = self.load();
            *self = s;
        }
    }

    pub fn handle_close(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application close requested");
        *self = RenderManagerState::Zero;
        event_loop.exit();
    }

    /// Handles window resize events
    pub fn handle_resize(&mut self, width: u32, height: u32) {
        trace!("Resizing window to {width}x{height}");
        if let Self::Loaded(r) = self {
            match r.resize(width, height) {
                Ok(_) => {}
                Err(err) => {
                    error!("handle_redraw draw failed: {err}");
                }
            }
        }
    }

    pub fn handle_redraw(&mut self) {
        if let Self::Loaded(r) = self {
            match r.draw() {
                Ok(_) => {}
                Err(err) => {
                    error!("handle_redraw draw failed: {err}");
                }
            }
        }
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
                    info!("Escape key pressed - closing application");
                    self.handle_close(event_loop);
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F11) => {
                    info!("F11 key pressed - toggling fullscreen");
                    // TODO: Implement fullscreen toggle
                    info!("Fullscreen toggle not yet implemented");
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F1) => {
                    info!("F1 key pressed - showing help");
                    // TODO: Implement help system
                }
                winit::keyboard::Key::Character("r") | winit::keyboard::Key::Character("R") => {
                    info!("R key pressed...");
                }
                _ => {
                    trace!("Key pressed: {:?}", event.logical_key);
                }
            }
        }
    }
}

impl<R> ApplicationHandler for RenderManagerState<R>
where
    R: RenderManager,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        trace!("ApplicationHandler resumed: {self}");
        self.resumed(event_loop);
        trace!("ApplicationHandler resumed: {self}");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                trace!("CloseRequested event");
                self.handle_close(event_loop);
            }
            WindowEvent::RedrawRequested => {
                // trace(LOG_MODULE_NAME, &format!("RedrawRequested event"));
                self.handle_redraw();
            }
            WindowEvent::Resized(size) => {
                trace!("Resized event: {size:?}");
                self.handle_resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                trace!("KeyboardInput event: {event:?}");
                self.handle_keyboard_input(event_loop, event);
            }
            WindowEvent::Focused(focused) => {
                trace!("Window focus changed: {id:?}");
                trace!("Window focus changed: {focused}");
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                trace!("Scale factor changed: {scale_factor}");
            }
            _ => {
                trace!("Window {id:?} event: {event:?}");
            }
        }
    }
}
