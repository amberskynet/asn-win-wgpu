use std::sync::Arc;

use asn_gui_core::{AsnGuiWindowConfig, TAsnRenderManager};
use asn_logger::{log::error, m_trace};
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::WinitWindow;

use super::{EmptyState, InitializationState, ReadyState};

const LOG_MODULE_NAME: &str = "app_handler";

impl<R> ApplicationHandler for super::InitializationState<EmptyState<R>, ReadyState<R>>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
        if let super::InitializationState::Uninitialized(u) = self {
            let mut r = u.r.take().unwrap();

            let conf = AsnGuiWindowConfig::default();

            let w = super::winit_utils::new_window(event_loop, &conf).unwrap();
            let arc_w = Arc::new(w);

            match r.init(arc_w) {
                Ok(_) => {
                    let r = ReadyState { r };
                    *self = InitializationState::Initialized(r);
                }
                Err(err) => {
                    error!("ApplicationHandler resumed error: {err}");
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                m_trace!("CloseRequested event");
                self.handle_close(event_loop);
            }
            WindowEvent::RedrawRequested => {
                // trace(LOG_MODULE_NAME, &format!("RedrawRequested event"));
                self.handle_redraw();
            }
            WindowEvent::Resized(size) => {
                m_trace!("Resized event: {size:?}");
                self.handle_resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                m_trace!("KeyboardInput event: {event:?}");
                self.handle_keyboard_input(event_loop, event);
            }
            WindowEvent::Focused(focused) => {
                m_trace!("Window focus changed: {id:?}");
                m_trace!("Window focus changed: {focused}");
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                m_trace!("Scale factor changed: {scale_factor}");
            }
            _ => {
                m_trace!("Window {id:?} event: {event:?}");
            }
        }
    }
}
