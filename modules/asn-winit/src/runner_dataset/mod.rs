mod application_handler;
mod window_manager;

use std::sync::Arc;

use asn_gui_core::TAsnRenderManager;

use crate::WinitWindow;

pub struct RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    window: Option<Arc<WinitWindow>>,
    r: R,
}

impl<R> RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    pub fn new(r: R) -> Self {
        Self { window: None, r }
    }
}
