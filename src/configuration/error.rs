//! This module provides custom exception to the config parser.

use std::io::Error as IoError;
use std::{error::Error, fmt};

use toml::{de::Error as TomlDeError, ser::Error as TomlSeError};

/// Variants of all Erros the ConfigParser can encounter.
///
/// # Variants:
///
/// * `FileOpError` - Wraps a [`std::io::Error`](std::io::Error). Used for handling errors during
///   file operations.
/// * `NoConfigFileError` - Indicates that no config file has been found, or it has been moved or
///   deleted during program startup.
/// * `MissingConfigOptionError` - Indicates that a required config option is missing from the
///   config file.
/// * `DeserializationError` - Wraps a [`toml::de::Error`](toml::de::Error). Indicates an error
///   during deserialization of the TOML config file.
/// * `SerializationError` - Wraps a [`toml::se::Error`](toml::se::Error). Indicates an error
///   during Serialization of config parameters to a TOML value.
/// * `Other` - Expects a `String` message. Used for edge cases and general purpose error cases.
#[derive(Debug)]
pub enum ConfigError {
    FileOpError(IoError),
    NoConfigFileError(String),
    MissingConfigOptionError(String),
    DeserializationError(TomlDeError),
    SerializationError(TomlSeError),
    Other(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConfigError::FileOpError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m File operation failed: {err}")
            }
            ConfigError::NoConfigFileError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m No config file found: {err}")
            }
            ConfigError::MissingConfigOptionError(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m Missing required config parameter: {err}"
                )
            }
            ConfigError::DeserializationError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Invalid config file: {err}")
            }
            ConfigError::SerializationError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Serialization error: {err}")
            }
            ConfigError::Other(ref err) => {
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
