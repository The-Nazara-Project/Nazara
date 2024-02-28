//! This module is responsible for creating a default configuration file as well as validating and reading the config
//! file.
//!
//! A default configuration file looks like this:
//!
//! ```toml
//! [netbox]
//! netbox_uri = ""
//! netbox_api_token = ""
//!
//! [system]
//! location = ""
//! ```
//!
//! It will be created at ` ~/.nazara-config.toml`.

use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};
use toml::Value;

use super::config_exceptions::{self, *};

#[derive(Debug, Deserialize)]
pub struct ConfigData {
    netbox_api_token: String,
    netbox_uri: String,
    name: String,
    system_location: String,
    device_role: String,
}

/// Set up configuration
///
/// This function reads the configuration file located at `~/.nazara-config.toml`. If no file can be found, a warning is
/// displayed to the user and a default config file is written.
/// If command line arguments are given, the parameters read from the file will be overwritten.
///
/// # Returns
///
/// * `Ok(ConfigData)` - A `ConfigData` object containing the netbox URI and API token.
/// * `Err` - Prints an Error if the file cannot be validated.
///
/// # Panics
///
/// The function panics under these conditions:
///
/// * If the initialization of the config file raises an error.
/// * When using a default (empty) configuration file and not providing all required CLI arguments.
/// * If the configuration file cannot be read.
pub fn set_up_configuration(
    uri: Option<String>,
    token: Option<String>,
    name: Option<String>,
    location: Option<String>,
    device_role: Option<String>,
) -> Result<ConfigData, String> {
    let mut conf_data: ConfigData;

    println!("Checking for existing configuration file...");

    if file_exists(&get_config_dir()) {
        println!("Configuration file already exists. Validating...");
        // TODO Rewrite validation logic to properly condition here
        match ConfigData::validate_config_file() {
            Ok(_) => {
                println!("\x1b[32m[success]\x1b[0m Configuration file \x1b[32mvalid\x1b[0m. Loading defaults...");
                conf_data = ConfigData::read_config_file();

                if uri.is_some() {
                    conf_data.netbox_uri = uri.unwrap();
                }

                if token.is_some() {
                    conf_data.netbox_api_token = token.unwrap();
                }

                if name.is_some() {
                    conf_data.name = name.unwrap();
                }

                if location.is_some() {
                    conf_data.system_location = location.unwrap();
                }

                if device_role.is_some() {
                    conf_data.device_role = device_role.unwrap();
                }

                return Ok(conf_data);
            }
            Err(err) => return Err(err),
        }
    }

    println!("\x1b[36m[info]\x1b[0m No config file found. Creating default...");

    match ConfigData::initialize_config_file() {
        Ok(_) => {
            println!("\x1b[32m[success]\x1b[0m Default configuration file created successfully.")
        }
        Err(_) => {
            let err = config_exceptions::UnableToCreateConfigError {
                message: "\x1b[31m[FATAL]\x1b[0m An error occurred while initializing the config!"
                    .to_string(),
            };
            err.abort(12)
        }
    }

    if uri.is_none() || token.is_none() {
        let err = config_exceptions::NoConfigError {
            message: "\x1b[31m[FATAL]\x1b[0m No configuration parameters found in CLI while using an empty config file!\nPlease enter valid configuration parameters in the configuration file or provide them via the CLI.".to_string()
        };
        err.abort(11)
    }

    conf_data = ConfigData::read_config_file();

    if uri.is_some() && token.is_some() && location.is_some() {
        conf_data.netbox_uri = uri.unwrap();
        conf_data.netbox_api_token = token.unwrap();
        conf_data.name = name.unwrap();
        conf_data.system_location = location.unwrap();
        conf_data.device_role = device_role.unwrap();
    }

    println!("\x1b[32m[success]\x1b[0m Configuration loaded.\x1b[0m");
    Ok(conf_data)
}

/// Check if config file exists at default path.
///
/// If true there a new config file will not be created.
///
/// # Arguments
///
/// * `path: &str` - The path to the config file.
///
/// # Returns
///
/// * `bool` - True or False depending on the file's existence.
fn file_exists(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_file()
    } else {
        false
    }
}

/// Construct path of config directory.
///
/// This function will pull the path to the home directory from the `$XDG_CONFIG_HOME` environment variable.
///
/// `NOTE: This operation only works on Unix systems. This will need to be rewritten for other operating systems.`
///
/// # Returns
///
/// * `config_file_path: PathBuf` - The directory the config file is located (~/.nazara-config.toml)
///
/// # Panics
///
/// This function panics if no `$XDF_CONFIG_HOME` variable can be found.
fn get_config_dir() -> PathBuf {
    let home_dir: String = match std::env::var("HOME") {
        Ok(val) => val,
        Err(err) => {
            panic!(
                "\x1b[31m[FATAL]\x1b[0m No $XDG_CONFIG_HOME variable found! ({})",
                err
            )
        }
    };

    let config_file_path: PathBuf = Path::new(&home_dir).join(".nazara-config.toml");

    config_file_path
}

