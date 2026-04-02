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
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path::PathBuf};

use super::util::*;
use crate::error::*;

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
    pub name: Option<String>,
    pub description: String,
    pub comments: String,
    pub status: String,
    #[serde(default)]
    pub primary_ip4: Option<String>,
    #[serde(default)]
    pub primary_ip6: Option<String>,
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

/// View config file.
///
/// # Returns
///
/// Either `Ok(())` or `NazaraError` depending on operation outcome.
pub fn view_config_file() -> NazaraResult<()> {
    let config_path = get_config_path(true);

    if !Path::new(&config_path).exists() {
        warn!(
            "No config file found at '{}' use 'nazara write-config' to write a new one.",
            config_path.display()
        );
        return Ok(());
    }

    let contents = fs::read_to_string(&config_path).map_err(NazaraError::FileOpError)?;

    // Parse as TOML for formatting
    let parsed: toml::Value =
        toml::from_str(&contents).map_err(NazaraError::DeserializationError)?;

    let pretty = toml::to_string_pretty(&parsed).map_err(NazaraError::SerializationError)?;

    println!("\n=== Current Nazara Configuration ===\n");
    println!("{}", pretty);
    println!("==================================\n");

    Ok(())
}

/// Write the configuration file when the `write-config` command is used.
/// **Overrides current config if it already exists.** Use the `--uri` and
/// `--token` CLI flags to override them temporarily.
///
/// # Parameters
///
/// * `uri: &str` - The URI of the NetBox instance.
/// * `token: &str` - The authentication token for NetBox.
/// * `name: Option<&String>` - The name of the machine.
/// * `description: Option<&String>` - A description of the machine. (optional)
/// * `comments: Option<&String>` - A comment for the entry. (optional)
/// * `status: Option<&String>` - Status of the machine. (optional; default: 'active')
/// * `device_type: Option<&i64>` - ID of the device type.
/// * `role: Option<&i64>` - ID of the machine's role
/// * `site: Option<&i64>` - ID of the site. (physical devices only)
/// * `cluster: Option<&i64>` - ID of the cluster. (VMs only)
/// * `json: Option<&String>` - JSON representation of desired configuration values. (Optional; exclusive with all other parameters)
///
/// # Returns
///
/// Either `Ok(())` or `NazaraError` depending on operation outcome.
pub fn write_config_file(
    uri: &Option<String>,
    token: &Option<String>,
    name: &Option<String>,
    description: &Option<String>,
    comments: &Option<String>,
    status: &Option<String>,
    primary_ip4: &Option<String>,
    primary_ip6: &Option<String>,
    device_type: &Option<i64>,
    role: &Option<i64>,
    site: &Option<i64>,
    cluster: &Option<i64>,
    force: &bool,
    json: &Option<String>,
) -> NazaraResult<()> {
    let config_path = get_config_path(true);
    let config_dir = get_config_path(false);

    fs::create_dir_all(&config_dir).map_err(NazaraError::FileOpError)?;

    if let Some(json_str) = json {
        return write_config_from_json(&config_path, &json_str);
    }

    if file_exists(&config_path) && !force {
        update_existing_config(
            &config_path,
            uri,
            token,
            name,
            description,
            comments,
            status,
            primary_ip4,
            primary_ip6,
            device_type,
            role,
            site,
            cluster,
        )
    } else {
        create_new_config(
            &config_path,
            uri,
            token,
            name,
            description,
            comments,
            status,
            device_type,
            role,
            site,
            cluster,
        )
    }
}

/// Check presence and validity of config file.
pub fn check_config_file() -> NazaraResult<()> {
    let config_path = get_config_path(true);
    if !file_exists(&config_path) {
        return Err(NazaraError::Other(
            "[Error] Configuration file does not exist!".into(),
        ));
    }
    status!("Checking integrity of config file...");
    ConfigData::validate_config_file(ValidationMode::Soft)?;
    success!("Configuration file valid!");
    Ok(())
}

