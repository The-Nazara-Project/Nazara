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
//! |`22`  |InvalidNetworkInterfaceError|Unable to identify a Network Interface as such.|
//! |`23`  |NoNetworkInterfacesException|Unable to find any Network Interfaces.|
//! |`24`  |--Undefined--|--Undefined--|
//! |`25`  |--Undefined--|--Undefined--|
//! |`26`  |UnableToParseUTF8|Nazara was unable to parse the output of your plugin from utf8.|
//! |`27`  |InvalidPluginOutputError|Your Plugin did not return valid JSON output.|
//! |`28`  |PluginExecutionError|Nazara was unable to execute your Plugin script.|
//! |`29`  |--Reserved--|Used for the `Other` error type if no other code can be defined.|
//!
use std::{error::Error, fmt, process, string::FromUtf8Error};

use serde_json::Error as SerdeJsonError;

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
#[derive(Debug)]
pub enum CollectorError {
    DmiError(String),
    UnableToCollectDataError(String),
    InvalidNetworkInterfaceError(String),
    NoNetworkInterfacesError(String),
	UnableToParseUTF8(FromUtf8Error),
    InvalidPluginOutputError(SerdeJsonError),
	PluginExecutionError(String),
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
			CollectorError::UnableToParseUTF8(ref err) => {
				write!(f, "\x1b[31m[error]\x1b[0m Unable to parse stdout from UTF8 to string: {}", err)
			}
            CollectorError::InvalidPluginOutputError(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m Plugin returned invalid JSON: {}",
                    err
                )
            }
			CollectorError::PluginExecutionError(ref err) => {
				write!(
					f,
					"\x1b[31m[error]\x1b[0m Plugin execution failed: {}",
					err
				)
			}
            CollectorError::Other(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Collector Error: {}", err)
            }
        }
    }
}

impl Error for CollectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CollectorError::DmiError(_) => None,
            CollectorError::UnableToCollectDataError(_) => None,
            CollectorError::InvalidNetworkInterfaceError(_) => None,
            CollectorError::NoNetworkInterfacesError(_) => None,
			CollectorError::UnableToParseUTF8(ref err) => Some(err),
            CollectorError::InvalidPluginOutputError(ref err) => Some(err),
			CollectorError::PluginExecutionError(_) => None,
            CollectorError::Other(_) => None,
        }
    }
}

impl From<SerdeJsonError> for CollectorError {
    fn from(err: SerdeJsonError) -> Self {
        CollectorError::InvalidPluginOutputError(err)
    }
}

impl From<FromUtf8Error> for CollectorError {
	fn from(err: FromUtf8Error) -> Self {
		CollectorError::UnableToParseUTF8(err)
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
            CollectorError::InvalidNetworkInterfaceError(_) => 22,
            CollectorError::NoNetworkInterfacesError(_) => 23,
			CollectorError::UnableToParseUTF8(_) => 26,
            CollectorError::InvalidPluginOutputError(_) => 27,
			CollectorError::PluginExecutionError(_) => 28,
            CollectorError::Other(_) => 29,
        }
    }
}

