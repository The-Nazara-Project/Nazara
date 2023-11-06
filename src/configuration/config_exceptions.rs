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
//! |Code  |Name              |Explanation                                                |
//! |------|------------------|-----------------------------------------------------------|
//! |`10`  |PermissionDenied  |Unable to create config file on system.                    |
//! |`11`  |EmptyConfigFile   |Default config file was created but not parameters set.    |
//! |`12`  |UnableToInitConfig|An error occurred while trying to initialize config file.  |
//! |`13`  |ConfigWriteError  |An error occurred while trying to write to the config file.|
//! |`14`  |ConfigReadError   |Unable to read configuration file.                         |
//! |`15`  |TomlSyntaxError   |Your TOML is not valid. Please check for syntax errors.    |
//! |`16`  |--Undefined--     |--Undefined--|
//! |`17`  |--Undefined--     |--Undefined--|
//! |`18`  |--Undefined--     |--Undefined--|
//! |`19`  |--Undefined--     |--Undefined--|
//!
use std::{fmt, process};

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
