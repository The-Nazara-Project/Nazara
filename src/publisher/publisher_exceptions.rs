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

use std::{error::Error, fmt};

use crate::configuration;

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
