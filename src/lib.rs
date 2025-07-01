mod app;
mod data;

extern crate asn_logger;
extern crate winit;

use app::App;
use asn_logger::{error, info};
use data::LOG_MODULE_NAME;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run() {
    info(LOG_MODULE_NAME, "try run");

    let event_loop = EventLoop::new().unwrap();

    // event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let result = event_loop.run_app(&mut app);

    match result {
        Ok(_) => info(LOG_MODULE_NAME, "exit successful"),
        Err(e) => error(LOG_MODULE_NAME, e.to_string().as_str()),
    }
}
