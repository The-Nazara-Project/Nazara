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
        match self {
            NetBoxApiError::Reqwest(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Request error: {err}")
            }
            NetBoxApiError::VersionMismatch(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m API Client version error: {err}")
            }
            NetBoxApiError::MissingVersion(err) => {
                write!(
                    f,
                    "\x1b[31m[error]x1b[0m API Client missing version error: {err}"
                )
            }
            NetBoxApiError::JsonParse(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m JSON parsing error: {err}")
            }
            NetBoxApiError::Other(err) => write!(f, "\x1b[31m[error]\x1b[0m {err}"),
        }
    }
}

/// Needed for [`NetBoxApiError::Reqwest`] and [`NetBoxApiError::JsonParse`] as these contain error types from dependencies.
/// Others are ignored as they originate in Nazara.
impl Error for NetBoxApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NetBoxApiError::Reqwest(err) => Some(err),
            NetBoxApiError::JsonParse(err) => Some(err),
            _ => None,
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
