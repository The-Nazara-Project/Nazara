//! # Translator Module
//!
//! This module's sole responsibility is translating system information into payloads usable by the
//! [`api_client`](crate::publisher::api_client) module to register the machine in NetBox.
//!
//! The information comes from the two collectors (both
//! [`crate::collectors::dmi`] and
//! [`crate::collectors::network`] as well as the
//! [`crate::configuration::parser`] module.
//! It is then formed into data structures that NetBox can understand.
//!
//! This approach has been chosen, so the collectors and configuration parser can remain relatively
//! unchanged in case NetBox significantly redesigns their API.

use crate::collectors::network::NetworkInformation;
use crate::{Machine, configuration::parser::ConfigData};
use core::net::IpAddr;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::process;
use std::str::FromStr;
use thanix_client::paths::{
    self, DcimPlatformsListQuery, DcimSitesListQuery, IpamIpAddressesListQuery,
};
use thanix_client::types::{
    IPAddress, WritableDeviceWithConfigContextRequest, WritableIPAddressRequest,
    WritableInterfaceRequest, WritableVirtualMachineWithConfigContextRequest,
};
use thanix_client::util::ThanixClient;

/// Translates the machine information to a [`WritableDeviceWithConfigContextRequest`]
/// as required by NetBox's API.
///
/// # Note
///
/// Certain information provided in the config file will be overwritten if a different one is detected by the collector!
///
/// # Parameters
/// - `state: &ThanixClient` - API Client instance used for search and validation.
/// - `machine: &Machine` -  Collected information about the device.
/// - `config_data: ConfigData` -  Additional information about the device provided by config file or CLI.
///
/// # Returns
///
/// - `WritableDeviceWithConfigContextRequest` - A device payload.
#[allow(clippy::field_reassign_with_default)]
pub fn information_to_device(
    state: &ThanixClient,
    machine: &Machine,
    config_data: ConfigData,
) -> WritableDeviceWithConfigContextRequest {
    println!("Creating Device object...");

    let mut payload = WritableDeviceWithConfigContextRequest::default();

    payload.name = Some(config_data.system.name.clone());
    payload.device_type = Value::from(config_data.system.device_type);
    payload.role = Value::from(config_data.system.device_role);
    if config_data.system.tenant.is_some() {
        payload.tenant = Some(Value::from(config_data.system.tenant));
    }
    let platform_id = get_platform_id(state, std::env::consts::ARCH.to_owned());
    if platform_id.is_some() {
        payload.platform = Some(Value::from(platform_id));
    }
    payload.serial = machine.dmi_information.system_information.serial.clone();
    // payload.asset_tag = todo!();
    payload.site = match get_site_id(state, &config_data) {
        Some(site_id) => Value::from(site_id),
        None => {
            eprintln!(
                "\x1b[31m[error]\x1b[0m An Error occured while validating the site ID or name."
            );
            process::exit(1);
        }
    };
    if config_data.system.rack.is_some() {
        payload.rack = Some(Value::from(config_data.system.rack));
    }
    payload.face = Some(config_data.system.face);
    // payload.position = todo!();
    // payload.longitude = todo!();
    // payload.latitude = todo!();
    payload.status = config_data.system.status;
    payload.airflow = Some(config_data.system.airflow);
    payload.comments = config_data.system.comments;
    // payload.config_template = todo!();
    payload.custom_fields = machine.custom_information.clone();
    payload.description = config_data.system.description;
    // payload.local_context_data = todo!();
    // payload.oob_ip = todo!();
    // payload.tags = todo!();
    // payload.virtual_chassis = todo!();
    // payload.vc_position = todo!();
    // payload.vc_priority = todo!();
    if config_data.system.tenant.is_some() {
        payload.tenant = Some(Value::from(config_data.system.tenant));
    }
    if config_data.system.location.is_some() {
        payload.location = Some(Value::from(config_data.system.location));
    }

    payload
}

/// Translate gathered information about the virtual machine into a usable Payload.
/// Returns a payload for the VM POST or UPDATE request.
///
/// - `state`: The client instance to be used for communication.
/// - `machine`: The collected information about the virtual machine.
/// - `config_data`: Data parsed from the `nazar-config.toml`.
#[allow(unused)]
pub fn information_to_vm(
    state: &ThanixClient,
    machine: &Machine,
    config_data: ConfigData,
) -> WritableVirtualMachineWithConfigContextRequest {
    todo!("Translation of collected information to VM not implemented yet!")
}

