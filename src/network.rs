use anyhow::Result;
use std::{io, net::TcpStream};

use crate::config::CONFIG;

#[derive(Debug)]
pub enum NetworkTestError {
    InvalidTestAddress(anyhow::Error),
    Other(anyhow::Error),
}

/// Returns an Err() if network is down
pub fn is_network_connected() -> Result<(), NetworkTestError> {
    match TcpStream::connect(&CONFIG.test_address) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            io::ErrorKind::InvalidInput => Err(NetworkTestError::InvalidTestAddress(error.into())),
            _ => Err(NetworkTestError::Other(error.into())),
        },
    }
}
