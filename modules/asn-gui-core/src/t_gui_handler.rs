pub trait TAsnGuiHandler {
    type FrameContext;

    fn init(&mut self);
    fn update(&mut self);

    fn draw(&mut self, fcx: Self::FrameContext);
}
