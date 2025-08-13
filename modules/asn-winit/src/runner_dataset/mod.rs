mod application_handler;
mod asn_win_manager_impl;

use std::sync::Arc;

use asn_gui_core::TAsnRenderManager;
use winit::window::Window;

pub struct RunnerDataset<R>
where
    R: TAsnRenderManager,
{
    window: Option<Arc<Window>>,
    r: R,
}

impl<R> RunnerDataset<R>
where
    R: TAsnRenderManager,
{
    fn new(r: R) -> Self {
        Self { window: None, r }
    }
}

pub fn new_runner_dataset<R>(r: R) -> RunnerDataset<R>
where
    R: TAsnRenderManager,
{
    RunnerDataset::new(r)
}
