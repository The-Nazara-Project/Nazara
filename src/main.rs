mod collectors;
pub mod configuration;
mod publisher;

use clap::Parser;
use collectors::{dmi_collector, network_collector};
use configuration::config_parser::set_up_configuration;
use std::process;

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
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// URI to your Netbox instance
    #[arg(short, long)]
    uri: Option<String>,

    /// Your API authentication token
    #[arg(short, long)]
    token: Option<String>,

    /// The location of the machine (must be one of the locations you have set as available in your Netbox instance)
    #[arg(short, long)]
    location: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    // println!("Uri: {}\nToken: {}", args.uri.clone().unwrap(), args.token.clone().unwrap());

    let output: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();
    println!("{:#?}", output);

    let output2 = network_collector::construct_network_information().unwrap();

    println!("{:#?}", output2);

    let config = match set_up_configuration(args.uri, args.token, args.location) {
        Ok(conf) => conf,
        Err(err) => {
            println!("{}", err);
            process::exit(1)
        }
    };

    println!("Configuration: \n{:#?}", config);
}
