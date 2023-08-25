//! ## Config Exception Module
//!
//! This module provides custom exception to the config parser.
use std::fmt;

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
    pub fn panic(&self) -> ! {
        panic!("{}", self)
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
///
/// # Panics
///
/// When the config file cannot be read, the config parameters cannot be loaded therefore
/// the program panics.
pub struct UnableToReadConfigError {
    pub message: String,
}

impl fmt::Display for UnableToReadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl UnableToReadConfigError {
    pub fn panic(&self) -> ! {
        panic!("{}", self)
    }
}

/// This error is raised when the configuration file is not found.
///
/// This error is unrecoverable.
///
/// # Panics
///
/// If no configuration file is present when validating config, the program panics.
pub struct NoConfigFileError {
    pub message: String,
}

impl fmt::Display for NoConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl NoConfigFileError {
    pub fn panic(&self) -> ! {
        panic!("{}", self)
    }
}

/// If the config file does not have valid syntax, the program mus abort and prompt
/// the user to fix it.
///
/// # Panics
///
/// Causes the program to panic when the config file does not have valid toml syntax.
pub struct InvalidConfigFileError {
    pub message: String,
}

impl fmt::Display for InvalidConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl InvalidConfigFileError {
    pub fn panic(&self) -> ! {
        panic!("{}", self)
    }
}
