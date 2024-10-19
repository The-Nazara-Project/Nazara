//! # Pluginhandler
//! This module is responsible for executing plugin scripts which collect the user's
//! `custom_fields` attributes for their Devices, Interfaces and IPAddresses.
//!
//! Currently, Nazara is set to handle `bash`, `python` and `Lua` scripts.

use std::error::Error;
use std::fs;
use std::process::Command;

use crate::collectors::collector_exceptions::CollectorError;
use serde_json::{self, Value};

/// Execute a given script.
///
/// # Parameters
///
/// * `path: PathBuf` - The Path of the script to execute relative to the CWD. (If none, plugins
///   directory will be searched.)
pub fn execute(path: &str) -> Result<Value, Box<dyn Error>> {
    println!("Attempting to execute plugin at path '{}'...", path);
    if !fs::metadata(path)?.is_file() {
        return Err("Provided path does not lead to file.".into());
    }

    let output = Command::new("bash").arg(path).output()?;

    if !output.status.success() {
        let err = CollectorError::PluginExecutionError("Either you have a syntax error in your code or the file does not exist.".to_string());
        return Err(err.into());
    }

    let stdout_str = String::from_utf8(output.stdout)?;

    let json_output: Value = serde_json::from_str(&stdout_str)?;

    Ok(json_output)
}

/// Validate the output of the given Plugin to make sure it is valid JSON.
///
/// # Parameters
///
/// * TODO
///
/// # Returns
///
/// * `Ok(())` if the output is valid JSON.
/// * `Err(collector_exceptions::CollectorError::InvalidPluginOutputError)` if the output is not
///   valid JSON.
fn validate() -> Result<(), CollectorError> {
	todo!()
}
