use asn_logger::{init_log, AsnLogConfig, AsnLogLevel};

pub fn setup_log() {
    let mut c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };


    c.module_levels
        .insert(String::from("wgpu_core"), AsnLogLevel::Off);
    c.module_levels
        .insert(String::from("wgpu_hal"), AsnLogLevel::Off);
    c.module_levels
        .insert(String::from("naga"), AsnLogLevel::Off);
    // c.module_levels
    //     .insert(String::from("asn-win-wgpu"), AsnLogLevel::Off);

    init_log(&c);

}
