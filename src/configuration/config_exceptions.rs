//! # Config Exception Module
//!
//! This module provides custom exception to the config parser.
//!
//! ## Custom Error Codes
//!
//! We use custom error codes in our code to help identifying possible problems.
//!
//! The config module uses error codes in the range of 10 - 19.
//!
//! |Code  |Name                    |Explanation                                                |
//! |------|------------------------|-----------------------------------------------------------|
//! |`10`  |FileOpError             |Indicates errors with operating on the config file.        |
//! |`11`  |NoConfigFileErrror      |Configuration file could not be found.                     |
//! |`12`  |InvalidConfigFileError  |Contains a TOML-Deserialize error.                         |
//! |`13`  |SerializationError      |Constains a TOML-Serialization error.                      |
//! |`14`  |MissingConfigOptionError|A required config option is missing from the config file.|
//! |`15`  |--Undefined--           |--Undefined--                                              |
//! |`16`  |--Undefined--           |--Undefined--|
//! |`17`  |--Undefined--           |--Undefined--|
//! |`18`  |--Undefined--           |--Undefined--|
//! |`19`  |Other                   |Undefined Error type, more info in the error message.      |
//!
use std::io::Error as IoError;
use std::{error::Error, fmt, process};

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
                write!(f, "\x1b[31m[error]\x1b[0m File operation failed: {}", err)
            }
            ConfigError::NoConfigFileError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m No config file found: {}", err)
            }
            ConfigError::MissingConfigOptionError(ref err) => {
                write!(
                    f,
                    "\x1b[31m[error]\x1b[0m Missing required config parameter: {}",
                    err
                )
            }
            ConfigError::DeserializationError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Invalid config file: {}", err)
            }
            ConfigError::SerializationError(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Serialization error: {}", err)
            }
            ConfigError::Other(ref err) => {
                write!(f, "\x1b[31m[error]\x1b[0m Config Parser error: {}", err)
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfigError::FileOpError(ref err) => Some(err),
            ConfigError::NoConfigFileError(_) => None,
            ConfigError::MissingConfigOptionError(_) => None,
            ConfigError::DeserializationError(ref err) => Some(err),
            ConfigError::SerializationError(ref err) => Some(err),
            ConfigError::Other(_) => None,
        }
    }
}

impl From<IoError> for ConfigError {
    fn from(err: IoError) -> Self {
        ConfigError::FileOpError(err)
    }
}

impl From<TomlDeError> for ConfigError {
    fn from(err: TomlDeError) -> Self {
        ConfigError::DeserializationError(err)
    }
}

impl From<TomlSeError> for ConfigError {
    fn from(err: TomlSeError) -> Self {
        ConfigError::SerializationError(err)
    }
}

impl ConfigError {
    /// Abort the process, if necessary.
    ///
    /// If no `exit_code` is given, will try to detect one depending on the Error variant used.
    ///
    /// # parameters
    ///
    /// * `&self`
    /// * `exit_code: Option<i32>` - The code which the application should output when exiting. If
    /// none will try to detect it depending on the error variant.
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
            ConfigError::FileOpError(_) => 10,
            ConfigError::NoConfigFileError(_) => 11,
            ConfigError::DeserializationError(_) => 12,
            ConfigError::SerializationError(_) => 13,
            ConfigError::MissingConfigOptionError(_) => 14,
            ConfigError::Other(_) => 19,
        }
    }
}
