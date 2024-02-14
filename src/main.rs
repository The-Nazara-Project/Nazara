mod collectors;
pub mod configuration;
pub mod publisher;

use clap::Parser;
use collectors::{
    dmi_collector::{self, DmiInformation},
    network_collector::{self, NetworkInformation},
};
use configuration::config_parser::set_up_configuration;
use publisher::publisher::*;
use reqwest::blocking::Client;
use std::process;
use thanix_client::util::ThanixClient;

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

    /// The location of the machine (must be one of the locations you have set as available in your Netbox instance)
    #[arg(short, long)]
    location: Option<String>,

    /// The role of the machine (switch, server, router, etc.)
    #[arg(short, long)]
    device_role: Option<String>,
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
        "{} \n(c) Christopher Hock aka ByteOtter. (github.com/ByteOtter)\n
         Licensed under the terms of the GPL-v3.0 License.\n\
         Check github.com/The-Nazara-Project/Nazara/LICENSE for more info.\n",
        ascii_art
    );

    let config = match set_up_configuration(
        args.uri,
        args.token,
        args.name.clone(),
        args.location,
        args.device_role,
    ) {
        Ok(conf) => conf,
        Err(err) => {
            println!("{}", err);
            process::exit(1)
        }
    };

    let client: ThanixClient = ThanixClient {
        base_url: config.get_netbox_uri().to_string(),
        authentication_token: config.get_api_token().to_string(),
        client: Client::new(),
    };

    match probe(&client) {
        Ok(()) => {}
        Err(err) => println!("{}", err),
    };

    let dmi_information: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();

    let network_information = network_collector::construct_network_information().unwrap();

    let machine: Machine = Machine {
        name: args.name,
        dmi_information,
        network_information,
    };

    // Passing a name in any way is mandatory for a virtual machine
    if machine.dmi_information.system_information.is_virtual && machine.name.is_none() {
        panic!("[FATAL] No name has been provided for this virtual machine! Providing a name as search parameter is mandatory for virtual machines.")
    }

    let _ = register_machine(&client, machine);

    // println!("{:#?}", network_information);

    // let system_information: SystemData = SystemData {
    //     dmi_information,
    //     network_information,
    //     name: config.name,
    //     system_location: config.get_system_location().to_string(),
    //     device_role: config.device_role,
    // };

    // let payload: CreateMachinePayload = CreateMachinePayload { system_information };

    // match netbox_client.create_machine(&payload) {
    //     Ok(()) => println!("Machine created!"),
    //     Err(err) => eprintln!("Error: {:?}", err),
    // }
}