/// Create new config file with given parameters.
///
/// # Parameters
/// * `config_path: &Path` - The path to the config directory.
/// * `uri: &str` - The URI of the NetBox instance.
/// * `token: &str` - The authentication token for NetBox.
/// * `description: Option<String>` - Optional description of the machine.
/// * `comments: Option<String>` - Optional comments for the entry.
/// * `status: Option<String>` - Status of the device or VM.
/// * `device_tpye: Option<String>` - The status of the machine. (default: 'active')
/// * `role: Option<i64>` - The ID of the device role. (physical devices only)
/// * `site: Option<i64>` - The ID of the site where the device is located. (physical devices only)
/// * `cluster: Option<i64>` - The ID of the VM cluster. (VMs only)
///
/// # Returns
///
/// `Ok(())` or `NazaraError` depending on operation outcome.
fn create_new_config(
    config_path: &std::path::Path,
    uri: &Option<String>,
    token: &Option<String>,
    name: &Option<String>,
    description: &Option<String>,
    comments: &Option<String>,
    status: &Option<String>,
    device_type: &Option<i64>,
    role: &Option<i64>,
    site: &Option<i64>,
    cluster: &Option<i64>,
) -> NazaraResult<()> {
    let mut contents = include_str!("config_template.toml").to_string();

    if let Some(v) = token {
        contents = contents.replace(
            "netbox_api_token = \"\"",
            &format!("netbox_api_token = \"{}\"", v),
        );
    }
    if let Some(v) = uri {
        contents = contents.replace("netbox_uri = \"\"", &format!("netbox_uri = \"{}\"", v));
    }
    if let Some(v) = name {
        contents = contents.replace("name = \"\"", &format!("name = \"{}\"", v));
    }
    if let Some(v) = description {
        contents = contents.replace("description = \"\"", &format!("description = \"{}\"", v));
    }
    if let Some(v) = comments {
        contents = contents.replace(
            "comments = \"Automatically registered by Nazara.\"",
            &format!("comments = \"{}\"", v),
        );
    }
    if let Some(v) = status {
        contents = contents.replace("status = \"active\"", &format!("status = \"{}\"", v));
    }

    // Machine-specific section
    if device_type.is_some() || role.is_some() || site.is_some() {
        contents.push_str(&format!(
            "\n[device]\ndevice_type = {}\nrole = {}\nsite = {}\n",
            device_type.unwrap_or(0),
            role.unwrap_or(0),
            site.unwrap_or(0)
        ));
    } else if let Some(c) = cluster {
        contents.push_str(&format!("\n[vm]\ncluster = {}\n", c));
    }

    // Write final file
    let mut file = File::create(config_path).map_err(NazaraError::FileOpError)?;
    file.write_all(contents.as_bytes())
        .map_err(NazaraError::FileOpError)?;

    status!("Checking integrity of the file...");
    ConfigData::validate_config_file(ValidationMode::Soft)?;
    Ok(())
}

