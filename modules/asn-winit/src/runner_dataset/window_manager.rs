use asn_gui_core::{AsnGuiWindowConfig, TAsnRenderManager, TAsnWindowManager};
use asn_logger::error;

use crate::{WinitWindow, data::LOG_MODULE_NAME, runner_dataset::RunnerDataset};

impl<R> TAsnWindowManager for RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    type AsnWindow = winit::window::Window;

    type AsnWindowContext = winit::event_loop::ActiveEventLoop;

    fn new_window(
        &mut self,
        ctx: &Self::AsnWindowContext,
        conf: &AsnGuiWindowConfig,
    ) -> Result<Self::AsnWindow, Box<dyn std::error::Error>> {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title(&conf.window_title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                conf.window_width,
                conf.window_height,
            ))
            .with_resizable(true)
            .with_decorations(true);

        let window = match ctx.create_window(window_attributes) {
            Ok(window) => window,
            Err(e) => {
                error(LOG_MODULE_NAME, &format!("Failed to create window: {e}"));
                return Err(Box::new(std::io::Error::other(format!(
                    "RunnerDataset:init_window error: {e}"
                ))));
            }
        };

        Ok(window)
    }
}
