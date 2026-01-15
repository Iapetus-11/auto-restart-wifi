use std::{thread::sleep, time::Duration};

use anyhow::{Context, Result};

use crate::{config::CONFIG, network::is_network_connected, usb::toggle_wifi_adapter};

mod config;
mod network;
mod usb;

fn main() -> Result<()> {
    let test_interval_duration = Duration::from_secs_f32(CONFIG.test_interval_seconds.abs());

    loop {
        sleep(test_interval_duration);

        match is_network_connected() {
            Ok(()) => continue,
            Err(error) => {
                eprintln!("Network connection failed: {error:#?}");
            }
        }

        toggle_wifi_adapter().context("Toggling wifi adapter off and back on")?;
    }
}
