//! # Pluginhandler
//! This module is responsible for executing plugin scripts which collect the user's
//! `custom_fields` attributes for their Devices, Interfaces and IPAddresses.
//!
//! Currently, Nazara is set to handle `bash`, `python` and `Lua` scripts.

use std::collections::HashMap;
use std::hash::RandomState;
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
pub fn execute(
    path: Option<String>,
) -> Result<HashMap<String, Value, RandomState>, Box<dyn Error>> {
    let script_path = match path.as_deref() {
        Some(p) => Path::new(p),
        None => return Err("No path provided.".into()),
    };

    println!(
        "Attempting to execute plugin at path '{}'...",
        script_path.display()
    );

    if !script_path.is_file() {
        return Err("Provided path does not lead to a file.".into());
    }

    let output = Command::new("bash").arg(script_path).output()?;

    if !output.status.success() {
        let err = CollectorError::PluginExecutionError(
            "Either you have a syntax error in your code or the file does not exist.".to_string(),
        );
        return Err(err.into());
    }

    let stdout_str = String::from_utf8(output.stdout)?;

    validate(&stdout_str)?; // Validate JSON format

    let json_output: HashMap<String, Value> = serde_json::from_str(&stdout_str)?;
    Ok(json_output)
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
