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
//! site_id = 0 # The ID of the site this device is located at.
//! description = ""
//! comments = "Automatically registered using Nazara."
//! device_type = 0
//! role = 0
//! # Name of the network interface to set. (e.g eth0, etc)
//! # If not set, the first active interface will be selected.
//! primary_network_interface = ""
//! face = "" # Direction this device may face (e.g front or rear)
//! status = "active" # Status of the device. 'active' by default.
//! airflow = "front-to-rear" # Direction of airflow.
//!
//! # Optional data of your device
//! # This section may be empty
//! [[system.optional]]
//! # tenant_group = 0 # The ID of the department this device belongs to.
//! # tenant = 0 # ID of the team or individual this device blongs to.
//! # location = 0 # ID of the location of the device.
//! # rack = 0 # ID of the Rack this device sits in.
//! # position = 0 # Position of the device within the Rack.
//! platform = "x86_64" # Name of the paltform of this device.
//!
//! # These will be parsed into a single HashMap. You must provide
//! # the correct field labels as there is no way for Nazara to know.
//!
//! # These values are purely exemplary.
//! [system.custom_fields]
//!
//! # Network Interfaces Configuration (optional)
//! #[[nwi]]
//! #name = "" # Required. Must match interface that exists on the machine.
//! #enabled = true
//! #rtype = "type1"
//! #parent = 1
//! #bridge = 1
//! #lag = 1
//! #mtu = 1500
//! #duplex = "full"
//! #wwn = "wwn12345"
//! #mgmt_only = false
//! #description = "Automatically created by Nazara."
//! #mode = ""
//! #rf_role = ""
//! #rf_channel = ""
//! #poe_role = ""
//! #poe_channel = ""
//! #rf_channel_frequency = 2400.0
//! #rf_channel_width = 20.0
//! #tx_power = 20
//! #untagged_vlans = [10, 20]
//! #tagged_vlans = [30, 40]
//! #mark_connected = true
//! #wireless_lans = [50, 60]
//! #vrf = 1
//! # Custom fields specific for this interface
//! #[nwi.custom_fields]
//! # ...
//! ```
//!
//! It will be created at ` $HOME/.nazara/.config.toml`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::hash::RandomState;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};

use super::config_exceptions::ConfigError;

/// Configuration State set by the configuration file.
///
/// # Members
/// * netbox: `NetBoxConfig` - Configuration parameters for the NetBox connection.
/// * system: `SystemConfig` - Parameters abouth the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub netbox: NetboxConfig,
    pub system: SystemConfig,
    pub nwi: Option<Vec<NwiConfig>>,
}

/// Configuration Parameters relevant for NetBox connection.
///
/// # Members
/// * netbox_api_token: `String` - The API token required for authentication.
/// * netbox_uri: `String` - The base URL of your NetBox instance. (e.g https://netbox.yourorg.com)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetboxConfig {
    pub netbox_api_token: String,
    pub netbox_uri: String,
}

/// Additional information about the system.
///
/// # Members
///
/// * name: `String` - Name of the device. *Required for virtual machines! Must be unique!*
/// * site_id: `i64` - ID of the site the device is located in.
/// * description: `String` - Short description of the device.
/// * comments: `String` - Comment field.
/// * device_type: `i64` - ID of the device type.
/// * device_role: `i64` - ID of the device role.
/// * face: `String` - Tag of the orientation of the device.
/// * status: `String` - Status of the device. (Default: `active`)
/// * airflow: `String` - Airflow orientation of the device.
/// * primary_network_interface: `Option<String>` - Name of the network interface you want to set as
/// primary.
/// * custom_fields: `Option<HashMap<String, Value, RandomState>>` - Unsorted, unfiltered list of
/// information that will be handed to NetBox as-is.
/// * platform_name: `Option<String>` - Name of the processor architecture used as a fallback if collection by `uname`
/// fails. *Must be present in your NetBox instance!*
/// * tenant_group: `Option<i64>` - ID of the tenant group this device belongs to. (e.g: department)
/// * tenant: `Option<i64>` - ID of tenant this device belongs to. (e.g: team or individual)
/// * rack: `Option<i64>` - ID of the rack this device is located in.
/// * position: `Option<i64>` - Position of the device within a rack if any.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemConfig {
    pub name: String,
    pub site_id: Option<i64>,
    pub site_name: Option<String>,
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
    pub tenant_group_name: Option<String>,
    pub tenant: Option<i64>,
    pub tenant_name: Option<i64>,
    pub location: Option<i64>,
    pub rack: Option<i64>,
    pub position: Option<i64>,
}

