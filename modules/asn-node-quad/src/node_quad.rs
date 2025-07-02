use asn_logger::trace;

use crate::data::LOG_MODULE_NAME;

pub struct AsnNodeQuad {}

impl AsnNodeQuad {
    pub fn new() -> Self {
        AsnNodeQuad {}
    }
    pub fn draw(&mut self) {
        trace(LOG_MODULE_NAME, format!("draw").as_str());
    }
}
