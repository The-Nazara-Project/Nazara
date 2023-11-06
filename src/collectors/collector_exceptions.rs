//! # Collector Exception Module
//!
//! This module provides exceptions to the information collectors.
//!
//! ## Error Codes
//!
//! We use custom error codes to help with the identification of problems.
//!
//! The config module uses error codes in the range of 20 - 29.
//!
//! |Code  |Name       |Explanation                  |
//! |------|-----------|-----------------------------|
//! |`20`  |DmiError   |Unable to execute `dmidecode`|
//! |`21`  |UnableToCollectDataError|Unspecified Error with data collection. Usually appears when subprocess fails or an output is malformed.|
//! |`22`  |--Undefined--|--Undefined--|
//! |`23`  |--Undefined--|--Undefined--|
//! |`24`  |--Undefined--|--Undefined--|
//! |`25`  |InvalidNetworkInterfaceError|Unable to identify a Network Interface as such.|
//! |`26`  |NoNetworkInterfacesException|Unable to find any Network Interfaces.|
//! |`27`  |--Undefined--|--Undefined--|
//! |`28`  |--Undefined--|--Undefined--|
//! |`29`  |--Undefined--|--Undefined--|
//!
use std::{fmt, process};

/// Handles general errors with collecting information.
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
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
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
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
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
