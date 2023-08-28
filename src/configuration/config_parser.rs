//! This module is responsible for creating a default configuration file as well as validating and reading the config
//! file.
//!
//! A default configuration file looks like this:
//!
//! ```toml
//! [netbox]
//! netbox_uri = ""
//! netbox_api_token = ""
//! ```
//!
//! It will be created at `/etc/opt/.nbs-config.toml`.

use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};
use toml::Value;

use super::config_exceptions::{self, *};

#[derive(Debug, Deserialize)]
pub struct ConfigData {
    netbox_uri: String,
    netbox_auth_token: String,
}

/// Set up configuration
pub fn set_up_configuration() -> Result<ConfigData, String> {
    let conf_data: ConfigData;

    println!("Checking for existing configuration file...");
    if file_exists(&get_config_dir()) {
        println!("Configuration file already exists. Validating...");
        // TODO Rewrite validation logic to properly condition here
        match ConfigData::validate_config_file() {
            Ok(_) => {
                println!("Configuration file valid. Loading defaults...");
                conf_data = ConfigData::read_config_file();
                return Ok(conf_data);
            }
            Err(err) => return Err(err),
        }
    } else {
        println!("No config file found. Creating default...")
    }

    match ConfigData::initialize_config_file() {
        Ok(_) => {}
        Err(_) => {
            panic!("FATAL: An error occurred while initializing the config!")
        }
    }
    conf_data = ConfigData::read_config_file();

    println!("\x1b[32mConfiguration loaded.\x1b[0m");
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
/// * `config_file_path: PathBuf` - The directory the config file is located (~/.nbs-config.toml)
fn get_config_dir() -> PathBuf {
    let home_dir: String = match std::env::var("HOME") {
        Ok(val) => val,
        Err(err) => {
            panic!("FATAL: No $XDG_CONFIG_HOME variable found! ({})", err)
        }
    };

    let config_file_path: PathBuf = Path::new(&home_dir).join(".nbs-config.toml");

    return config_file_path;
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
    /// If it is not able to create a new config file at `etc/opt/.nbs-config.toml` or if it cannot write the defaults
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
            netbox_config_table.insert(
                "netbox_api_token_path".to_string(),
                Value::String("".to_string()),
            );
            netbox_config_table
        };

        // Insert the netbox section as value under the header "netbox"
        config.insert("netbox".to_string(), Value::Table(netbox_section));

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
                    message: format!("FATAL: Unable to create config file! ({})", err),
                };
                exc.panic();
            }
        };

        // Write default contents to file
        match file.write_all(toml_string.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                let exc: UnableToCreateConfigError = config_exceptions::UnableToCreateConfigError {
                    message: format!("FATAL: Unable to write defaults to config file! ({})", err),
                };
                exc.panic();
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
        let mut file: File = match File::open(get_config_dir()) {
            Ok(file) => file,
            Err(_) => {
                let exc: NoConfigFileError = config_exceptions::NoConfigFileError {
                    message: format!("\x1b[31mFATAL:\x1b[0m No configuration file found!"),
                };
                exc.panic();
            }
        };

        let mut contents: String = String::new();

        if let Err(err) = file.read_to_string(&mut contents) {
            let exc: UnableToReadConfigError = config_exceptions::UnableToReadConfigError {
                message: format!(
                    "\x1b[31mFATAL:\x1b[0m Unable to read config file! ({})",
                    err
                ),
            };
            exc.panic()
        }

        // TODO: This raises an error!
        let config_content: ConfigData = match toml::from_str(&contents) {
            Ok(config) => config,
            Err(err) => {
                let exc: InvalidConfigFileError = config_exceptions::InvalidConfigFileError {
                    message: format!("\x1b[31mFATAL:\x1b[0m Invalid config file syntax! Make sure the configuration file has valid TOML syntax.{}", err),
                };
                exc.panic()
            }
        };

        if config_content.netbox_uri.is_empty() {
            println!("Warning: Parameter netbox_uri is empty!");
            return Err(
                "Error: Config parameter 'netbox_uri' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_content.netbox_auth_token.is_empty() {
            println!("Warning: Parameter netbox_api_token is empty!");
            return Err(
                "Error: Config parameter 'netbox_api_token' is empty! This parameter is mandatory."
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
        let mut file_content: File = match File::open(get_config_dir()) {
            Ok(file) => file,
            Err(err) => {
                let exc = config_exceptions::NoConfigFileError {
                    message: format!(
                        "FATAL: An error occured while reading the config file! ({})",
                        err
                    ),
                };
                exc.panic()
            }
        };

        let mut contents: String = String::new();

        if let Err(err) = file_content.read_to_string(&mut contents) {
            let exc: UnableToReadConfigError = config_exceptions::UnableToReadConfigError {
                message: format!(
                    "\x1b[31mFATAL:\x1b[0m Unable to read config file! ({})",
                    err
                ),
            };
            exc.panic()
        }

        let config_content: ConfigData = match toml::from_str(&contents) {
            Ok(config) => config,
            Err(err) => {
                let exc: InvalidConfigFileError = config_exceptions::InvalidConfigFileError {
                    message: format!("\x1b[31mFATAL:\x1b[0m Invalid config file syntax! Make sure the configuration file has valid TOML syntax. {}", err),
                };
                exc.panic()
            }
        };

        return config_content;
    }
}
