use super::frame_context::WgpuFrameContext;

use super::RenderManager;
use asn_gui_core::TAsnRenderManager;

impl TAsnRenderManager for RenderManager {
    type FrameContext = WgpuFrameContext;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>> {
        let fcx = WgpuFrameContext {};
        Ok(fcx)
    }

    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>> {
        let _ = fcx;
        Ok(())
    }
}
