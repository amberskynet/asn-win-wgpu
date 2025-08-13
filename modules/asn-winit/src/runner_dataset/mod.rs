mod application_handler;
mod asn_win_manager_impl;
mod t_window_manager;

use asn_gui_core::AsnGuiWindowConfig;

use winit::window::Window;

pub struct RunnerDataset {
    window: Option<Window>,
    default_window_config: AsnGuiWindowConfig,
}

impl RunnerDataset {
    fn new(config: &AsnGuiWindowConfig) -> Self {
        Self {
            window: None,
            default_window_config: config.clone(),
        }
    }
}

pub fn new_runner_dataset(config: &AsnGuiWindowConfig) -> RunnerDataset {
    let _ = config;
    RunnerDataset::new(config)
}
