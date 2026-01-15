use std::sync::LazyLock;

use clap::Parser;

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(long, short = 'i', default_value_t = 2.0)]
    pub test_interval_seconds: f32,
    #[arg(long, short = 'a')]
    pub test_address: String,
    #[arg(long, short = 't')]
    pub usb_device_target: String,
    #[arg(long, short = 'd', default_value_t = 2.0)]
    pub network_off_seconds: f32,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::parse());
