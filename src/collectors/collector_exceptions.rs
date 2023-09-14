//! ## Collector Exception Module
//!
//! This module provides exceptions to the information collectors.
use std::{fmt, process};

/// This error handles general errors with collecting information.
///
/// Either because the command is unavailable, requires sudo privileges or other failures.
///
/// As this is an Error this cannot be recovered from and the program must abort.
pub struct UnableToCollectDataError {
    pub message: String,
}

impl fmt::Display for UnableToCollectDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl UnableToCollectDataError {
    pub fn abort(&self) -> ! {
        println!("{}", self);
        process::exit(1)
    }
}

/// This exception shall be raised whenever a Network Interface cannot be identified.
///
/// Usually because some or all parameters such as name, addr or mac_addr are missing.
///
/// As this is an Error this cannot be recovered from and the program must abort.
pub struct InvalidNetworkInterfaceError {
    pub message: String,
}

impl fmt::Display for InvalidNetworkInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl InvalidNetworkInterfaceError {
    pub fn abort(&self) -> ! {
        println!("{}", self);
        process::exit(1)
    }
}

/// This exception will be raised if no Network Interfaces can be found, so if the returned vector is empty.
///
/// This is not a unrecoverable error.
#[derive(Debug)]
pub struct NoNetworkInterfacesException {
    pub message: String,
}

impl fmt::Display for NoNetworkInterfacesException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