/// Translates gathered information into a Interface payload.
///
/// # Parameters
/// - `interface: &NetworkInformation` - The interface to be translated into a payload.
/// - `config_data: &ConfigData` - The configuration data.
/// - `device_id: &i64` - The ID of the device that this interface belongs to.
///
/// # Returns
/// - `WritableInterfaceRequest` - The payload to use for Interface operations.
#[allow(clippy::field_reassign_with_default)]
pub fn information_to_interface(
    config_data: &ConfigData,
    interface: &NetworkInformation,
    device_id: &i64,
) -> WritableInterfaceRequest {
    println!(
        "Creating Network Interface payload for '{}'...",
        &interface.name
    );

    let mut payload = WritableInterfaceRequest::default();

    payload.device = Value::from(device_id.to_owned());
    payload.name = interface.name.clone();

    // Get NwiConfig for the given interface
    let nwi_config = config_data.nwi.as_ref().and_then(|nwi_list| {
        nwi_list
            .iter()
            .find(|nwi| nwi.name.as_deref() == Some(&interface.name))
    });

    // This looks as horrible as it does, because at least for NetBox v3.6.9, we have to implement a
    // workaround on the API client side, making all Interface fields Options because we sometimes
    // get data back that does not comply with the api schema, failing serialization.
    payload.r#type = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.r#type.clone())
        .unwrap_or(String::from("other"));
    payload.parent = nwi_config.as_ref().and_then(|nwi| nwi.parent);
    payload.bridge = nwi_config.as_ref().and_then(|nwi| nwi.bridge);
    payload.lag = nwi_config.as_ref().and_then(|nwi| nwi.lag);
    payload.mtu = nwi_config.as_ref().and_then(|nwi| nwi.mtu);

    if let Some(x) = &interface.mac_addr {
        payload.primary_mac_address = Some(json!({"mac_address": x}));
    }
    payload.speed = Some(interface.interface_speed.unwrap_or_default());
    payload.description = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.description.clone())
        .unwrap_or_else(|| String::from("This interface was automatically created by Nazara."));
    payload.mode = Some(
        nwi_config
            .as_ref()
            .and_then(|nwi| nwi.mode.clone())
            .unwrap_or_default(),
    );
    payload.rf_role = Some(
        nwi_config
            .as_ref()
            .and_then(|nwi| nwi.rf_role.clone())
            .unwrap_or_default(),
    );
    payload.rf_channel = Some(
        nwi_config
            .as_ref()
            .and_then(|nwi| nwi.rf_channel.clone())
            .unwrap_or_default(),
    );
    payload.poe_mode = Some(
        nwi_config
            .as_ref()
            .and_then(|nwi| nwi.poe_mode.clone())
            .unwrap_or_default(),
    );
    payload.poe_type = Some(
        nwi_config
            .as_ref()
            .and_then(|nwi| nwi.poe_type.clone())
            .unwrap_or_default(),
    );
    payload.custom_fields = Some(
        nwi_config
            .as_ref()
            .and_then(|nwi| nwi.custom_fields.clone())
            .unwrap_or_default(),
    );
    payload.mark_connected = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.mark_connected)
        .unwrap_or(true);
    payload.enabled = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.enabled)
        .unwrap_or(interface.is_connected);
    payload.vdcs = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.vdcs.clone())
        .unwrap_or_default();
    payload.label = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.label.clone())
        .unwrap_or_default();
    payload.mgmt_only = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.mgmt_only)
        .unwrap_or(false);
    payload.tagged_vlans = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.tagged_vlans.clone())
        .unwrap_or_default();
    payload.wireless_lans = nwi_config
        .as_ref()
        .and_then(|nwi| nwi.wireless_lans.clone())
        .unwrap_or_default();
    payload.tags = Vec::new(); // FIXME: Currently not support tags, because they are hard.

    payload
}

/// Returns the payload necessary to create a new IP address.
///
/// # Parameters
/// - `interface_address: IpAddr` - The IpAddress of the interface to register.
/// - `interface_id: i64` - ID of the network interface this IP belongs to.
pub fn information_to_ip(interface_address: IpAddr, interface_id: i64) -> WritableIPAddressRequest {
    println!("Creating IP Address payload...");

    // payload.vrf = todo!();
    // payload.tenant = todo!();
    // payload.role = todo!();
    // payload.nat_inside = todo!();
    // payload.dns_name = todo!();
    // payload.tags = todo!();
    WritableIPAddressRequest {
        address: format!("{interface_address}"),
        status: String::from("active"),
        assigned_object_type: Some(String::from("dcim.interface")),
        assigned_object_id: Some(interface_id as u64),
        description: String::from("This Address was automatically created by Nazara."),
        comments: String::from("Automatically created by Nazara."),
        custom_fields: Some(HashMap::new()),
        ..Default::default()
    }
}

/// Returns the ID of the platform this machine uses.
///
/// # Parameters
/// - `state: &ThanixClient` - The client required for searching for the platform.
///
///	# Returns
///	* `Option<i64>` - The ID of the platform, if found. Else `None`.
///
/// # Panics
///
/// If the NetBox connection fails, this thread will panic.
fn get_platform_id(state: &ThanixClient, platform_name: String) -> Option<i64> {
    println!("Searching for id of platform '{platform_name}' ... ");

    let platform_list = match paths::dcim_platforms_list(state, DcimPlatformsListQuery::default()) {
        Ok(response) => {
            println!("List received. Analyzing...");

            match response {
                paths::DcimPlatformsListResponse::Http200(platforms) => platforms.results?,
                _ => {
                    todo!(
                        "Handling of non 200 Response code when getting platforms not implemented yet."
                    )
                }
            }
        }
        Err(e) => {
            eprintln!("[\x1b[31m[error]\x1b[0m Failure while receiving list of platforms.\n{e}");
            process::exit(1);
        }
    };

    for platform in platform_list {
        if platform.name == Some(platform_name.clone()) {
            println!("\x1b[32m[success]\x1b[0m Platform ID found. Proceeding...");
            return Some(platform.id);
        }
    }
    None
}

