use asn_gui_core::TAsnRenderManager;

use crate::WinitWindow;

pub mod application_handler_impl;
mod window_event_handler;
mod winit_utils;

pub struct EmptyState<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    pub r: Option<R>,
}

pub struct ReadyState<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    pub r: R,
}

pub enum InitializationState<U, I> {
    Uninitialized(U),
    Initialized(I),
}
