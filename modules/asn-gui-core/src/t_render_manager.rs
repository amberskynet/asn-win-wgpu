use std::sync::Arc;

pub trait TAsnRenderManager {
    type FrameContext;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait TAsnSurface {
    type AsnWindow;

    fn init(&mut self, w: Arc<Self::AsnWindow>) -> Result<(), Box<dyn std::error::Error>>;
}
