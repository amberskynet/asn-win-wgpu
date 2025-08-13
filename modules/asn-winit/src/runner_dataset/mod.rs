mod application_handler;
mod asn_win_manager_impl;

use winit::window::Window;

pub struct RunnerDataset {
    window: Option<Window>,
}

impl RunnerDataset {
    fn new() -> Self {
        Self { window: None }
    }
}

pub fn new_runner_dataset() -> RunnerDataset {
    RunnerDataset::new()
}
