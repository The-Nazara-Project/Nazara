//! This module provides errors to the information collectors.

use serde_json::Error as SerdeJsonError;
use std::{error::Error, fmt, string::FromUtf8Error};

/// Errors that a collector might encounter.
#[allow(unused)]
#[derive(Debug)]
pub enum CollectorError {
    /// Contains a String with a reason why the execution of dmidecode failed.
    Dmi(String),
    /// Used to indicate that the collection of system data failed.
    UnableToCollectData(String),
    /// Used to indicate that one of the collected NWIs might be malformed or invalid.
    InvalidNetworkInterface(String),
    /// Used in case the NWI collector crate cannot find any interfaces.
    NoNetworkInterfaces(String),
    UnableToParseUTF8(FromUtf8Error),
    InvalidPluginOutput(SerdeJsonError),
    PluginExecution(String),
    /// Expects a `String` message. Used for edge cases and general purpose errors.
    Other(String),
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CollectorError::Dmi(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Dmi Error: {}", err)
            }
            CollectorError::UnableToCollectData(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Collector Error: {}", err)
            }
            CollectorError::InvalidNetworkInterface(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Network Collector Error: {}", err)
            }
            CollectorError::NoNetworkInterfaces(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Network Collector Error: {}", err)
            }
            CollectorError::UnableToParseUTF8(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m Unable to parse stdout from UTF8 to string: {}",
                    err
                )
            }
            CollectorError::InvalidPluginOutput(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m Plugin returned invalid JSON: {}",
                    err
                )
            }
            CollectorError::PluginExecution(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Plugin execution failed: {}", err)
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
            CollectorError::UnableToParseUTF8(ref err) => Some(err),
            CollectorError::InvalidPluginOutput(ref err) => Some(err),
            _ => None,
        }
    }
}
