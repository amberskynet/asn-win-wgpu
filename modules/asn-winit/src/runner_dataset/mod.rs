mod application_handler;
mod runner_dataset;

use asn_gui_core::AsnGuiWindowConfig;

pub fn new_runner_dataset(config: &AsnGuiWindowConfig) -> runner_dataset::RunnerDataset {
    let _ = config;
    runner_dataset::RunnerDataset {}
}
