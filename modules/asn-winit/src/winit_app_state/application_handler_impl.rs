use std::sync::Arc;

use asn_gui_core::{AsnGuiWindowConfig, TAsnRenderManager};
use asn_logger::log::error;
use winit::application::ApplicationHandler;

use crate::WinitWindow;

use super::{EmptyState, InitializationState, ReadyState};

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
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let _ = event_loop;
        let _ = event;
        let _ = window_id;
    }
}
