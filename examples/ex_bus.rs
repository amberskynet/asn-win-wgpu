mod log_utils;

use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use asn_logger::m_info;
use log_utils::setup_log;
use tokio_bus::new_tokio_bus;

pub const LOG_MODULE_NAME: &str = file!();

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum TaskType {
    TaskNone,
    TaskUpdate,
}

fn main() {
    setup_log();

    m_info!("Hello from module");

    let bus = new_tokio_bus::<TaskType>(16);

    let mut recv = bus.get_receiver();
    let send = bus.get_sender();

    send.send_message(TaskType::TaskUpdate).unwrap();
    send.send_message(TaskType::TaskNone).unwrap();

    let result1 = recv.get_message().unwrap();
    let result2 = recv.get_message().unwrap();

    println!("Result: {:?}", result1);
    println!("Result: {:?}", result2);
}