/// Update existing TOML file in place, replacing only provided keys.
///
/// # Parameters
/// * `config_path: &Path` - The path to the config directory.
/// * `uri: &str` - The URI of the NetBox instance.
/// * `token: &str` - The authentication token for NetBox.
/// * `description: Option<String>` - Optional description of the machine.
/// * `comments: Option<String>` - Optional comments for the entry.
/// * `status: Option<String>` - Status of the device or VM.
/// * `device_tpye: Option<String>` - The status of the machine. (default: 'active')
/// * `role: Option<i64>` - The ID of the device role. (physical devices only)
/// * `site: Option<i64>` - The ID of the site where the device is located. (physical devices only)
/// * `cluster: Option<i64>` - The ID of the VM cluster. (VMs only)
///
/// # Returns
///
/// `Ok(())` or `NazaraError` depending on operation outcome.
fn update_existing_config(
    config_path: &std::path::Path,
    uri: &Option<String>,
    token: &Option<String>,
    name: &Option<String>,
    description: &Option<String>,
    comments: &Option<String>,
    status: &Option<String>,
    primary_ip4: &Option<String>,
    primary_ip6: &Option<String>,
    device_type: &Option<i64>,
    role: &Option<i64>,
    site: &Option<i64>,
    cluster: &Option<i64>,
) -> NazaraResult<()> {
    let mut contents = fs::read_to_string(config_path).map_err(NazaraError::FileOpError)?;

    if let Some(v) = token {
        contents = replace_key(contents, "netbox", "netbox_api_token", &v);
    }
    if let Some(v) = uri {
        contents = replace_key(contents, "netbox", "netbox_uri", &v);
    }
    if let Some(v) = name {
        contents = replace_key(contents, "common", "name", &v);
    }
    if let Some(v) = description {
        contents = replace_key(contents, "common", "description", &v);
    }
    if let Some(v) = comments {
        contents = replace_key(contents, "common", "comments", &v);
    }
    if let Some(v) = status {
        contents = replace_key(contents, "common", "status", &v);
    }
    if let Some(v) = primary_ip4 {
        contents = replace_key(contents, "common", "primary_ip4", v);
    }
    if let Some(v) = primary_ip6 {
        contents = replace_key(contents, "common", "primary_ip6", v);
    }

    // Machine-specific
    if device_type.is_some() || role.is_some() || site.is_some() {
        contents = uncomment_section(contents, "device");
        if !contents.contains("[device]") {
            contents.push_str("\n[device]\n");
        }
        contents = uncomment_key(contents, "device", "device_type");
        contents = uncomment_key(contents, "device", "role");
        contents = uncomment_key(contents, "device", "site");
        if let Some(v) = device_type {
            contents = replace_key(contents, "device", "device_type", &v.to_string());
        }
        if let Some(v) = role {
            contents = replace_key(contents, "device", "role", &v.to_string());
        }
        if let Some(v) = site {
            contents = replace_key(contents, "device", "site", &v.to_string());
        }
        contents = comment_out_key(contents, "vm", "cluster");
        contents = comment_out_section(contents, "vm")
    } else if let Some(c) = cluster {
        contents = uncomment_section(contents, "vm");
        if !contents.contains("[vm]") {
            contents.push_str("\n[vm]\n");
        }
        contents = uncomment_key(contents, "vm", "cluster");
        contents = replace_key(contents, "vm", "cluster", &c.to_string());
        contents = comment_out_key(contents, "device", "device_type");
        contents = comment_out_key(contents, "device", "role");
        contents = comment_out_key(contents, "device", "site");
        contents = comment_out_section(contents, "device")
    }

    fs::write(config_path, contents).map_err(NazaraError::FileOpError)?;
    status!("Checking integrity of the file...");
    ConfigData::validate_config_file(ValidationMode::Soft)?;
    success!(
        "Updated existing configuration at {} (preserved comments)",
        config_path.display()
    );
    Ok(())
}

/// Write config file from JSON blob.
///
/// # Parameters
///
/// * `config_path: &Path` - The path of the config directory.
/// * `json_str: &str` - The pure JSON blob from command line.
///
/// # Returns
///
/// `Ok(())` or `NazaraError` depending on operation outcome.
fn write_config_from_json(config_path: &std::path::Path, json_str: &str) -> NazaraResult<()> {
    let parsed: Value = serde_json::from_str(json_str).map_err(NazaraError::JsonParse)?;

    let nb_uri = parsed
        .get("netbox")
        .and_then(|key| key.get("netbox_uri"))
        .and_then(|value| value.as_str())
        .unwrap_or_default();

    let nb_token = parsed
        .get("netbox")
        .and_then(|key| key.get("netbox_api_token"))
        .and_then(|value| value.as_str())
        .unwrap_or_default();

    let mut contents = include_str!("config_template.toml").to_string();
    contents = contents.replace("netbox_uri = \"\"", &format!("netbox_uri = \"{}\"", nb_uri));
    contents = contents.replace(
        "netbox_api_token = \"\"",
        &format!("netbox_api_token = \"{}\"", nb_token),
    );

    if let Some(common) = parsed.get("common") {
        if let Some(name) = common.get("name").and_then(|v| v.as_str()) {
            contents = contents.replace("name = \"\"", &format!("name = \"{}\"", name));
        }
        if let Some(desc) = common.get("description").and_then(|v| v.as_str()) {
            contents =
                contents.replace("description = \"\"", &format!("description = \"{}\"", desc));
        }
        if let Some(comment) = common.get("comments").and_then(|v| v.as_str()) {
            contents = contents.replace(
                "comments = \"Automatically registered by Nazara.\"",
                &format!("comments = \"{}\"", comment),
            );
        }
        if let Some(status) = common.get("status").and_then(|v| v.as_str()) {
            contents = contents.replace("status = \"active\"", &format!("status = \"{}\"", status));
        }
        if let Some(ip4) = common.get("primary_ip4").and_then(|v| v.as_str()) {
            contents =
                contents.replace("primary_ip4 = \"\"", &format!("primary_ip4 = \"{}\"", ip4));
        }
        if let Some(ip6) = common.get("primary_ip6").and_then(|v| v.as_str()) {
            contents =
                contents.replace("primary_ip6 = \"\"", &format!("primary_ip6 = \"{}\"", ip6));
        }
    }

    if let Some(device) = parsed.get("device") {
        let device_type = device
            .get("device_type")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let role = device.get("role").and_then(|v| v.as_i64()).unwrap_or(0);
        let site = device.get("site").and_then(|v| v.as_i64()).unwrap_or(0);
        contents.push_str(&format!(
            "\n[device]\ndevice_type = {}\nrole = {}\nsite = {}\n",
            device_type, role, site
        ));
    } else if let Some(vm) = parsed.get("vm") {
        let cluster = vm.get("cluster").and_then(|v| v.as_i64()).unwrap_or(0);
        contents.push_str(&format!("\n[vm]\ncluster = {}\n", cluster));
    }

    fs::write(config_path, contents).map_err(NazaraError::FileOpError)?;
    success!(
        "Configuration written (JSON mode) to '{}'",
        config_path.display()
    );
    Ok(())
}

