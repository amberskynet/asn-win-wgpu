//! Модуль для управления состоянием приложения и обработки событий
//!
//! Этот модуль содержит реализацию обработчика событий приложения,
//! управление состоянием рендерера и обработку различных событий окна.
use std::{fmt, sync::Arc};

pub enum RenderManagerState<S> {
    Zero,
    Empty(S),
    Loaded(S),
}

impl<S> fmt::Display for RenderManagerState<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderManagerState::Zero => write!(f, "RenderManagerState::Zero"),
            RenderManagerState::Empty(_) => write!(f, "RenderManagerState::Empty"),
            RenderManagerState::Loaded(_) => write!(f, "RenderManagerState::Loaded"),
        }
    }
}

use asn_gui_core::AsnGuiWindowConfig;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop};

use crate::{WinitRenderManager, asn_winit_state::AsnWinitState, winit_utils::new_window};

use asn_logger::log::*;

pub fn new_state<R>(r: R) -> RenderManagerState<AsnWinitState<R>>
where
    R: WinitRenderManager,
{
    let s = AsnWinitState { r, is_init: false };
    RenderManagerState::Empty(s)
}

// Блок методов для обработки различных событий приложения
impl<R> RenderManagerState<R>
where
    R: WinitRenderManager,
{
    /// Обрабатывает событие возобновления работы приложения
    pub fn handle_resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let RenderManagerState::Empty(r) = self {
            let conf = AsnGuiWindowConfig::default();
            let w = new_window(event_loop, &conf).unwrap();
            r.init(Arc::new(w)).unwrap();

            *self = RenderManagerState::Loaded(r);
        }
    }

    /// Обрабатывает запрос на закрытие приложения
    pub fn handle_close(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application close requested");
        *self = RenderManagerState::Zero;
        event_loop.exit();
    }

    /// Обрабатывает событие изменения размера окна
    pub fn handle_resize(&mut self, width: u32, height: u32) {
        trace!("Resizing window to {width}x{height}");
        if let RenderManagerState::Loaded(r) = self {
            match r.resize(width, height) {
                Ok(_) => {}
                Err(err) => {
                    error!("handle_redraw draw failed: {err}");
                }
            }
        }
    }

    /// Обрабатывает запрос на перерисовку окна
    pub fn handle_redraw(&mut self) {
        if let RenderManagerState::Loaded(r) = self {
            match r.draw() {
                Ok(_) => {}
                Err(err) => {
                    error!("handle_redraw draw failed: {err}");
                }
            }
        }
    }
}

// don't change new_state(r) to new_state(f: FnOnce() -> R) -  we need external render manager for start_frame()/end_frame()

// Блок реализации ApplicationHandler
impl<R> ApplicationHandler for RenderManagerState<R>
where
    R: WinitRenderManager,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        trace!("ApplicationHandler resumed: {self}");
        self.handle_resumed(event_loop);
        trace!("ApplicationHandler resumed: {self}");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // trace!("ApplicationHandler about_to_wait");
        self.handle_redraw();
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
                // Используем новый модуль обработки клавиатуры
                crate::keyboard_handler::handle_key_event(event_loop, event);
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
