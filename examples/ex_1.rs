extern crate asn_logger;
extern crate asn_node_quad;
extern crate asn_win_wgpu;

use asn_logger::{init_log, AsnLogConfig, AsnLogLevel};
use asn_node_quad::AsnNodeQuad;
use asn_win_wgpu::run;

fn main() {
    let mut c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    let node = AsnNodeQuad::new();

    c.module_levels
        .insert(String::from("wgpu_core"), AsnLogLevel::Off);
    c.module_levels
        .insert(String::from("wgpu_hal"), AsnLogLevel::Off);
    c.module_levels
        .insert(String::from("naga"), AsnLogLevel::Off);
    c.module_levels
        .insert(String::from("asn-win-wgpu"), AsnLogLevel::Off);

    init_log(&c);

    run()
}
