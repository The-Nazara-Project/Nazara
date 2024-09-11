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
//! |`29`  |--Reserved--|Used for the `Other` error type if no other code can be defined.|
//!
use std::{error::Error, fmt, process};

use serde::Serialize;

/// Variants of all Errors the Collector might encounter.
///
/// # Variants:
///
/// * `DmiError` - Contains a String with a reason why the execution of dmidecode failed.
/// * `UnableToCollectDataError` - Used to indicate that the collection of system data failed.
/// * `InvalidNetworkInterfaceError` - Used to indicate that one of the collected NWIs might be
/// malformed or invalid
/// * `NoNetworkInterfacesError` - Used in case the NWI collector crate cannot find any interfaces.
/// * `Other` - Expects a `String` message. Used for edge cases and general purpose errors.
#[derive(Serialize, Debug)]
pub enum CollectorError {
    DmiError(String),
    UnableToCollectDataError(String),
    InvalidNetworkInterfaceError(String),
    NoNetworkInterfacesError(String),
    Other(String),
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CollectorError::DmiError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Dmi Error: {}", err)
            }
            CollectorError::UnableToCollectDataError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Collector Error: {}", err)
            }
            CollectorError::InvalidNetworkInterfaceError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Network Collector Error: {}", err)
            }
            CollectorError::NoNetworkInterfacesError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Network Collector Error: {}", err)
            }
            CollectorError::Other(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Collector Error: {}", err)
            }
        }
    }
}

impl Error for CollectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}

impl CollectorError {
    /// Abort the process, if necessary.
    ///
    /// If no `exit_code` is given, will try to detect one depending on the Error variant used.
    ///
    /// # Parameters
    ///
    /// * `&self`
    /// * `exit_code: Option<i32>` - The code which the application should output when exiting. If
    /// `None`, will try to detect it depending on the error type.
    ///
    /// # Returns
    ///
    /// This function does not return.
    pub fn abort(&self, exit_code: Option<i32>) -> ! {
        let code: i32;
        if exit_code.is_none() {
            code = self.figure_exit_code();
        } else {
            code = exit_code.unwrap();
        }

        eprintln!("{} (Error code: {})", self, code);
        process::exit(code);
    }

    /// Detect exit code depending on the error type, if non is given to `abort()`.
    fn figure_exit_code(&self) -> i32 {
        match &self {
            CollectorError::DmiError(_) => 20,
            CollectorError::UnableToCollectDataError(_) => 21,
            CollectorError::InvalidNetworkInterfaceError(_) => 25,
            CollectorError::NoNetworkInterfacesError(_) => 26,
            CollectorError::Other(_) => 29,
        }
    }
}

// /// Handles general errors with collecting information.
// ///
// /// Either because the command is unavailable, requires sudo privileges or other failures.
// ///
// /// As this is an Error this cannot be recovered from and the program must abort.
// pub struct UnableToCollectDataError {
//     pub message: String,
// }

// impl fmt::Display for UnableToCollectDataError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// impl UnableToCollectDataError {
//     pub fn abort(&self, exit_code: i32) -> ! {
//         println!("{} (Error code: {})", self, exit_code);
//         process::exit(exit_code)
//     }
// }

// /// This exception shall be raised whenever a Network Interface cannot be identified.
// ///
// /// Usually because some or all parameters such as name, addr or mac_addr are missing.
// ///
// /// As this is an Error this cannot be recovered from and the program must abort.
// pub struct InvalidNetworkInterfaceError {
//     pub message: String,
// }

// impl fmt::Display for InvalidNetworkInterfaceError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// impl InvalidNetworkInterfaceError {
//     pub fn abort(&self, exit_code: i32) -> ! {
//         println!("{} (Error code: {})", self, exit_code);
//         process::exit(exit_code)
//     }
// }

// /// This exception will be raised if no Network Interfaces can be found, so if the returned vector is empty.
// ///
// /// This is not a unrecoverable error.
// #[derive(Debug)]
// pub struct NoNetworkInterfacesException {
//     pub message: String,
// }

// impl fmt::Display for NoNetworkInterfacesException {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }
