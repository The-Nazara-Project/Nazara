//! # Nazara
//!
//! Nazara is an experimental Rust program that automates the collection of system information for
//! NetBox.
//! It enables the automatic creation of new machines in NetBox or population of information fields
//! for existing ones.
//!
//! > Note: Nazara is currently in an alpha state. Bugs are bound to happen. If you encounter any,
//!> please [report them](https://github.com/The-Nazara-Project/Nazara/issues)
//! >
//! > Furthermore, *Nazara currently does not fully support custom fields for any NetBox object*.
//!> Though this is the next item on our agenda.
//!
//! # Installation
//!
//! ## Building from source
//!
//! To use Nazara, you will need to have the Rust programming language and `cargo` installed. If you do not have them
//! installed already, you can follow the instructions provided in the [official Rust documentation](https://www.rust-lang.org/tools/install).
//!
//! *Please note that this program only works on Linux systems.*
//!
//! Once you have everything installed, you can clone this repository and build the program by running the following commands:
//!
//! ```bash
//! git clone https://github.com/The-Nazara-Project/Nazara.git
//! cd Nazara
//! cargo build --release
//! ```
//!
//! This will create an executable file in the `target/release` directory.
//!
//! ### Installation via `crates.io`
//!
//! Nazara is published on `crates.io`. If your operating system permits cargo to install packages globally, simply run `cargo install nazara` to install it.
//!
//! # Usage
//!
//! To use Nazara, you will need to configure the URL of your NetBox API and provide an API token to the program by
//! configuring all of these parameters inside the [configuration file](#configuring-via-nazaraconfigtomlfile).
//!
//! After that, simply run
//!
//! ```bash
//!  nazara register
//! ```
//!
//! to register a new machine, or run
//!
//! ```bash
//!  nazara update $MACHINE_ID
//! ```
//!
//! to update an existing one.
//!
//!
//! in your terminal. Nazara will automatically collect all required system information and decide whether to create a new device, or update an existing entry.
//!
//! # Configuration
//!
//! Nazara supports two ways of providing configuration parameters: CLI arguments and a configuration file.
//!
//! Nazara requires two parameters from you:
//!
//! - `API_URL`: The URL of your NetBox API
//! - `API_TOKEN`: The authentication token for the NetBox API
//!
//! ## Configuring via CLI
//!
//! Here is an example for passing these parameters on using the CLI:
//!
//! ```bash
//! sudo ./target/release/Nazara --uri <API_URL> --token <API_TOKEN>
//! ```
//!
//! ## Configuring via `$HOME/.config/nazara/config.toml`file.
//!
//! Nazara's configuration must be located in the root user's home directory at `$HOME/.config/nazara/config.toml`.
//!
//! Aside from the NetBox system parameters, configuration via the `config.toml` also allows you to add certain
//! custom fields to your system information that cannot be automatically selected. A great example would be the
//! `System Location` entry. To specify that, simply add the parameter under the `[system]` block in your configuration file.
//!
//! A default configuration file looks like this:
//!
//! ```toml
#![doc = include_str!("configuration/config_template.toml")]
//! ```
//!
//! ### The `config` commands
//!
//! Nazara provides you with several commands to manage your configuration files:
//!
//! - `write-config`: Write a new config file or overwrite an existing one.
//! - `check-config`: Validate if your config is still valid.
//! - `view-config`: Print config to console.
//!
//! The `write-config` allows you to change individual parameters, or perform a bulk update by passing a `JSON` structure
//! via CLI. **These options are exclusive. Passing both is disallowed.**
//!
//! For further information on how to configure Nazara, run `nazara --help` or visit [our documentation](https://the-nazara-project.github.io/Nazara/users/configuration.html).
//!
//! *Please note that this section is still a work in progress and all information is subject to change.*
//!
//! ## Configuring custom fields using user plugins
//!
//! Users are able to fill `custom_fields` parameters in their NetBox objects using custom bash scripts.
//! These scripts should be placed inside the `$HOME/.config/nazara/scripts/` directory.
//!
//! These scripts can collect the desired information and output *a valid JSON representation* to `stdout`.
//! Nazara then reads this output, validates it, and attempts to parse it to a `HashMap` of values.
//!
//! If everything works out, this will populate all of your custom fields no matter what fields you specified, as long as your script
//! is correct.
//!
//! > Warning:
//! >
//! > Users must make sure that the output of their scripts matches the name of their desired custom fields they specified
//! > in NetBox.
//! >
//! > Currently, **we only support text fields** as all the other field types would require smart parsing on our end.
//! > We are currently investigating on how to achieve this.
//!
//!# Contributing
//!
//! If you would like to contribute to Nazara, feel free to check the [contributing guide](./CONTRIBUTING.md) for
//! information on our workflow and check the issues section for any open issue.
//!
//! # License
//!
//! Nazara is released under the terms of the [GPL-v3.0](./LICENSE).

