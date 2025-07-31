//! This module is responsible for creating a default configuration file as well as validating and reading the config
//! file.
//!
//! ```toml
#![doc = include_str!("config_template.toml")]
//! ```
//!
//! It will be created at ` $HOME/.config/nazara/config.toml`.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};

use crate::NazaraError;
use crate::error::NazaraResult;

/// Configuration state set by the configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigData {
    /// Configuration parameters for the NetBox connection.
    pub netbox: NetboxConfig,
    /// Common parameters.
    pub common: CommonConfig,
    #[serde(flatten)]
    pub machine: MachineConfig,
}

/// Configuration parameters relevant for a NetBox connection.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetboxConfig {
    /// The API token required for authentication.
    pub netbox_api_token: String,
    /// The base URL of your NetBox instance. (e.g <https://netbox.yourorg.com>)
    pub netbox_uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonConfig {
    pub name: String,
    pub description: String,
    pub comments: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MachineConfig {
    #[serde(rename = "device")]
    Device(DeviceConfig),
    #[serde(rename = "vm")]
    VM(VmConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceConfig {
    pub device_type: i64,
    pub role: i64,
    pub site: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VmConfig {
    pub cluster: i64,
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
pub fn set_up_configuration(uri: Option<&str>, token: Option<&str>) -> NazaraResult<ConfigData> {
    let mut conf_data;

    println!("Checking for existing configuration file...");

    if file_exists(&get_config_path(true)) {
        println!("Configuration file already exists. Validating...");
        // TODO Rewrite validation logic to properly condition here
        ConfigData::validate_config_file()?;
        println!("Configuration file valid. Loading defaults...");
        conf_data = ConfigData::read_config_file()?;

        if let Some(x) = uri {
            conf_data.netbox.netbox_uri = x.to_owned();
        }

        if let Some(x) = token {
            conf_data.netbox.netbox_api_token = x.to_owned();
        }

        return Ok(conf_data);
    }

    println!("No config file found. Creating default...");

    ConfigData::initialize_config_file(uri, token)?;
    println!("Default configuration file created successfully.");

    if uri.is_none() || token.is_none() {
        return Err(NazaraError::MissingConfigOptionError(String::from(
            "netbox_uri, netbox_token",
        )));
    }

    conf_data = ConfigData::read_config_file()?;

    // FIXME: This is a good place for if-let chains, but some older compilers don't support that.
    if let (Some(u), Some(t)) = (uri, token) {
        conf_data.netbox.netbox_uri = u.to_owned();
        conf_data.netbox.netbox_api_token = t.to_owned();
    }

    println!("Configuration loaded.");
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
    let home_dir = std::env::var("HOME").expect("No $HOME variable found!");
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
    fn initialize_config_file(uri: Option<&str>, token: Option<&str>) -> NazaraResult<()> {
        let file = include_str!("config_template.toml");
        let mut contents = file.to_owned();

        // Replace placeholders with actual values if exist.
        if let Some(uri) = uri {
            contents = contents.replace("{NETBOX_URI}", uri);
        }
        if let Some(token) = token {
            contents = contents.replace("{NETBOX_TOKEN}", token);
        }

        // Path to the output file
        let config_path = get_config_path(false);
        std::fs::create_dir_all(&config_path).map_err(NazaraError::FileOpError)?;
        let mut output_file =
            File::create(config_path.join("config.toml")).map_err(NazaraError::FileOpError)?;

        output_file
            .write_all(contents.as_bytes())
            .map_err(|e| NazaraError::FileOpError(e))?;

        Ok(())
    }

    /// Looks for a config file at the standard location and check if it is valid.
    /// If it is not or does not exists, an error is returned.
    fn validate_config_file() -> NazaraResult<()> {
        // TODO improve this
        let mut file = File::open(get_config_path(true)).map_err(NazaraError::FileOpError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| NazaraError::FileOpError(e))?;

        let config_data: ConfigData =
            toml::from_str(&contents).map_err(NazaraError::DeserializationError)?;

        if config_data.netbox.netbox_uri.is_empty() {
            return Err(NazaraError::MissingConfigOptionError(String::from(
                "netbox_url",
            )));
        }

        if config_data.netbox.netbox_api_token.is_empty() {
            return Err(NazaraError::MissingConfigOptionError(String::from(
                "netbox_api_token",
            )));
        }

        Ok(())
    }

    /// Opens and reads the config file and writes the set parameters into a
    /// [`ConfigData`] object, which is then returned.
    fn read_config_file() -> NazaraResult<ConfigData> {
        let mut file = File::open(get_config_path(true))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        toml::from_str(&contents).map_err(|x| x.into())
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
