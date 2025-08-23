pub trait TAsnGuiElement {
    type FrameContext;

    fn update(&mut self);
    fn draw(&mut self, fcx: &mut Self::FrameContext);
}