/// This function reads the configuration file located at `$HOME/.config/nazara/config.toml`.
/// If no file can be found, a warning is displayed to the user and a default config file is written.
/// If command line arguments are given, the parameters read from the file will be overwritten.
///
/// # Parameters
/// - `uri`: The URI of the NetBox instance
/// - `token`: The API tokent to be used
/// - `name`: The name of the machine to register
pub fn set_up_configuration(uri: Option<&str>, token: Option<&str>) -> NazaraResult<ConfigData> {
    let mut conf_data;

    status!("Checking for existing configuration file...");

    if file_exists(&get_config_path(true)) {
        info!("Configuration file already exists. Validating...");
        ConfigData::validate_config_file(ValidationMode::Strict)?;
        success!("Configuration file valid. Loading defaults...");
        conf_data = ConfigData::read_config_file()?;

        if let Some(x) = uri {
            conf_data.netbox.netbox_uri = x.to_owned();
        }

        if let Some(x) = token {
            conf_data.netbox.netbox_api_token = x.to_owned();
        }

        return Ok(conf_data);
    }

    info!("No config file found. Creating default...");

    ConfigData::initialize_config_file(uri, token)?;
    success!("Default configuration file created successfully.");

    if uri.is_none() || token.is_none() {
        return Err(NazaraError::MissingConfigOptionError(String::from(
            "netbox_uri, netbox_token",
        )));
    }

    conf_data = ConfigData::read_config_file()?;

    if let (Some(u), Some(t)) = (uri, token) {
        conf_data.netbox.netbox_uri = u.to_owned();
        conf_data.netbox.netbox_api_token = t.to_owned();
    }

    success!("Configuration loaded.");
    Ok(conf_data)
}

/// Checks if a config file exists at a given path.
/// Returns true if the file exists.
///
/// # Parameters
/// - `path`: The filepath to check.
///
/// # Returns
/// True if the file exists.
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
/// This function panics if no `$HOME` variable can be found.
fn get_config_path(with_file: bool) -> PathBuf {
    let home_dir = std::env::var("HOME").expect("No $HOME variable found!");
    if with_file {
        return Path::new(&home_dir).join(".config/nazara/config.toml");
    }
    Path::new(&home_dir).join(".config/nazara/")
}

/// Not every process requires involving config validation
/// requires Nazara to abort. Instead we decide how strictly
/// to validate and warn/fail depending on context.
#[derive(Debug, Clone, Copy)]
pub enum ValidationMode {
    Soft,
    Strict,
}

