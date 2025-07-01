extern crate asn_logger;
extern crate asn_win_wgpu;

use asn_logger::{init_log, AsnLogConfig, AsnLogLevel};
use asn_win_wgpu::run;

fn main() {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };
    init_log(&c);

    run()
}
