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
//!  nazara
//! ```
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
//! sudo ./target/release/Nazara --api-url <API_URL> --api-token <API_TOKEN>
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
pub mod publisher;

use clap::Parser;
use collectors::{
    dmi::{self, DmiInformation},
    network::{self, NetworkInformation},
    plugin::execute,
};
use configuration::parser::set_up_configuration;
use publisher::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::{collections::HashMap, error::Error, process};
use thanix_client::util::ThanixClient;

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

/// The arguments that Nazara expects to get via the cli.
///
/// Arguments can be passed like this:
///
/// ```
/// nazara --uri <NETBOX_URI> --token <NETBOX_TOKEN>
/// ```
///
/// These arguments override the ones defined in the `$HOME/.config/nazara/config.toml`.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Only prints collected information to stdout
    #[arg(short, long)]
    dry_run: bool,

    /// URI to your NetBox instance
    #[arg(short, long)]
    uri: Option<String>,

    /// Your API authentication token
    #[arg(short, long)]
    token: Option<String>,

    /// The Path to a plugin script you want to run
    #[arg(short, long)]
    plugin: Option<String>,
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
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

    // Collect machine information.
    let machine = Machine {
        name: None,
        dmi_information: dmi::construct_dmi_information()?,
        network_information: network::construct_network_information()?,
        custom_information: match execute(args.plugin) {
            Ok(info) => Some(info),
            Err(e) => panic!("{}", e.to_string()),
        },
    };

    // Passing a name in any way is mandatory for a virtual machine.
    if machine.dmi_information.system_information.is_virtual && machine.name.is_none() {
        eprintln!(
            "[FATAL] No name has been provided for this virtual machine! Providing a name as search parameter is mandatory for virtual machines."
        );
        process::exit(1)
    }

    // If we only want to do a dry run, we only have to print the collected information.
    if args.dry_run {
        println!("Dry run results:");
        dbg!(&machine);
    } else {
        let config = set_up_configuration(args.uri.as_deref(), args.token.as_deref())?;

        let client = ThanixClient {
            base_url: config.get_netbox_uri().to_string(),
            authentication_token: config.get_api_token().to_string(),
            client: Client::new(),
        };

        println!("Testing connection...");
        test_connection(&client)?;

        // Register the machine or VM with NetBox
        register_machine(&client, machine, config)?;
        println!("\x1b[32mAll done, have a nice day!\x1b[0m");
    }

    Ok(())
}