/// Returns the ID of the ipv4 Adress linked to this device if it exists.
/// If not, a new IPv4 Adress object will be created in NetBox.
///
/// The function will retrieve a list of IPv4 Adresses from NetBox,
/// then search this list for the IP Adress Nazara collected.
/// The `primary_network_interface` parameter specified in the `nazara_config.toml`
/// will be used to specify which adress to search for.
///
/// Returns the ID of the IP address object `i64` if a match has been found.
///
/// # Parameters
/// - `state`: The client required for making API requests.
/// - `machine`: The collected machine information.
fn get_primary_addresses(
    state: &ThanixClient,
    machine: &Machine,
    preferred_nwi: &str,
) -> Option<i64> {
    println!("Retrieving list of Addresses...");
    let key_nwi;

    if let Some(nwi_match) = machine
        .network_information
        .iter()
        .find(|nwi| nwi.name == preferred_nwi)
    {
        key_nwi = nwi_match;
    } else {
        eprintln!("\x1b[31m[error] Specified Network Interface '{preferred_nwi}' not found!");
        process::exit(1);
    };

    // TODO: Split this API call off so it is only done once.
    let ip_list: Vec<IPAddress> = match paths::ipam_ip_addresses_list(
        state,
        IpamIpAddressesListQuery::default(),
    ) {
        Ok(response) => {
            println!("IPAddress list received. Analyzing...");

            match response {
                paths::IpamIpAddressesListResponse::Http200(adresses) => adresses.results?,
                paths::IpamIpAddressesListResponse::Other(response) => {
                    eprintln!(
                        "\x1b[31m[error]\x1b[0m Failure while trying to retrieve list of IPAddresses. \n --- Unexpected response: {} ---",
                        response.text().unwrap()
                    );
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!(
                "\x1b[31m[error]\x1b[0m Failure while retrieving list of IPv4 Adresses.\n --- Unexpected response: {e} ---"
            );
            process::exit(1);
        }
    };

    let mut result: Option<i64> = None;

    for (idx, addr) in ip_list.iter().enumerate() {
        print!(
            "Searching for matching IP Adress... ({:?}/{:?})\r",
            idx + 1,
            ip_list.len()
        );
        let ip = IpAddr::from_str(addr.address.clone()?.split("/").next().unwrap()).unwrap(); // TODO: Errorhandling
        match ip {
            IpAddr::V4(x) => match key_nwi.v4ip {
                Some(y) => {
                    if x == y {
                        result = Some(addr.id);
                    }
                }
                None => todo!(),
            },
            IpAddr::V6(x) => match key_nwi.v6ip {
                Some(y) => {
                    if x == y {
                        result = Some(addr.id);
                    }
                }
                None => todo!(),
            },
        }
    }
    result
}

/// Search for the site specified in the config file by ID or by name.
/// Returns the ID of the site if found.
///
/// # Parameters
/// - `state`: The client required for performing API requests.
/// - `config_data`: The configuration data found in the config file.
fn get_site_id(state: &ThanixClient, config_data: &ConfigData) -> Option<i64> {
    println!("Searching for site...");
    if config_data.system.site_id.is_some() {
        // Check if site with given ID exists.
        let target = match paths::dcim_sites_retrieve(state, config_data.system.site_id.unwrap()) {
            Ok(response) => match response {
                paths::DcimSitesRetrieveResponse::Http200(site) => site.id,
                paths::DcimSitesRetrieveResponse::Other(response) => {
                    eprintln!(
                        "\x1b[31m[error]\x1b[0m Error while searching for site by site_id.\n--- Unexpected response: {} ---",
                        response.text().unwrap()
                    );
                    process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("\x1b[31m[error]\x1b[0m Error while searching for site.\n{e}");
                process::exit(1);
            }
        };
        println!("\x1b[32m[success]\x1b[0m Valid site ID. Proceeding...");
        return Some(target);
    }
    println!("\x1b[36m[info]\x1b[0m No 'site_id' specified. Searching by name...");
    let site_list;
    match paths::dcim_sites_list(state, DcimSitesListQuery::default()) {
        Ok(response) => match response {
            paths::DcimSitesListResponse::Http200(sites) => site_list = sites.results?,
            paths::DcimSitesListResponse::Other(response) => {
                eprintln!(
                    "\x1b[31[error] Error while retrieving site list.\n--- Unexpected response: {} ---",
                    response.text().unwrap()
                );
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("\x1b[31m[error]\x1b[0m Error while performing site list query.\n{e}");
            process::exit(1);
        }
    }
    let target = config_data.system.site_name.clone().unwrap();

    Some(
        site_list
            .iter()
            .find(|&site| site.name.as_ref().is_some_and(|x| *x == target))
            .unwrap()
            .id,
    )
}
