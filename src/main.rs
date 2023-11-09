mod collectors;
pub mod configuration;
pub mod publisher;

use clap::Parser;
use collectors::{dmi_collector, network_collector};
use configuration::config_parser::set_up_configuration;
use publisher::publisher::*;
use std::process;

use crate::publisher::{api_client::NetBoxClient, publisher_exceptions::NetBoxApiError};

/// The arguments that netbox-sync expects to get via the cli.
///
/// Arguments can be passed like this:
///
/// ```
/// netbox-sync --uri <NETBOX_URI> --token <NETBOX_TOKEN>
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
░█▀█░█▀▀░▀█▀░█▀▄░█▀█░█░█░░░░░█▀▀░█░█░█▀█░█▀▀
░█░█░█▀▀░░█░░█▀▄░█░█░▄▀▄░▄▄▄░▀▀█░░█░░█░█░█░░
░▀░▀░▀▀▀░░▀░░▀▀░░▀▀▀░▀░▀░░░░░▀▀▀░░▀░░▀░▀░▀▀▀"#;

    // Welcome Message.
    println!(
        "{} \n(c) Christopher Hock aka ByteOtter. (github.com/ByteOtter)\n\
         Licensed under the terms of the MIT License.\n\
         Check github.com/ByteOtter/netbox-sync/LICENSE for more info.\n",
        ascii_art
    );

    let config = match set_up_configuration(
        args.uri,
        args.token,
        args.name,
        args.location,
        args.device_role,
    ) {
        Ok(conf) => conf,
        Err(err) => {
            println!("{}", err);
            process::exit(1)
        }
    };

    Publisher::probe(&config.get_netbox_uri(), &config.get_api_token());

    // println!("Configuration: \n{:#?}", config);

    // println!("Uri: {}\nToken: {}", args.uri.clone().unwrap(), args.token.clone().unwrap());

    let dmi_information: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();
    // println!("{:#?}", dmi_information);

    let network_information = network_collector::construct_network_information().unwrap();

    // println!("{:#?}", network_information);

    // let system_information: SystemData = SystemData {
    //     dmi_information,
    //     network_information,
    //     system_location: config.get_system_location().to_string(),
    // };

    // let payload: CreateMachinePayload = CreateMachinePayload { system_information };

    // match netbox_client.create_machine(&payload) {
    //     Ok(()) => println!("Machine created!"),
    //     Err(err) => eprintln!("Error: {:?}", err),
    // }
}
