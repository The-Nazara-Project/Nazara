//! This module is responsible for creating a default configuration file as well as validating and reading the config
//! file.
//!
//! ```toml
#![doc = include_str!("config_template.toml")]
//! ```
//!
//! It will be created at ` $HOME/.config/nazara/config.toml`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::hash::RandomState;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};

use super::error::ConfigError;
/// Configuration state set by the configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigData {
    /// Configuration parameters for the NetBox connection.
    pub netbox: NetboxConfig,
    /// Parameters about the system.
    pub system: SystemConfig,
    pub nwi: Option<Vec<NwiConfig>>,
}

/// Configuration parameters relevant for a NetBox connection.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetboxConfig {
    /// The API token required for authentication.
    pub netbox_api_token: String,
    /// The base URL of your NetBox instance. (e.g <https://netbox.yourorg.com>)
    pub netbox_uri: String,
}

/// Additional information about the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemConfig {
    /// Name of the device. *Required for virtual machines! Must be unique!*
    pub name: String,
    /// ID of the site the device is located in.
    pub site_id: Option<i64>,
    pub site_name: Option<String>,
    /// Short description of the device.
    pub description: String,
    /// Comment field.
    pub comments: String,
    /// ID of the device type.
    pub device_type: i64,
    /// ID of the device role.
    pub device_role: i64,
    /// Tag of the orientation of the device.
    pub face: String,
    /// Status of the device. (Default: `active`)
    pub status: String,
    /// Airflow orientation of the device.
    pub airflow: String,
    /// Name of the network interface you want to set as primary.
    pub primary_network_interface: Option<String>,
    /// Unsorted, unfiltered list of information that will be handed to NetBox as-is.
    pub custom_fields: Option<HashMap<String, Value, RandomState>>,
    /// ID of the tenant group this device belongs to (e.g: department).
    pub tenant_group: Option<i64>,
    pub tenant_group_name: Option<String>,
    /// ID of tenant this device belongs to (e.g: team or individual).
    pub tenant: Option<i64>,
    pub tenant_name: Option<i64>,
    pub location: Option<i64>,
    /// ID of the rack this device is located in.
    pub rack: Option<i64>,
    /// Position of the device within a rack if any.
    pub position: Option<i64>,
}

/// Information about the system's interface.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NwiConfig {
    /// The name of the interface.
    pub name: Option<String>,
    /// The ID of the interface, if it already exists. Mutually exclusive with name.
    pub id: Option<i64>,
    pub vdcs: Option<Vec<i64>>,
    /// The module assigned to this interface.
    pub module: Option<i64>,
    /// The phyiscal label of this device if any.
    pub label: Option<String>,
    #[serde(rename = "rtype")]
    /// The type of the interface (e.g. "bridge")
    pub r#type: Option<String>, // I hate this field name, but that's what the openAPI schema said.
    /// Whether this device is enabled or not. Default: `True`.
    pub enabled: Option<bool>,
    /// ID of the parent interface if applicable.
    pub parent: Option<i64>,
    /// ID of the bridge device for this interface if applicable.
    pub bridge: Option<i64>,
    pub lag: Option<i64>,
    pub mtu: Option<u32>,
    pub duplex: Option<String>,
    pub wwn: Option<String>,
    /// Whether this interface may only be used for management. Default: `False`.
    pub mgmt_only: Option<bool>,
    /// Optional description of the device.
    pub description: Option<String>,
    /// The mode this interface operates in.
    pub mode: Option<String>,
    pub rf_role: Option<String>,
    pub rf_channel: Option<String>,
    /// The PoE mode of the interface.
    pub poe_mode: Option<String>,
    pub poe_type: Option<String>,
    pub rf_channel_frequency: Option<f64>,
    pub rf_channel_width: Option<f64>,
    pub tx_power: Option<u8>,
    /// List of IDs of untagged VLANs assigned to this interface.
    pub untagged_vlans: Option<Vec<i64>>,
    /// List of IDs of tagged VLANs assigned to this interface.
    pub tagged_vlans: Option<Vec<i64>>,
    /// Whether this interface is connected. Default: `True`.
    pub mark_connected: Option<bool>,
    pub wireless_lans: Option<Vec<i64>>,
    pub vrf: Option<i64>,
    /// Any Custom fields you wish to add in form a of a Key-Value list.
    pub custom_fields: Option<HashMap<String, Value, RandomState>>,
}

/// This function reads the configuration file located at `$HOME/.config/nazara/config.toml`.
/// If no file can be found, a warning is displayed to the user and a default config file is written.
/// If command line arguments are given, the parameters read from the file will be overwritten.
///
/// # Parameters
///
/// * `uri: Option<&str>` - The URI of the NetBox instance
/// * `token: Option<&str>` - The API tokent to be used
/// * `name: Option<&str>` - The name of the machine to register
///
/// # Panics
///
/// The function panics under these conditions:
/// - If the initialization of the config file raises an error.
/// - When using a default (empty) configuration file and not providing all required CLI arguments.
/// - If the configuration file cannot be read.
pub fn set_up_configuration(
    uri: Option<&str>,
    token: Option<&str>,
    name: Option<&str>,
) -> Result<ConfigData, ConfigError> {
    let mut conf_data;

    println!("Checking for existing configuration file...");

    if file_exists(&get_config_path(true)) {
        println!("Configuration file already exists. Validating...");
        // TODO Rewrite validation logic to properly condition here
        match ConfigData::validate_config_file() {
            Ok(_) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Configuration file \x1b[32mvalid\x1b[0m. Loading defaults..."
                );
                conf_data = ConfigData::read_config_file();

                if let Some(x) = uri {
                    conf_data.netbox.netbox_uri = x.to_owned();
                }

                if let Some(x) = token {
                    conf_data.netbox.netbox_api_token = x.to_owned();
                }

                if let Some(x) = name {
                    conf_data.system.name = x.to_owned();
                }

                return Ok(conf_data);
            }
            Err(err) => return Err(err),
        }
    }

    println!("\x1b[36m[info]\x1b[0m No config file found. Creating default...");

    ConfigData::initialize_config_file(uri, token, name)?;
    println!("\x1b[32m[success]\x1b[0m Default configuration file created successfully.");

    if uri.is_none() || token.is_none() {
        panic!(
            "{}",
            ConfigError::MissingConfigOptionError(String::from("netbox_uri, netbox_token"))
        );
    }

    conf_data = ConfigData::read_config_file();

    if uri.is_some() && token.is_some() && token.is_some() {
        conf_data.netbox.netbox_uri = uri.unwrap().to_owned();
        conf_data.netbox.netbox_api_token = token.unwrap().to_owned();
        conf_data.system.name = name.unwrap().to_owned();
    }

    println!("\x1b[32m[success]\x1b[0m Configuration loaded.\x1b[0m");
    Ok(conf_data)
}

