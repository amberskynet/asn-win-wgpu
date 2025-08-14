use crate::data::LOG_MODULE_NAME;

use super::RenderManager;
use super::frame_context::WgpuFrameContext;
use super::wgpu_context::WgpuContext;

use asn_gui_core::TAsnRenderManager;
use asn_logger::*;
use asn_winit::WinitWindow;

impl TAsnRenderManager for RenderManager {
    type FrameContext = WgpuFrameContext;
    type Window = WinitWindow;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>> {
        let r = match self.s.as_mut() {
            Some(r) => r,
            None => {
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:begin_frame error - manager not initialized"
                ))));
            }
        };

        let fcx = match WgpuFrameContext::new(&r.surface, &r.device) {
            Ok(fcx) => fcx,
            Err(e) => {
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:begin_frame error - {e}"
                ))));
            }
        };

        Ok(fcx)
    }

    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>> {
        let _ = fcx;
        Ok(())
    }

    fn init(&mut self, w: std::sync::Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let context = match pollster::block_on(WgpuContext::new(w)) {
            Ok(context) => context,
            Err(e) => {
                m_error!("Failed to create GPU state: {e}");
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:init error: {e}"
                ))));
            }
        };

        self.s = Some(context);

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let s = match self.s.as_mut() {
            Some(s) => s,
            None => {
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:resize error - manager not initialized"
                ))));
            }
        };

        if let Err(e) = s.resize(width, height) {
            return Err(Box::new(std::io::Error::other(format!(
                "RenderManager:resize error - {e}"
            ))));
        }

        Ok(())
    }
}
