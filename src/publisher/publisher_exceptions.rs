//! This module contains custom error types for the publisher module.
//!
//! ## Error Codes
//!
//! We use custom error codes to help with the identification of problems.
//!
//! The publisher module uses error codes in the range of 30 - 39.
//!
//! |Code|Name   |Explanation|
//! |----|-------|-----------|
//! |`30`|--TBA--|--TBA--    |
//! |`31`|--TBA--|--TBA--    |
//! |`32`|--TBA--|--TBA--    |
//! |`33`|--TBA--|--TBA--    |
//! |`34`|--TBA--|--TBA--    |
//! |`35`|--TBA--|--TBA--    |
//! |`36`|--TBA--|--TBA--    |
//! |`37`|--TBA--|--TBA--    |
//! |`38`|--TBA--|--TBA--    |
//! |`39`|--TBA--|--TBA--    |
//!

use reqwest::Error as ReqwestError;

use std::{error::Error, fmt, process};

#[derive(Debug)]
pub enum NetBoxApiError {
    Reqwest(ReqwestError),
    Other(String), // For other types of Errors, if necessary.
}

impl fmt::Display for NetBoxApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NetBoxApiError::Reqwest(ref err) => write!(f, "Request error: {}", err),
            NetBoxApiError::Other(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl Error for NetBoxApiError {}

impl From<ReqwestError> for NetBoxApiError {
    fn from(err: ReqwestError) -> Self {
        NetBoxApiError::Reqwest(err)
    }
}

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

/// Imlement `abort` function to exit the program in an orderly manner, printing the error message
/// in the process.
impl PayloadValidationError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code);
    }
}
