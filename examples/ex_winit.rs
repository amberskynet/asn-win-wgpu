extern crate asn_logger;
extern crate asn_winit;

mod log_utils;
use asn_logger::info;
use asn_winit::*;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = "ex_winit";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log();

    info(LOG_MODULE_NAME, "hello from main()");

    run();

    Ok(())
}
