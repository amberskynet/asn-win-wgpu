mod log_utils;

use asn_logger::m_info;
use log_utils::setup_log;

pub const LOG_MODULE_NAME: &str = file!();

fn main() {
    setup_log();

    m_info!("Hello from module");

    // let bus = new_tokio_bus();
}
