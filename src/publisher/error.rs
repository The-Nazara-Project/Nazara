//! This module contains custom error types for the publisher module.

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

use std::{error::Error, fmt};

/// Variants of all Errors the API can encounter on Nazara's end.
#[derive(Debug)]
pub enum NetBoxApiError {
    /// Wraps a `reqwest::Error`. Used for handling failures with requests and responses.
    Reqwest(ReqwestError),
    /// Used to indicate the `thanix_client` version is incompatible with NetBox.
    VersionMismatch(String),
    /// Used to indicate that NetBox's initial response does not contain the application version.
    MissingVersion(String),
    /// Wraps a `serde_json::Error`. Used to handle failures with response serialization.
    JsonParse(SerdeJsonError),
    /// Expects a `String` message. Used for edge cases and general purpose error cases.
    Other(String), // For other types of Errors, if necessary.
}

impl fmt::Display for NetBoxApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NetBoxApiError::Reqwest(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Request error: {err}")
            }
            NetBoxApiError::VersionMismatch(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m API Client version error: {err}"
                )
            }
            NetBoxApiError::MissingVersion(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]x1b[0m API Client missing version error: {err}"
                )
            }
            NetBoxApiError::JsonParse(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m JSON parsing error: {err}")
            }
            NetBoxApiError::Other(ref err) => write!(f, "\x1b[31m[error]\x1b[0m {err}"),
        }
    }
}

/// Needed for `NetBoxApiError::Reqwest` and `NetBoxApiError::JsonParse` as these contain other
/// error types from dependencies.
/// Others are ignored as they originate in Nazara.
impl Error for NetBoxApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            NetBoxApiError::Reqwest(ref err) => Some(err),
            NetBoxApiError::JsonParse(ref err) => Some(err),
            NetBoxApiError::MissingVersion(_) => None,
            NetBoxApiError::VersionMismatch(_) => None,
            NetBoxApiError::Other(_) => None,
        }
    }
}

impl From<ReqwestError> for NetBoxApiError {
    fn from(err: ReqwestError) -> Self {
        NetBoxApiError::Reqwest(err)
    }
}

impl From<SerdeJsonError> for NetBoxApiError {
    fn from(err: SerdeJsonError) -> Self {
        NetBoxApiError::JsonParse(err)
    }
}
