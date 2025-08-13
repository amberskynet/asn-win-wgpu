use winit::application::ApplicationHandler;

pub struct WinitApp {}

impl WinitApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
        // todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let _ = event;
        let _ = window_id;
        let _ = event_loop;
        // todo!()
    }
}
