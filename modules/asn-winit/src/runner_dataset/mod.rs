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
    fn new(r: R) -> Self {
        Self { window: None, r }
    }
}

pub fn new_runner_dataset<R>(r: R) -> RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    RunnerDataset::new(r)
}
