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
//! name = "some_name" # Required for virtual machines!
//! site_id = 0
//! description = ""
//! comments = "Automatically registered using Nazara."
//! device_type = 0
//! role = 0
//!
//!
//! # These will be parsed a singl HashMap with no further checking.
//! # Make sure that these custom fields line up with the
//! # Custom fields of your NetBox instance.
//! [[system.custom]]
//! cpu_count = 1
//! platform = "x86_64" # Overriden by collector
//! collect_cpu_information = true
//! collect_network_information = true
//! primary_network_interface = "eth0"
//! config_template = 0 # integer of the config_template ID
//! ```
//!
//! It will be created at ` ~/.nazara-config.toml`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::hash::RandomState;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};

use super::config_exceptions::{self, *};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub netbox: NetboxConfig,
    pub system: SystemConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetboxConfig {
    pub netbox_api_token: String,
    pub netbox_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    pub name: String,
    pub site_id: i64,
    pub description: String,
    pub comments: String,
    pub device_type: i64,
    pub device_role: i64,
    pub face: String,
    pub status: String,
    pub airflow: String,
    pub primary_network_interface: Option<String>,
    pub custom_fields: Option<HashMap<String, Value, RandomState>>,
    pub platform_name: Option<String>,
    // optional System information
    pub tenant_group: Option<i64>,
    pub tenant: Option<i64>,
    pub rack: Option<i64>,
    pub position: Option<i64>,
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
                    conf_data.netbox.netbox_uri = uri.unwrap();
                }

                if token.is_some() {
                    conf_data.netbox.netbox_api_token = token.unwrap();
                }

                if name.is_some() {
                    conf_data.system.name = name.unwrap();
                }

                return Ok(conf_data);
            }
            Err(err) => return Err(err),
        }
    }

    println!("\x1b[36m[info]\x1b[0m No config file found. Creating default...");

    match ConfigData::initialize_config_file(&uri, &token, &name) {
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

    if uri.is_some() && token.is_some() && name.is_some() {
        conf_data.netbox.netbox_uri = uri.unwrap();
        conf_data.netbox.netbox_api_token = token.unwrap();
        conf_data.system.name = name.unwrap();
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
    fn initialize_config_file(
        uri: &Option<String>,
        token: &Option<String>,
        name: &Option<String>,
    ) -> std::io::Result<()> {
        let template_path: &Path = Path::new("src/configuration/config_template.toml");
        let mut file: File = match File::open(&template_path) {
            Ok(file) => file,
            Err(err) => {
                let exc = config_exceptions::UnableToReadConfigError {
                message: format!("\x1b[31m[error]\x1b[0m An Error occurred while attempting to read template file! {}", err)
            };
                exc.abort(1);
            }
        };
        let mut contents: String = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => {
                panic!("{}", err);
            }
        };

        // Replace placeholders with actual values if exist.
        if let Some(uri) = uri {
            contents = contents.replace("{NETBOX_URI}", &uri);
        }
        if let Some(token) = token {
            contents = contents.replace("{NETBOX_TOKEN}", &token);
        }
        if let Some(name) = name {
            contents = contents.replace("{SYSTEM_NAME}", &name);
        }

        // Path to the output file
        let output_path = get_config_dir();
        let mut output_file = match File::create(&output_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("{}", err)
            }
        };
        match output_file.write_all(contents.as_bytes()) {
            Ok(()) => {}
            Err(err) => {
                panic!("{}", err)
            }
        };

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
        let mut file = File::open(get_config_dir())
            .map_err(|e| format!("[error] Failed to open config file! {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("[error] Failed to read config file! {}", e))?;

        let config_data: ConfigData = toml::from_str(&contents)
            .map_err(|e| format!("[error] Failed to deserialize toml parameters! {}", e))?;

        if config_data.netbox.netbox_uri.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'netbox_uri' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_data.netbox.netbox_api_token.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'netbox_api_token' is empty! This parameter is mandatory."
                    .to_string(),
            );
        }

        if config_data.system.name.is_empty() {
            return Err(
                "\x1b[31m[error]\x1b[0m Validation Error: Config parameter 'name' is empty! This parameter is mandatory."
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
        let mut file = match File::open(get_config_dir()) {
            Ok(file) => file,
            Err(err) => {
                let exc = config_exceptions::UnableToReadConfigError {
                    message: format!(
                        "[error] An error occurred while attempting to read the config file: {}",
                        err
                    ),
                };
                exc.abort(1);
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(u) => u,
            Err(err) => {
                let exc = config_exceptions::UnableToReadConfigError {
                    message: format!("[error] Unable to read config file to buffer! {}", err),
                };
                exc.abort(1);
            }
        };

        let config_data: ConfigData = match toml::from_str(&contents) {
            Ok(t) => t,
            Err(err) => {
                let exc = config_exceptions::UnableToCreateConfigError {
                    message: format!(
                        "[error] An error occured while trying to parse the toml: {}",
                        err
                    ),
                };
                exc.abort(1);
            }
        };

        config_data
    }

    /// Return NetBox URL. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * `system_location: &str` - The location of the system to be created/updated as read from the config file.
    pub fn get_netbox_uri(&self) -> &str {
        &self.netbox.netbox_uri
    }

    /// Return API auth token. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * `system_location: String` - The location of the system to be created/updated as read from the config file.
    pub fn get_api_token(&self) -> &str {
        &self.netbox.netbox_api_token
    }
}
