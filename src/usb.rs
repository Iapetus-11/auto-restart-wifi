use anyhow::{Context, Result, bail};
use regex::Regex;
use std::{
    io::{Write, pipe},
    process::{Command, Stdio},
    sync::LazyLock,
    thread::sleep,
    time::Duration,
};

use crate::config::CONFIG;

static LSUSB_LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Bus\s(?<bus>\d+)\sDevice\s(?<device>\d+):").unwrap());

fn find_adapter_bus_and_device() -> Result<(u16, u16)> {
    let lsusb_output = Command::new("lsusb").output()?;
    let lsusb_stdout = String::from_utf8_lossy(&lsusb_output.stdout);

    if !lsusb_output.status.success() {
        bail!("Command lsusb was not successful: {lsusb_stdout}")
    }

    let matched_device_lines = lsusb_stdout
        .lines()
        .filter(|line| line.contains(&CONFIG.usb_device_target))
        .collect::<Vec<_>>();
    let matched_device_line = match matched_device_lines.as_slice() {
        [matched_device_line] => matched_device_line,
        [] => {
            bail!(
                "Unable to find device for search pattern: {:?}",
                CONFIG.usb_device_target,
            );
        }
        matched_device_lines => {
            bail!(
                "Found {} different devices for search pattern: {:?}",
                matched_device_lines.len(),
                CONFIG.usb_device_target,
            );
        }
    };

    let Some(device_line_captures) = LSUSB_LINE_REGEX.captures(matched_device_line) else {
        bail!("Output from lsusb was not parseable: {matched_device_line:?}");
    };

    let Some(bus) = device_line_captures.name("bus").map(|bus| bus.as_str()) else {
        bail!("Output from lsusb was missing bus: {matched_device_line:?}");
    };
    let bus = bus.parse::<u16>().context("parsing bus as u16")?;

    let Some(device) = device_line_captures.name("device").map(|bus| bus.as_str()) else {
        bail!("Output from lsusb was missing device: {matched_device_line:?}");
    };
    let device = device.parse::<u16>().context("parsing device as u16")?;

    Ok((bus, device))
}

fn stdio_from_string(value: &str) -> Result<Stdio> {
    Ok(Stdio::from({
        let (reader, mut writer) = pipe().context("Creating pipe")?;
        writer
            .write_all(value.as_bytes())
            .context("Writing to pipe")?;
        reader
    }))
}

fn disable_usb_device(bus: u16, device: u16) -> Result<()> {
    let usb_unbind_status = Command::new("tee")
        .arg("/sys/bus/usb/drivers/usb/unbind")
        .stdin(stdio_from_string(&format!("{bus}-{device}"))?)
        .status()
        .context("Executing usb/unbind")?;

    if !usb_unbind_status.success() {
        bail!("Command usb/unbind was not successful");
    }

    Ok(())
}

fn enable_usb_device(bus: u16, device: u16) -> Result<()> {
    let usb_bind_status = Command::new("tee") // TODO: We can probably not use tee, right?
        .arg("/sys/bus/usb/drivers/usb/bind")
        .stdin(stdio_from_string(&format!("{bus}-{device}"))?)
        .status()
        .context("Executing usb/bind")?;

    if !usb_bind_status.success() {
        bail!("Command usb/bind was not successful");
    }

    Ok(())
}

pub fn toggle_wifi_adapter() -> Result<()> {
    let (bus, device) =
        find_adapter_bus_and_device().context("Finding network adapter bus and device")?;

    disable_usb_device(bus, device).context("Disabling network adapter")?;
    sleep(Duration::from_secs_f32(CONFIG.network_off_seconds));
    enable_usb_device(bus, device).context("Enabling network adapter")?;

    Ok(())
}
