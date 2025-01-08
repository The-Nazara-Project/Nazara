//! # Nazara
//!
//! Nazara is an experimental Rust program that automates the collection of system information for
//! NetBox.
//! It enables the automatic creation of new machines in NetBox or population of information fields
//! for existing ones.
//!
//! > Note: Nazara is currently in an alpha state. Bugs are bound to happen. If you encounter any,
//! please [report them](https://github.com/The-Nazara-Project/Nazara/issues)
//! >
//! > Furthermore, *Nazara currently does not fully support custom fields for any NetBox object*.
//! Though this is the next item on our agenda.
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
//! ./target/release/Nazara --api-url <API_URL> --api-token <API_TOKEN>
//! ```
//!
//! ## Configuring via `~/.nazara/config.toml`file.
//!
//! Nazara's configuration must be located in the user's home directory at `~/.nazara/config.toml`.
//!
//! ```toml
//! [netbox]
//! netbox_api_token = "$API_TOKEN"
//! netbox_uri = "$API_URI"
//! ```
//!
//! Aside from the NetBox system parameters, configuration via the `config.toml` also allows you to add certain
//! custom fields to your system information that cannot be automatically selected. A great example would be the
//! `System Location` entry. To specify that, simply add the parameter under the `[system]` block in your configuration file.
//!
//! ```toml
//! # A default configuration file looks like this:
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
//! # Optional data of your device
//! # This section may be empty
//! [[system.optional]]
//!
//! # tenant_group = 0 # The ID of the department this device belongs to.
//! # tenant = 0 # ID of the team or individual this device blongs to.
//! # location = 0 # ID of the location of the device.
//! # rack = 0 # ID of the Rack this device sits in.
//! # position = 0 # Position of the device within the Rack.
//! platform = "x86_64" # Name of the paltform of this device.
//! # These will be parsed into a single HashMap. You must provide
//! # the correct field labels as there is no way for Nazara to know.
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
//! *Please note that this section is still a work in progress and all information is subject to change.*
//!
//! ## Configuring custom fields using user plugins
//!
//! Users are able to fill `custom_fields` parameters in their NetBox objects using custom bash scripts.
//! These scripts should be placed inside the `~/.nazara/scripts/` directory.
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
    dmi_collector::{self, DmiInformation},
    network_collector::{self, NetworkInformation},
    pluginhandler::execute,
};
use configuration::config_parser::set_up_configuration;
use publisher::publisher::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::{collections::HashMap, process};
use thanix_client::util::ThanixClient;

use crate::collectors::pluginhandler;

/// The Machine struct
///
/// This struct represents your machine.
/// It holds all information collected and allows for sharing this
/// information between Nazara's modules.
///
/// It is used in places where it is necessary to have access to various
/// pieces of collected information from a single source of truth.
/// It will also be translated into the proper API type by the translator.
pub struct Machine {
    pub name: Option<String>,
    pub dmi_information: DmiInformation,
    pub network_information: Vec<NetworkInformation>,
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
/// These arguments override the ones defined in the `.nbs-config.toml`.
///
/// # Members
///
/// * `uri: String` - The URI to your Netbox instance.
/// * `token: String` - The authentication token for the netbox URI.
/// * `name: String` - The name of the device
/// * `location: String` - The location of the device
/// * `device_role: String` - The type of device (server, router, etc.)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// URI to your Netbox instance
    #[arg(short, long)]
    uri: Option<String>,

    /// Your API authentication token
    #[arg(short, long)]
    token: Option<String>,

    /// The name of the device
    #[arg(short, long)]
    name: Option<String>,

    /// The Path to a plugin script you want to run
    #[arg(short, long)]
    plugin: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let ascii_art = r#"
    ███╗   ██╗ █████╗ ███████╗ █████╗ ██████╗  █████╗
    ████╗  ██║██╔══██╗╚══███╔╝██╔══██╗██╔══██╗██╔══██╗
    ██╔██╗ ██║███████║  ███╔╝ ███████║██████╔╝███████║
    ██║╚██╗██║██╔══██║ ███╔╝  ██╔══██║██╔══██╗██╔══██║
    ██║ ╚████║██║  ██║███████╗██║  ██║██║  ██║██║  ██║
    ╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝
"#;

    // Welcome Message.
    println!(
        "{} \n(c) Tiara Hock aka ByteOtter. (github.com/ByteOtter)\n
         Licensed under the terms of the GPL-v3.0 License.\n\
         Check github.com/The-Nazara-Project/Nazara/LICENSE for more info.\n",
        ascii_art
    );

    let config = match set_up_configuration(args.uri, args.token, args.name.clone()) {
        Ok(conf) => conf,
        Err(err) => {
            err.abort(None);
        }
    };

    let client: ThanixClient = ThanixClient {
        base_url: config.get_netbox_uri().to_string(),
        authentication_token: config.get_api_token().to_string(),
        client: Client::new(),
    };

    match probe(&client) {
        Ok(()) => {}
        Err(err) => err.abort(None),
    };

    let dmi_information: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();

    let network_information: Vec<NetworkInformation> =
        network_collector::construct_network_information().unwrap();

    let machine: Machine = Machine {
        name: args.name,
        dmi_information,
        network_information,
        custom_information: match execute(args.plugin) {
            Ok(info) => Some(info),
            Err(e) => panic!("{}", e.to_string()),
        },
    };

    // Passing a name in any way is mandatory for a virtual machine
    if machine.dmi_information.system_information.is_virtual && machine.name.is_none() {
        eprintln!("[FATAL] No name has been provided for this virtual machine! Providing a name as search parameter is mandatory for virtual machines.");
        process::exit(1)
    }

    // Register the machine or VM with NetBox
    match register_machine(&client, machine, config) {
        Ok(_) => {
            println!("\x1b[32mAll done, have a nice day!\x1b[0m");
            process::exit(0);
        }
        Err(e) => e.abort(None),
    };
}
