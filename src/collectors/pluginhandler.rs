//! # Pluginhandler
//! This module is responsible for executing plugin scripts which collect the user's
//! `custom_fields` attributes for their Devices, Interfaces and IPAddresses.
//!
//! Currently, Nazara is set to handle `bash`, `python` and `Lua` scripts.

use std::collections::{HashMap, HashSet};
use std::hash::RandomState;
use std::path::PathBuf;
use std::process::Command;
use std::{error::Error, path::Path};

use crate::collectors::collector_exceptions::CollectorError;
use serde_json::{self, Value};

/// Execute a given script.
///
/// # Parameters
///
/// * `path: Option<String>` - The Path of the script to execute relative to the CWD. (If none, plugins
///   directory will be searched.)
///
/// # Returns
///
/// * `Ok(HashMap<String, Value, RandomState>)` - Returns a HashMap if the plugin script returns valid JSON.
/// * `Error` - If the execution of the plugin fails or it does not return a valid JSON.
pub fn execute(path: Option<String>) -> Result<HashMap<String, Value, RandomState>, Box<dyn Error>> {
    println!("Attempting to execute plugin at path '{:?}'...", path);

    // Verify that the path is valid
    if let Some(path) = path.clone() {
        if !PathBuf::from(path).is_file() {
            return Err("Provided path does not lead to a valid file.".into());
        }
    }

    // Execute the script
    let output = Command::new("bash").arg(path.unwrap()).output()?;

    // Check for execution errors
    if !output.status.success() {
        return Err("Script execution failed.".into());
    }

    // Convert stdout to a string
    let stdout_str = String::from_utf8(output.stdout)?;

    // Parse the output JSON
    let json_output: Value = serde_json::from_str(&stdout_str)?;

    let valid_arch: HashSet<&str> = [
        "aarch64", "i386", "ia64", "x86_64", "ppc64", "s390x", "ppc64le",
    ]
    .iter()
    .cloned()
    .collect();

    // Convert JSON to a HashMap, filtering invalid cpu_type and arch values
    let mut output_map: HashMap<String, Value> = json_output
        .as_object()
        .ok_or("Expected JSON object format")?
        .clone()
        .into_iter()
        .collect();

    if let Some(arch) = output_map.get("arch") {
        if let Some(arch_str) = arch.as_str() {
            if !valid_arch.contains(arch_str) {
                output_map.remove("arch");
            }
        }
    }

    // Parse and adjust Max_Power_Watt, RAM_GB, and Max_Capacity_TB as integers
    if let Some(max_power) = output_map.get("max_power") {
        if let Some(max_power_str) = max_power.as_str() {
            if let Ok(max_power_int) = max_power_str.parse::<i64>() {
                output_map.insert("max_power".to_string(), Value::Number(max_power_int.into()));
            } else {
                output_map.remove("max_power"); // Remove if not a valid integer
            }
        }
    }

    if let Some(ram_gb) = output_map.get("memory") {
        if let Some(ram_gb_str) = ram_gb.as_str() {
            if let Ok(ram_gb_int) = ram_gb_str.parse::<i64>() {
                output_map.insert("memory".to_string(), Value::Number(ram_gb_int.into()));
            } else {
                output_map.remove("memory"); // Remove if not a valid integer
            }
        }
    }

    if let Some(max_capacity) = output_map.get("capacity") {
        if let Some(max_capacity_str) = max_capacity.as_str() {
            if let Ok(max_capacity_int) = max_capacity_str.parse::<i64>() {
                output_map.insert("capacity".to_string(), Value::Number(max_capacity_int.into()));
            } else {
                output_map.remove("capacity"); // Remove if not a valid integer
            }
        }
    }

    Ok(output_map)
}

/// Validate the output of the given plugin to ensure it is valid JSON.
///
/// # Parameters
///
/// * `output: &str` - The output string to validate.
///
/// # Returns
///
/// * `Ok(())` if the output is valid JSON.
/// * `Err(CollectorError::InvalidPluginOutputError)` if the output is not valid JSON.
fn validate(output: &str) -> Result<(), CollectorError> {
    serde_json::from_str::<Value>(output)
        .map(|_| ())
        .map_err(|e| CollectorError::InvalidPluginOutputError(e))
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    use super::*;

    fn create_test_script(content: &str) -> Result<PathBuf, Box<dyn Error>> {
        // Create a temporary directory path
        let mut path = std::env::temp_dir();
        // Set the name of the script
        path.push("test_script.sh");

        // Create a new file at the specified path
        let mut file = File::create(&path)?;
        // Write the provided content into the file
        writeln!(file, "{}", content)?;

        // Make the script executable (Unix/Linux)
        #[cfg(unix)]
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;

        // Return the path to the created script
        Ok(path)
    }

    #[test]
    fn test_validate_invalid_json() {
        let invalid_json = "Invalid JSON format";

        let result = validate(invalid_json);
        assert!(result.is_err());
        if let Err(CollectorError::InvalidPluginOutputError(e)) = result {
            // Convert the error to a string and check its content
            let error_message = e.to_string();
            assert!(
                error_message.contains("expected value")
                    || error_message.contains("trailing characters")
            );
        }
    }

    #[test]
    fn test_execute_invalid_json() {
        // Create a script that outputs invalid JSON
        let script_content = r#"
        #!/bin/bash
        echo "Invalid JSON"
    "#;

        let path = create_test_script(script_content).unwrap();

        let result = execute(Some(path.to_str().unwrap().to_string()));
        assert!(result.is_err());
        if let Err(e) = result {
            // Convert the error to a string and check its content
            let error_message = e.to_string();
            assert!(
                error_message.contains("expected value")
                    || error_message.contains("trailing characters")
            );
        }
    }
}
