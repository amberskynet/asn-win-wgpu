#[derive(Default)]
pub struct AsnWindow {}

#[derive(Default)]
pub struct State {
    pub window: AsnWindow,
}

impl State {
    pub fn resize(&mut self, _width: u32, _height: u32) {
        // We'll do stuff here in the next tutorial
    }

    pub fn render(&mut self) {
        // self.window.request_redraw();
        // We'll do more stuff here in the next tutorial
    }
}
