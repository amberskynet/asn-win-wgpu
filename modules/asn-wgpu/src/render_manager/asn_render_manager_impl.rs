use super::RenderManager;
use super::frame_context::WgpuFrameContext;
use super::wgpu_context::WgpuContext;

use asn_gui_core::TAsnRenderManager;
use asn_winit::WinitWindow;

impl TAsnRenderManager for RenderManager {
    type FrameContext = WgpuFrameContext;
    type Window = WinitWindow;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>> {
        let fcx = WgpuFrameContext {};
        Ok(fcx)
    }

    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>> {
        let _ = fcx;
        Ok(())
    }

    fn init(&mut self, w: std::sync::Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let context = WgpuContext::new(w);
        self.s = Some(context);

        Ok(())
    }
}
