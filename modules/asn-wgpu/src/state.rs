use std::sync::Arc;

use asn_logger::trace;
use winit::window::Window;

use crate::data::LOG_MODULE_NAME;

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        trace(LOG_MODULE_NAME, "new State");
        State { window }
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        trace(LOG_MODULE_NAME, "resize");
        // We'll do stuff here in the next tutorial
    }

    pub fn render(&mut self) {
        trace(LOG_MODULE_NAME, "render");
        self.window.request_redraw();
        // We'll do more stuff here in the next tutorial
    }
}