mod collectors;
pub mod configuration;
pub mod error;
pub mod publisher;

use clap::{Parser, Subcommand};
use collectors::{
    dmi::{self, DmiInformation},
    network::{self, NetworkInformation},
    plugin::execute,
};
use configuration::parser::{
    check_config_file, set_up_configuration, view_config_file, write_config_file,
};
use publisher::{
    auto_register_or_update_machine, register_machine, test_connection, update_machine,
};
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;
use thanix_client::util::ThanixClient;

pub use error::NazaraError;

#[cfg(target_os = "linux")]
use crate::error::NazaraResult;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Register a new machine.
    // Any future arguments for this command into its block.
    Register,
    /// Update a given machine by ID.
    Update {
        /// The ID of the machine in NetBox.
        #[arg(long)]
        id: i64,
    },
    /// Attempt to detect whether an update or new registration is necessary. (DEPRECATED, old default behaviour)
    Auto,
    /// Write new config file or overwrite existing one with new values. Pass JSON for bulk changes.
    WriteConfig {
        /// The URI of your NetBox instance. Required if not using '--json'.
        #[arg(short, long, required_unless_present = "json", conflicts_with = "json")]
        uri: Option<String>,

        /// Your NetBox authentication token. Required if not using '--json'.
        #[arg(short, long, required_unless_present = "json", conflicts_with = "json")]
        token: Option<String>,

        /// The machine's name. (Optional; default: hostname)
        #[arg(short, long, conflicts_with = "json")]
        name: Option<String>,

        /// A description of the machine. (Optional)
        #[arg(short, long, conflicts_with = "json")]
        description: Option<String>,

        /// A comment for the entry. (Optional; default: 'Automatically registered by Nazara')
        #[arg(short, long, conflicts_with = "json")]
        comments: Option<String>,

        /// The status of the machine. (Optional; defaults: 'active')
        #[arg(short, long, conflicts_with = "json")]
        status: Option<String>,

        /// Device type ID. (if this is a physical device)
        #[arg(long, conflicts_with = "json")]
        device_type: Option<i64>,

        /// Device role ID.
        #[arg(long, conflicts_with = "json")]
        role: Option<i64>,

        /// Site ID. (for physical devices)
        #[arg(long, conflicts_with = "json")]
        site: Option<i64>,

        /// Cluster ID. (for VMs)
        #[arg(long, conflicts_with = "json")]
        cluster_id: Option<i64>,

        /// JSON of your configuration parameters. (Optional; exclusive with other options.)
        #[arg(long, conflicts_with_all = &[
            "uri", "token", "name", "descirption", "comments",
            "status", "device_type", "role", "site", "cluster_id"
        ])]
        json: Option<String>,
    },
    /// Validate configuration file.
    CheckConfig,
    ViewConfig,
}

/// The arguments that Nazara expects to get via the cli.
///
/// Arguments can be passed like this:
///
/// ```
/// nazara --uri <NETBOX_URI> --token <NETBOX_TOKEN> register
/// ```
///
/// These arguments override the ones defined in the `$HOME/.config/nazara/config.toml`.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Only prints collected information to stdout.
    #[arg(short, long)]
    dry_run: bool,

    /// Temporarily overwrite the url to your NetBox instance.
    #[arg(short, long)]
    uri: Option<String>,

    /// Temporarily use a different authentication token for NetBox.
    #[arg(short, long)]
    token: Option<String>,

    /// The Path to a plugin script you want to run.
    #[arg(short, long)]
    plugin: Option<String>,

    /// Subcommands.
    #[command(subcommand)]
    command: Commands,
}

/// This struct represents your machine.
/// It holds all information collected and allows for sharing this
/// information between Nazara's modules.
///
/// It is used in places where it is necessary to have access to various
/// pieces of collected information from a single source of truth.
/// It will also be translated into the proper API type by the translator.
#[derive(Debug)]
pub struct Machine {
    /// The name of the system to register. Read from the CLI.
    pub name: Option<String>,
    /// Information collected by `dmidecode`.
    pub dmi_information: DmiInformation,
    /// List of network interfaces.
    pub network_information: Vec<NetworkInformation>,
    /// Custom fields read from config file or via plugins.
    pub custom_information: Option<HashMap<String, Value>>,
}

