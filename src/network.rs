use anyhow::{Context, Result};
use std::net::TcpStream;

use crate::config::CONFIG;

/// Returns an Err() if network is down
pub fn is_network_connected() -> Result<()> {
    TcpStream::connect(&CONFIG.test_address)
        .map(|_| ())
        .with_context(|| format!("Attempting connection to {}", CONFIG.test_address))
}