impl ConfigData {
    /// Initializes a new default configuration file if none exists.
    ///
    /// # Parameters
    /// - `uri`: The URI of the NetBox instance
    /// - `token`: The API tokent to be used
    /// - `name`: The name of the machine to register
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
    fn validate_config_file(mode: ValidationMode) -> NazaraResult<()> {
        let mut file = File::open(get_config_path(true)).map_err(NazaraError::FileOpError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| NazaraError::FileOpError(e))?;

        // checks if the type, role, site, and cluster fields exist and warns/errors depending on the answer
        let config_unwrapped: Value = toml::from_str(&contents).unwrap();
        let config_objects: [&str; 2] = ["device", "vm"]; //config entries to check for
        let config_objects_device: [&str; 3] = ["device_type", "role", "site"]; //entries under "device" to check for
        let mut config_check_results: [bool; 2] = [false; 2];
        let mut config_error = false;
        let mut config_zero = false;
        for i in 0..2 {
            if config_unwrapped.get(config_objects[i]).is_some() {
                config_check_results[i] = true;
            }
        }

        // throws a warning/errror if both a device and a vm config exist or both dont exist.
        if config_check_results[0] && config_check_results[1] {
            failure!("Both a VM and Device Config exist.");
            config_error = true;
        } else if !config_check_results[0] && !config_check_results[1] {
            failure!("Neither Device Config nor VM Config exists");
            config_error = true;
        }

        // checks in more detail if the device configs are all correct
        if config_check_results[0] {
            for i in 0..3 {
                if config_unwrapped[config_objects[0]]
                    .get(config_objects_device[i])
                    .is_none()
                {
                    let string = format!(
                        "Device field exists but '{}' is empty",
                        config_objects_device[i]
                    );
                    failure!("{}", string);
                    config_error = true;
                } else if config_unwrapped[config_objects[0]]
                    .get(config_objects_device[i])
                    .unwrap()
                    == 0
                {
                    let string = format!(
                        "Device field exists but '{}' is 0",
                        config_objects_device[i]
                    );
                    warn!("{}", string);
                    config_zero = true;
                }
            }
        }
        // checks in more detail if the vm configs are all correct
        if config_check_results[1] {
            if config_unwrapped[config_objects[1]].get("cluster").is_none() {
                failure!("VM field exists but 'cluster' is empty");
                config_error = true;
            } else if config_unwrapped[config_objects[1]].get("cluster").unwrap() == 0 {
                warn!("VM field exists but 'cluster' is 0");
                config_zero = true;
            }
        }

        //errors out if something is very wrong with the config, warns if a change needs to be made, otherwise continues
        if config_error {
            return Err(NazaraError::Other("Incorrect Config options".to_owned()));
        } else if config_zero {
            warn!("Device/VM config is valid, but values set to 0 must be changed")
        } else {
            success!("Device/VM config is valid")
        }

        // this is the part that aborts if device/vm config is incorrect
        let config_data: ConfigData =
            toml::from_str(&contents).map_err(NazaraError::DeserializationError)?;

        // checks the uri and api token, creating a string of the missing configs for later errors
        let mut error_string: String = String::from("");
        if config_data.netbox.netbox_uri.is_empty() {
            warn!("Missing required config option: 'netbox_uri'");
            config_error = true;
            error_string.push_str("netbox_uri ");
        }
        if config_data.netbox.netbox_api_token.is_empty() {
            warn!("Missing required config option: 'netbox_api_token'");
            config_error = true;
            error_string.push_str("netbox_api_token");
        }

        if config_zero {
            match mode {
                ValidationMode::Soft => {
                    warn!("Config file contains invalid Entries");
                }
                ValidationMode::Strict => {
                    failure!("Config file contains invalid Entries");
                    return Err(NazaraError::Other(
                        "Config file contains invalid Entries".to_owned(),
                    ));
                }
            }
        }

        // prints a warning in soft mode and aborts in strict mode
        if config_error {
            match mode {
                ValidationMode::Soft => {
                    warn!("Config is missing required Netbox config options");
                }
                ValidationMode::Strict => {
                    failure!("Config is missing required Netbox config options");
                    return Err(NazaraError::MissingConfigOptionError(String::from(
                        "netbox_api_token",
                    )));
                }
            }
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
