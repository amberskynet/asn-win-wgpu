pub trait TAsnGuiHandler {
    // fn init(&mut self, f: &impl AsnGuiFabrica);
    fn update(&mut self);
    fn draw(&mut self);
}