/// Information about the system's interface.
///
/// # Members
///
/// * name: `String` - The name of the interface.
/// * id: `Option<i64>` - The ID of the interface, if it already exists. Mutually exclusive with
/// name.
/// * vdcs: `Option<Vec<i64>>`
/// * module: `Option<i64>` - The module assigned to this interface.
/// * label: `Option<String>` - The phyiscal label of this device if any.
/// * r#type: `String` - The type of the interface (e.g. "bridge")
/// * enabled: `bool` - Whether this device is enabled or not. Default: `True`.
/// * parent: `Option<i64>` - ID of the parent interface if applicable.
/// * bridge: `Option<i64>` - ID of the bridge device for this interface if applicable.
/// * lag: `Option<i64>`
/// * mtu: `Option<u32>`
/// * duplex: `Option<String>`
/// * wwn: `Option<String>`
/// * mgmt_only: `bool` - Whether this interface may only be used for management. Default: `False`.
/// * description: `Option<String>` - Optional description of the device.
/// * mode: `Option<String>` - The mode this interface operates in.
/// * rf_role: `Option<String>`
/// * rf_channel: `Option<String>`
/// * poe_mode: `Option<String>`
/// * poe_type: `Option<String>`
/// * rf_channel_frequency: `Option<f64>`
/// * rf_channel_width: `Option<f64>`
/// * poe_mode: `Option<String>` - The PoE mode of the interface.
/// * poe_type: `Option<String>`
/// * rf_channel_frequency: `Option<f64>`
/// * rf_channel_width: `Option<f64>`
/// * tx_power: `Option<u8>`
/// * untagged_vlans: `Option<Vec<i64>>` - List of IDs of untagged VLANs assigned to this
/// interface.
/// * tagged_vlans: `Option<Vec<i64>>` - List of IDs of tagged VLANs assigned to this interface.
/// * mark_connected: `bool` - Whether this interface is connected. Default: `True`.
/// * wireless_lans: `Option<Vec<i64>>`
/// * vrf: `Option<i64>`
/// * custom_fields: `Option<Hashmap<String, Value, RandomState>>` - Any Custom fields you wish to
/// add in form a of a Key-Value list.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NwiConfig {
    pub name: Option<String>,
    pub id: Option<i64>,
    pub vdcs: Option<Vec<i64>>,
    pub module: Option<i64>,
    pub label: Option<String>,
    #[serde(rename = "rtype")]
    pub r#type: Option<String>, // I hate this field name, but that's what the openAPI schema said.
    pub enabled: Option<bool>,
    pub parent: Option<i64>,
    pub bridge: Option<i64>,
    pub lag: Option<i64>,
    pub mtu: Option<u32>,
    pub duplex: Option<String>,
    pub wwn: Option<String>,
    pub mgmt_only: Option<bool>,
    pub description: Option<String>,
    pub mode: Option<String>,
    pub rf_role: Option<String>,
    pub rf_channel: Option<String>,
    pub poe_mode: Option<String>,
    pub poe_type: Option<String>,
    pub rf_channel_frequency: Option<f64>,
    pub rf_channel_width: Option<f64>,
    pub tx_power: Option<u8>,
    pub untagged_vlans: Option<Vec<i64>>,
    pub tagged_vlans: Option<Vec<i64>>,
    pub mark_connected: Option<bool>,
    pub wireless_lans: Option<Vec<i64>>,
    pub vrf: Option<i64>,
    pub custom_fields: Option<HashMap<String, Value, RandomState>>,
}

