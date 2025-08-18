use asn_gui_core::TAsnRenderManager;
use winit::application::ApplicationHandler;

use crate::WinitWindow;

use super::{EmptyState, InitializationState, ReadyState};

impl<R> ApplicationHandler for super::InitializationState<EmptyState<R>, ReadyState<R>>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let super::InitializationState::Uninitialized(u) = self {
            let r = u.r.take().unwrap();
            let r = ReadyState { r };
            *self = InitializationState::Initialized(r);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // todo!()
    }
}
