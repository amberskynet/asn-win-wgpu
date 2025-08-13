use std::sync::Arc;

pub trait TAsnRenderManager {
    type FrameContext;
    type Window;

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>>;
    fn begin_frame(&mut self) -> Result<Self::FrameContext, Box<dyn std::error::Error>>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), Box<dyn std::error::Error>>;
}
