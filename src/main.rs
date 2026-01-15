use std::{thread::sleep, time::Duration};

use anyhow::{Context, Result, bail};

use crate::{
    config::CONFIG,
    network::{NetworkTestError, is_network_connected},
    usb::toggle_wifi_adapter,
};

mod config;
mod network;
mod usb;

fn main() -> Result<()> {
    let test_interval_duration = Duration::from_secs_f32(CONFIG.test_interval_seconds.abs());

    loop {
        sleep(test_interval_duration);

        match is_network_connected() {
            Ok(()) => continue,
            Err(NetworkTestError::InvalidTestAddress(error)) => {
                bail!(
                    "Test address {:?} is not valid: {error:?}",
                    CONFIG.test_address
                );
            }
            Err(NetworkTestError::Other(error)) => {
                eprintln!("Network connection failed: {error:#?}\n");
            }
        }

        toggle_wifi_adapter().context("Toggling wifi adapter off and back on")?;
    }
}