/// Set up configuration
///
/// This function reads the configuration file located at `$HOME/.nazara/config.toml`. If no file can be found, a warning is
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
) -> Result<ConfigData, ConfigError> {
    let mut conf_data: ConfigData;

    println!("Checking for existing configuration file...");

    if file_exists(&get_config_dir()) {
        println!("Configuration file already exists. Validating...");
        // TODO Rewrite validation logic to properly condition here
        match ConfigData::validate_config_file() {
            Ok(_) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Configuration file \x1b[32mvalid\x1b[0m. Loading defaults..."
                );
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
        Err(e) => e.abort(None),
    }

    if uri.is_none() || token.is_none() {
        let err = ConfigError::MissingConfigOptionError(String::from("netbox_uri, netbox_token"));
        err.abort(None)
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
/// * path: `&str` - The path to the config file.
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
/// * config_file_path: `PathBuf` - The directory the config file is located ($HOME/.nazara/config.toml)
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

    let config_file_path: PathBuf = Path::new(&home_dir).join(".nazara/config.toml");

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
    /// If it is not able to create a new config file at `$HOME/.nazara-config.toml` or if it cannot write the defaults
    /// to the file, the function panics as this is the main method of configuring the program.
    fn initialize_config_file(
        uri: &Option<String>,
        token: &Option<String>,
        name: &Option<String>,
    ) -> Result<(), ConfigError> {
        let template_path: &Path = Path::new("src/configuration/config_template.toml");
        let mut file: File = match File::open(template_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(ConfigError::FileOpError(err));
            }
        };
        let mut contents: String = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => {
                return Err(ConfigError::FileOpError(err));
            }
        };

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
    fn validate_config_file() -> Result<(), ConfigError> {
        // TODO improve this
        let mut file = match File::open(get_config_dir()) {
            Ok(f) => f,
            Err(e) => return Err(ConfigError::FileOpError(e)),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => return Err(ConfigError::FileOpError(e)),
        }

        let config_data: ConfigData = match toml::from_str(&contents) {
            Ok(data) => data,
            Err(e) => return Err(ConfigError::DeserializationError(e)),
        };

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

    /// Opens and reads the config file and writes the set parameters into a [`ConfigData`](struct.ConfigData) Object
    /// which is then returned.
    ///
    /// # Returns
    ///
    /// * config: `ConfigData` - A [`ConfigData`] object.
    ///
    /// # Aborts
    ///
    /// This function pay terminate the process if it cannot read the cofnig file.
    fn read_config_file() -> ConfigData {
        let mut file = match File::open(get_config_dir()) {
            Ok(file) => file,
            Err(err) => {
                let exc: ConfigError = ConfigError::FileOpError(err);
                exc.abort(None);
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(u) => u,
            Err(err) => {
                let exc = ConfigError::FileOpError(err);
                exc.abort(None);
            }
        };

        let config_data: ConfigData = match toml::from_str(&contents) {
            Ok(t) => t,
            Err(err) => {
                let exc = ConfigError::DeserializationError(err);
                exc.abort(None);
            }
        };

        config_data
    }

    /// Return NetBox URL. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * system_location: `&str` - The location of the system to be created/updated as read from the config file.
    pub fn get_netbox_uri(&self) -> &str {
        &self.netbox.netbox_uri
    }

    /// Return API auth token. Necessary for payload generation.
    ///
    /// # Returns
    ///
    /// * system_location: `String` - The location of the system to be created/updated as read from the config file.
    pub fn get_api_token(&self) -> &str {
        &self.netbox.netbox_api_token
    }
}
