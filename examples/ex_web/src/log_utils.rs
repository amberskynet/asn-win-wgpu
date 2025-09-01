use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

pub fn setup_log() {
    let mut c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    c.module_levels
        .insert(String::from("wgpu_core"), AsnLogLevel::Error);
    c.module_levels
        .insert(String::from("wgpu_hal"), AsnLogLevel::Error);
    c.module_levels
        .insert(String::from("naga"), AsnLogLevel::Error);
    c.module_levels
        .insert(String::from("asn-win-wgpu"), AsnLogLevel::Error);

    c.module_levels
        .insert(String::from("wgpu_map"), AsnLogLevel::Error);

    init_log(&c);
}

#[allow(dead_code)]
pub fn main() {
    setup_log();
}
