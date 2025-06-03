//! This module provides error types for the config parser.

use std::io::Error as IoError;
use std::{error::Error, fmt};
use toml::{de::Error as TomlDeError, ser::Error as TomlSeError};

/// All errors the config parser can encounter.
#[derive(Debug)]
pub enum ConfigError {
    /// Used for handling errors during file operations.
    FileOpError(IoError),
    /// Indicates that no config file has been found, or it has been moved or deleted during program startup.
    NoConfigFileError(String),
    /// Indicates that a required config option is missing from the config file.
    MissingConfigOptionError(String),
    /// Indicates an error during deserialization of the TOML config file.
    DeserializationError(TomlDeError),
    /// Indicates an error during Serialization of config parameters to a TOML value.
    SerializationError(TomlSeError),
    /// Expects a `String` message. Used for edge cases and general purpose error cases.
    Other(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileOpError(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m File operation failed: {err}")
            }
            ConfigError::NoConfigFileError(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m No config file found: {err}")
            }
            ConfigError::MissingConfigOptionError(err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m Missing required config parameter: {err}"
                )
            }
            ConfigError::DeserializationError(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Invalid config file: {err}")
            }
            ConfigError::SerializationError(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Serialization error: {err}")
            }
            ConfigError::Other(err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Config Parser error: {err}")
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::FileOpError(err) => Some(err),
            ConfigError::NoConfigFileError(_) => None,
            ConfigError::MissingConfigOptionError(_) => None,
            ConfigError::DeserializationError(err) => Some(err),
            ConfigError::SerializationError(err) => Some(err),
            ConfigError::Other(_) => None,
        }
    }
}
