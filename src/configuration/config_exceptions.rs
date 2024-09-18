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
//! |`10`  |ConfigFileError         |Indicates errors with operating on the config file.        |
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

/// This error is raised when the program cannot create a config file.
///
/// As a config file is required for the program's operation, it must abort.
pub struct UnableToCreateConfigError {
    pub message: String,
}

impl fmt::Display for UnableToCreateConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl UnableToCreateConfigError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
    }
}

/// This exception is raised when the program cannot write to the config file.
///
/// E.g. when the user changes config parameters via the CLI.
pub struct ConfigWriteException {
    pub message: String,
}

impl fmt::Display for ConfigWriteException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// This error is raised when the config fil cannot be read for an unknown reason.
pub struct UnableToReadConfigError {
    pub message: String,
}

impl fmt::Display for UnableToReadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl UnableToReadConfigError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
    }
}

/// This error is raised when the configuration file is not found.
///
/// This error is unrecoverable.
pub struct NoConfigFileError {
    pub message: String,
}

impl fmt::Display for NoConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl NoConfigFileError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
    }
}

/// If the config file does not have valid syntax, the program mus abort and prompt
/// the user to fix it.
pub struct InvalidConfigFileError {
    pub message: String,
}

impl fmt::Display for InvalidConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl InvalidConfigFileError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
    }
}

/// If the config file is empty or contains empty fields, and no CLI parameters are given, this error shall be raised.
pub struct NoConfigError {
    pub message: String,
}

impl fmt::Display for NoConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl NoConfigError {
    pub fn abort(&self, exit_code: i32) -> ! {
        println!("{} (Error code: {})", self, exit_code);
        process::exit(exit_code)
    }
}
