pub trait TAsnGuiHandler {
    type GraphContext;
    type FrameContext;

    fn init(&mut self, gcx: &Self::GraphContext);

    fn update(&mut self, gcx: &Self::GraphContext);

    fn draw(&mut self, fcx: &mut Self::FrameContext);
}