impl ConfigData {
    /// Initializes a new default configuration file if none exists.
    ///
    /// # Returns
    ///
    /// An empty `Ok()` to indicate the operation's success.
    ///
    /// # Panics
    ///
    /// If it is not able to create a new config file at `~/.nazara-config.toml` or if it cannot write the defaults
    /// to the file, the function panics as this is the main method of configuring the program.
    fn initialize_config_file() -> std::io::Result<()> {
        // Create new toml table
        let mut config: toml::map::Map<String, Value> = toml::value::Table::new();

        // Create netbox section
        let netbox_section: toml::map::Map<String, Value> = {
            let mut netbox_config_table: toml::map::Map<String, Value> = toml::value::Table::new();
            netbox_config_table.insert("netbox_uri".to_string(), Value::String("".to_string()));
            netbox_config_table.insert(
                "netbox_api_token".to_string(),
                Value::String("".to_string()),
            );
            netbox_config_table
        };

        let system_section: toml::map::Map<String, Value> = {
            let mut system_config_table: toml::map::Map<String, Value> = toml::value::Table::new();
            system_config_table.insert("name".to_string(), Value::String("".to_string()));
            system_config_table
                .insert("system_location".to_string(), Value::String("".to_string()));
            system_config_table.insert("device_role".to_string(), Value::String("".to_string()));
            system_config_table
        };

        // Insert the netbox section as value under the header "netbox"
        config.insert("netbox".to_string(), Value::Table(netbox_section));

        config.insert("system".to_string(), Value::Table(system_section));

        let toml_string: String = match toml::to_string(&Value::Table(config)) {
            Ok(result) => result,
            Err(err) => {
                println!("{}", err);
                String::new()
            }
        };

        // Create a new File
        let mut file: File = match File::create(get_config_dir()) {
            Ok(file) => file,
            Err(err) => {
                let exc: UnableToCreateConfigError = config_exceptions::UnableToCreateConfigError {
                    message: format!(
                        "\x1b[31mFATAL:\x1b[0m Unable to create config file! ({})",
                        err
                    ),
                };
                exc.abort(10);
            }
        };

        // Write default contents to file
        match file.write_all(toml_string.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                let exc: UnableToCreateConfigError = config_exceptions::UnableToCreateConfigError {
                    message: format!(
                        "\x1b[31mFATAL:\x1b[0m Unable to write defaults to config file! ({})",
                        err
                    ),
                };
                exc.abort(13);
            }
        }
        Ok(())
    }

    /// Look for a config file at the standard location and check if it is valid. If it is not or does not exists, an Err
    /// is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - When the config file is valid.
    /// * `Err(String)` - When the config file is invalid.s
    ///
    /// # Panics
    ///
    /// This function will panic if:
    ///
    /// * not able to read the config file.
    /// * the config file does not have valid TOML syntax.
    fn validate_config_file() -> Result<(), String> {
        let file_contents: String = match fs::read_to_string(get_config_dir()) {
            Ok(contents) => contents,
            Err(err) => {
                let exc: UnableToReadConfigError = config_exceptions::UnableToReadConfigError {
                    message: format!("x1b[31m[FATAL]x1b[0m Unable to open config file! {}", err),
                };
                exc.abort(14)
            }
        };

        let config_contents: Value = match toml::from_str(&file_contents) {
            Ok(config) => config,
            Err(err) => {
                let exc: InvalidConfigFileError = config_exceptions::InvalidConfigFileError {
                    message: format!("\x1b[31m[FATAL]\x1b[0m Invalid config file syntax! Make sure the configuration file has valid TOML syntax. ({})", err),
                };
                exc.abort(15)
            }
        };

        let config_parameters: ConfigData = ConfigData {
            netbox_api_token: config_contents["netbox"]["netbox_uri"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            netbox_uri: config_contents["netbox"]["netbox_api_token"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            name: config_contents["system"]["name"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            system_location: config_contents["system"]["system_location"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            device_role: config_contents["system"]["device_role"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
        };

        if config_parameters.netbox_uri.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'netbox_uri' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_parameters.netbox_api_token.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'netbox_api_token' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_parameters.name.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'name' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_parameters.system_location.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'system_location' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_parameters.device_role.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'device_role' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        return Ok(());
    }

    /// Opens and reads the config file and writes the set parameters into a [`ConfigData`](struct.ConfigData) Object
    /// which is then returned.
    ///
    /// # Returns
    ///
    /// * `config: ConfigData` - A `ConfigData` object.
    fn read_config_file() -> ConfigData {
        let file_contents: String = match fs::read_to_string(get_config_dir()) {
            Ok(contents) => contents,
            Err(err) => {
                let exc: UnableToReadConfigError = config_exceptions::UnableToReadConfigError {
                    message: format!("\x1b[31m[FATAL]\x1b[0m Unable to open config file! {}", err),
                };
                exc.abort(14)
            }
        };

        let config_content: Value = match toml::from_str(&file_contents) {
            Ok(config) => config,
            Err(err) => {
                let exc: InvalidConfigFileError = config_exceptions::InvalidConfigFileError {
                    message: format!("\x1b[31m[FATAL]\x1b[0m Invalid config file syntax! Make sure the configuration file has valid TOML syntax. ({})", err),
                };
                exc.abort(15)
            }
        };

        let config_parameters: ConfigData = ConfigData {
            netbox_api_token: config_content["netbox"]["netbox_api_token"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            netbox_uri: config_content["netbox"]["netbox_uri"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            name: config_content["system"]["name"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            system_location: config_content["system"]["system_location"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
            device_role: config_content["system"]["device_role"]
                .as_str()
                .unwrap()
                .trim()
                .to_string(),
        };

        return config_parameters;
    }

    /// Return NetBox URL. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * `system_location: &str` - The location of the system to be created/updated as read from the config file.
    pub fn get_netbox_uri(&self) -> &str {
        &self.netbox_uri
    }

    /// Return API auth token. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * `system_location: String` - The location of the system to be created/updated as read from the config file.
    pub fn get_api_token(&self) -> &str {
        &self.netbox_api_token
    }

    /// Return system location. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * `system_location: String` - The location of the system to be created/updated as read from the config file.
    pub fn get_system_location(&self) -> &str {
        &self.system_location
    }
}
