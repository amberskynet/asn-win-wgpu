mod data;
mod winit_app;

extern crate asn_logger;
extern crate winit;

use asn_logger::{info, init_log, warn, AsnLogConfig, AsnLogLevel};
use data::LOG_MODULE_NAME;
use winit::event_loop::{ControlFlow, EventLoop};
use winit_app::App;

pub fn run() {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };
    init_log(&c);

    info(LOG_MODULE_NAME, "try run");

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let result = event_loop.run_app(&mut app);

    match result {
        Ok(_) => info(LOG_MODULE_NAME, "exit successful"),
        Err(e) => {}
    }
}