/// Checks if a config file exists at a given path.
/// Returns true if the file exists.
///
/// # Parameters
///
/// * `path: &Path` - The filepath to check.
///
/// # Returns
///
/// True/False depending on whether the file exists.
fn file_exists(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_file()
    } else {
        false
    }
}

/// Constructs a path to the config directory.
/// This function will fetch the path to the home directory from the `$HOME` environment variable.
///
/// # Panics
///
/// This function panics if no `$HOME` variable can be found.
fn get_config_path(with_file: bool) -> PathBuf {
    let home_dir = std::env::var("HOME").expect("\x1b[31m[FATAL]\x1b[0m No $HOME variable found!");
    if with_file {
        return Path::new(&home_dir).join(".config/nazara/config.toml");
    }
    Path::new(&home_dir).join(".config/nazara/")
}

impl ConfigData {
    /// Initializes a new default configuration file if none exists.
    ///
    /// # Parameters
    ///
    /// * `uri: Option<&str>` - The URI of the NetBox instance
    /// * `token: Option<&str>` - The API tokent to be used
    /// * `name: Option<&str>` - The name of the machine to register
    ///
    /// # Panics
    ///
    /// If it is not able to create a new config file at `$HOME/.config/nazara/config.toml` or if it cannot write the defaults
    /// to the file, the function panics as this is the main method of configuring the program.
    fn initialize_config_file(
        uri: Option<&str>,
        token: Option<&str>,
        name: Option<&str>,
    ) -> Result<(), ConfigError> {
        let file = include_str!("config_template.toml");
        let mut contents = file.to_owned();

        // Replace placeholders with actual values if exist.
        if let Some(uri) = uri {
            contents = contents.replace("{NETBOX_URI}", uri);
        }
        if let Some(token) = token {
            contents = contents.replace("{NETBOX_TOKEN}", token);
        }
        if let Some(name) = name {
            contents = contents.replace("{SYSTEM_NAME}", name);
        }

        // Path to the output file
        let config_path = get_config_path(false);
        std::fs::create_dir_all(&config_path).map_err(ConfigError::FileOpError)?;
        let mut output_file =
            File::create(config_path.join("config.toml")).map_err(ConfigError::FileOpError)?;

        output_file
            .write_all(contents.as_bytes())
            .map_err(|e| ConfigError::FileOpError(e))?;

        Ok(())
    }

    /// Looks for a config file at the standard location and check if it is valid.
    /// If it is not or does not exists, an error is returned.
    fn validate_config_file() -> Result<(), ConfigError> {
        // TODO improve this
        let mut file =
            File::open(get_config_path(true)).map_err(|e| ConfigError::FileOpError(e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| ConfigError::FileOpError(e))?;

        let config_data: ConfigData =
            toml::from_str(&contents).map_err(|e| ConfigError::DeserializationError(e))?;

        if config_data.netbox.netbox_uri.is_empty() {
            return Err(ConfigError::MissingConfigOptionError(String::from(
                "netbox_url",
            )));
        }

        if config_data.netbox.netbox_api_token.is_empty() {
            return Err(ConfigError::MissingConfigOptionError(String::from(
                "netbox_api_token",
            )));
        }

        if config_data.system.name.is_empty() {
            return Err(ConfigError::MissingConfigOptionError(String::from(
                "system name",
            )));
        }

        // Optional NWI Section
        if let Some(nwi_list) = &config_data.nwi {
            for nwi in nwi_list {
                if nwi.r#type.is_none() {
                    return Err(ConfigError::MissingConfigOptionError(String::from(
                        "r#type",
                    )));
                }
            }
        } else {
            println!(
                "\x1b[36m[info]\x1b[0m No network interfaces defined in the 'nwi' section. This is allowed."
            );
        }

        Ok(())
    }

    /// Opens and reads the config file and writes the set parameters into a
    /// [`ConfigData`] object, which is then returned.
    ///
    /// # Panics
    ///
    /// This function will panic if it cannot read the config file.
    fn read_config_file() -> ConfigData {
        let mut file = File::open(get_config_path(true)).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        toml::from_str(&contents).unwrap()
    }

    /// Returns NetBox URL. Necessary for payload generation.
    pub fn get_netbox_uri(&self) -> &str {
        &self.netbox.netbox_uri
    }

    /// Returns API auth token. Necessary for payload generation.
    pub fn get_api_token(&self) -> &str {
        &self.netbox.netbox_api_token
    }
}
