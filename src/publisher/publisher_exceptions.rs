//! This module contains custom error types for the publisher module.
//!
//! ## Error Codes
//!
//! We use custom error codes to help with the identification of problems.
//!
//! The publisher module uses error codes in the range of 30 - 39.
//!
//!	> [!Note]
//!	> A lot of errors come from `thanix_client` and are being escalated without explicitly casting
//!	> them into these custom types.
//!
//! |Code|Name   |Explanation|
//! |----|-------|-----------|
//! |`30`|ReqwestError|Implies an error with sending or receiving information to NetBox.|
//! |`31`|JsonParse|Indicates erro while attempting to parse the JSON response.|
//! |`32`|MissingVersion|Indicates that NetBox did not send the application version in the probing
//! step.|
//! |`33`|VersionMismatch|Indicates that the used `thanix_client` version is incompatible with
//! NetBox.|
//! |`34`|--TBA--|--TBA--    |
//! |`35`|--TBA--|--TBA--    |
//! |`36`|--TBA--|--TBA--    |
//! |`37`|--TBA--|--TBA--    |
//! |`38`|--TBA--|--TBA--    |
//! |`39`|--TBA--|--TBA--    |
//!

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

use std::{error::Error, fmt, process};

/// Variants of all Errors the API can encounter on Nazara's end.
///
/// # Variants:
///
/// * `Reqwest` - Wraps a `reqwest::Error`. Used for handling failures with requests and responses.
/// * `VersionMismatch` - Used to indicate the `thanix_client` version is incompatible with NetBox.
/// * `MissingVersion` - Used to indicate that NetBox's initial response does not contain the
/// application version.
/// * `JsonParse` - Wraps a `serde_json::Error`. Used to handle failures with response
/// serialization.
/// * `Other` - Expects a `String` message. Used for edge cases and general purpose error cases.
#[derive(Debug)]
pub enum NetBoxApiError {
    Reqwest(ReqwestError),
    VersionMismatch(String),
    MissingVersion(String),
    JsonParse(SerdeJsonError),
    Other(String), // For other types of Errors, if necessary.
}

impl fmt::Display for NetBoxApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NetBoxApiError::Reqwest(ref err) => write!(f, "[error] Request error: {}", err),
            NetBoxApiError::VersionMismatch(ref err) => {
                write!(f, "[error] API Client version error: {}", err)
            }
            NetBoxApiError::MissingVersion(ref err) => {
                write!(f, "[error] API Client missing version error: {}", err)
            }
            NetBoxApiError::JsonParse(ref err) => write!(f, "[error] JSON parsing error: {}", err),
            NetBoxApiError::Other(ref err) => write!(f, "[Error] {}", err),
        }
    }
}

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

impl NetBoxApiError {
    /// Abort the process, if necessary.
    ///
    /// If no `exit_code` is given, will try to detect one depending on the Error variant used.
    ///
    /// # Parameters
    ///
    /// * `&self`
    /// * `exit_code: Option<i32>` - The code which the application should output when exiting. If
    /// none, will try to detect it depending on the error variant.
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

    /// Detect exit code depending on the error type, if none is given to `abort()`.
    fn figure_exit_code(&self) -> i32 {
        match &self {
            NetBoxApiError::Reqwest(_) => 30,
            NetBoxApiError::JsonParse(_) => 31,
            NetBoxApiError::MissingVersion(_) => 32,
            NetBoxApiError::VersionMismatch(_) => 33,
            NetBoxApiError::Other(_) => 34,
        }
    }
}

// TODO: Deprecate this.
/// Error to be thrown when the validation of an API request payload fails.
///
/// # Members
///
/// * message: `String` - The error message containing the reason for the failure.
pub struct PayloadValidationError {
    message: String,
}

/// Implements the `Format` trait for `PayloadValidationError` for easy printing.
impl fmt::Display for PayloadValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FATAL] PayloadValidationError: {}", self.message)
    }
}

/// Implement `abort` function to exit the program in an orderly manner, printing the error message
/// in the process.
impl PayloadValidationError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code);
    }
}
