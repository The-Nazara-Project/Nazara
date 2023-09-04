mod collectors;
pub mod configuration;

use clap::Parser;
use collectors::{dmi_collector, network_collector};
use configuration::config_parser::set_up_configuration;

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
    uri: String,

    /// Your API authentication token
    #[arg(short, long)]
    token: String,
}

fn main() {
    let args: Args = Args::parse();

    println!("Uri: {}\nToken: {}", args.uri, args.token);

    let output: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();
    println!("{:#?}", output);

    let output2 = network_collector::construct_network_information().unwrap();

    println!("{:#?}", output2);

    let config = match set_up_configuration(args.uri, args.token) {
        Ok(conf) => conf,
        Err(err) => {
            panic!("{}", err)
        }
    };

    println!("Configuration: \n{:#?}", config);
}
