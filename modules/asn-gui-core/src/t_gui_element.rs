pub trait TAsnGuiElement {
    type GraphContext;
    type FrameContext;

    fn update(&mut self, gcx: &Self::GraphContext);
    fn draw(&mut self, fcx: &mut Self::FrameContext);
}
