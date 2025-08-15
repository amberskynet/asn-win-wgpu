pub trait TAsnGuiHandler {
    type GraphContext;
    type FrameContext;

    fn init(&mut self, gcx: Self::GraphContext);

    fn update(&mut self);

    fn draw(&mut self, fcx: Self::FrameContext);
}