fn warn_auto_deprecated() {
    let msg = "
    \x1b[33m[WARNING] +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ [WARNING]\x1b[0m
    \x1b[33m[WARNING] Running Nazara in 'Auto' mode is deprecated. Please use 'register' or 'update' subcommands instead. [WARNING]\x1b[0m
    \x1b[33m[WARNING] +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ [WARNING]\x1b[0m
";
    eprintln!("{}", msg);
}

/// Collect machine information.
///
/// Stubs the collector to start putting together all information about the machine.
///
/// # Parameters
///
/// * `plugin: Option<String>` - The path to a plugin. (optional)
///
/// # Returns
///
/// Either a `Machine` instance or a `NazaraError`, if collection failed.
fn start_collection(plugin: Option<String>) -> NazaraResult<Machine> {
    Ok(Machine {
        name: None,
        dmi_information: dmi::construct_dmi_information()?,
        network_information: network::construct_network_information()?,
        custom_information: Some(execute(plugin)?),
    })
}

#[cfg(target_os = "linux")]
fn main() -> NazaraResult<()> {
    let args: Args = Args::parse();

    const ASCII_ART: &str = r#"
    ███╗   ██╗ █████╗ ███████╗ █████╗ ██████╗  █████╗
    ████╗  ██║██╔══██╗╚══███╔╝██╔══██╗██╔══██╗██╔══██╗
    ██╔██╗ ██║███████║  ███╔╝ ███████║██████╔╝███████║
    ██║╚██╗██║██╔══██║ ███╔╝  ██╔══██║██╔══██╗██╔══██║
    ██║ ╚████║██║  ██║███████╗██║  ██║██║  ██║██║  ██║
    ╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝
    (c) Tiara Hock aka ByteOtter. (github.com/ByteOtter)

    Licensed under the terms of the GPL-v3.0 License.
    Check github.com/The-Nazara-Project/Nazara/LICENSE for more info.
"#;

    // Welcome Message.
    println!("{ASCII_ART}");

    match &args.command {
        Commands::WriteConfig {
            uri,
            token,
            name,
            description,
            comments,
            status,
            device_type,
            site,
            role,
            cluster_id,
            json,
        } => {
            println!("Writing configuration file...");
            if json.is_some() {
                write_config_file(
                    "", // ignored in JSON mode
                    "", // ignored in JSON mode
                    name,
                    description,
                    comments,
                    status,
                    device_type,
                    role,
                    site,
                    cluster_id,
                    json,
                )?;
            } else {
                // Enforce required params for manual mode
                let uri = uri
                    .as_deref()
                    .ok_or_else(|| NazaraError::Other("Missing required argument: --uri".into()))?;
                let token = token.as_deref().ok_or_else(|| {
                    NazaraError::Other("Missing required argument: --token".into())
                })?;

                write_config_file(
                    uri,
                    token,
                    name,
                    description,
                    comments,
                    status,
                    device_type,
                    role,
                    site,
                    cluster_id,
                    &None,
                )?;
            }
            println!("Configuration written successfully.");
            return Ok(());
        }
        Commands::CheckConfig => {
            check_config_file()?;
            return Ok(());
        }
        Commands::ViewConfig => {
            view_config_file()?;
            return Ok(());
        }
        _ => {} // Other commands handled below.
    }

    let machine = start_collection(args.plugin.clone())?;

    // Passing a name in any way is mandatory for a virtual machine.
    if machine.dmi_information.system_information.is_virtual && machine.name.is_none() {
        return Err(NazaraError::Other(
            "No name has been provided for this virtual machine! Providing a name as search parameter is mandatory for virtual machines.".into(),
        ));
    }

    // If we only want to do a dry run, we only have to print the collected information.
    if args.dry_run {
        println!("Dry run results:");
        dbg!(&machine);
        return Ok(());
    }

    // TODO: Do we still need this?
    let config = set_up_configuration(args.uri.as_deref(), args.token.as_deref())?;

    let client = ThanixClient {
        base_url: config.get_netbox_uri().to_string(),
        authentication_token: config.get_api_token().to_string(),
        client: Client::new(),
    };

    println!("Testing connection...");
    test_connection(&client)?;

    // Register the machine or VM with NetBox
    // TODO: Match here for given subcommand
    match &args.command {
        Commands::Register => register_machine(&client, machine, config)?,
        Commands::Update { id } => update_machine(&client, machine, config, id.to_owned())?,
        Commands::Auto {} => {
            warn_auto_deprecated();
            auto_register_or_update_machine(&client, machine, config)?;
        }
        _ => {
            // Already covered further up
        }
    }
    println!("All done, have a nice day!");

    Ok(())
}
