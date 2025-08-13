extern crate asn_logger;

mod data;
use data::LOG_MODULE_NAME;

use asn_logger::*;

pub fn run() {
    info(LOG_MODULE_NAME, "run()");
}
