use crate::data::LOG_MODULE_NAME;
use std::sync::Arc;

use asn_logger::{info, trace};
use asn_wgpu::State;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    // window: Arc<Window>,
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let w = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let s = pollster::block_on(State::new(Arc::clone(&w)));

        // self.window = w;

        self.state = Some(s);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info(LOG_MODULE_NAME, "The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                // self.window.as_ref().request_redraw();
                self.state.as_mut().unwrap().render();
            }
            _ => {
                let mess = format!("{id:?} {event:?}");
                trace(LOG_MODULE_NAME, mess.as_str());
            }
        }
    }
}
