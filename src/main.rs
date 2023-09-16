mod collectors;
pub mod configuration;
mod publisher;

use clap::Parser;
use collectors::{dmi_collector, network_collector};
use configuration::config_parser::set_up_configuration;
use netbox_api::apis::configuration::{ApiKey, Configuration};
use netbox_api::apis::dcim_api;
use netbox_api::models::writable_device_with_config_context::Face;
use netbox_api::models::WritableDeviceWithConfigContext;
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

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.base_path = "https://demo.netbox.dev".to_string();
    config.user_agent = Some("asdf".to_string());
    let key = ApiKey {
        prefix: None,
        key: "9bc449c89a195b9b9cebfa65b61de23d0912c0a0".to_string(),
    };
    config.api_key = Some(key);
    let device = WritableDeviceWithConfigContext::new(
        Some(String::from("Horst")),
        1,
        1,
        None,
        0,
        None,
        Face::Front,
        None,
    );

    dcim_api::dcim_devices_create(&config, device)
        .await
        .unwrap();

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
