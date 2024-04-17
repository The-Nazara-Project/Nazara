//! # Translator Module
//!
//! This module handles the translation and processing of the data sent to or received from NetBox.
//!
//! TODO:
//! - Identify primary IPv4 or IPv6 using the primary_network_interface field from `ConfigData`.
use std::collections::HashMap;
use std::process;
use thanix_client::paths::{self, DcimPlatformsListQuery, IpamIpAddressesListQuery};
use thanix_client::types::{
    IPAddress, Platform, WritableDeviceWithConfigContextRequest, WritableVirtualMachineWithConfigContextRequest
};
use thanix_client::util::ThanixClient;

use crate::{configuration::config_parser::ConfigData, Machine};

/// Translate the machine information to a `WritableDeviceWithConfigContextRequest` required by
/// NetBox's API.
///
/// *Certain information provided in the config file, like the CPU platform, will be overwritten
/// if another one is detected by the collector!*
///
/// # Parameters
///
/// - `state: &ThanixClient` - API Client instance used for search and validation.
/// - `machine: &Machine` - Collected information about the device.
/// - `config_data: ConfigData` - Additional information about the device provided by config file
/// or CLI.
///
/// # Returns
///
/// - `device: WritableDeviceWithConfigContextRequest` - Payload for machine creation request
pub fn information_to_device(
    state: &ThanixClient,
    machine: &Machine,
    config_data: ConfigData,
) -> WritableDeviceWithConfigContextRequest {
    let wanted_platform: Option<String> = if let Some(arch) =
        machine.dmi_information.cpu_information.arch.as_ref()
    {
        println!("[info] CPU architecture was collected. Used by default, overriding possible config options...");
        Some(arch.clone())
    } else if let Some(config_value) = config_data.system.platform_name.as_ref() {
        println!("[info] Architecture was not collected. Using config specifications...");
        Some(config_value.clone())
    } else {
        println!("[warning] No cpu architecture specified. Proceeding with 'none'...");
        None
    };

    let mut payload: WritableDeviceWithConfigContextRequest = WritableDeviceWithConfigContextRequest::default();
    payload.name = Some(config_data.system.name);
    payload.device_type = config_data.system.device_type;
    payload.role = config_data.system.device_role;
    payload.tenant = config_data.system.tenant;
    payload.platform = match wanted_platform {
        Some(platform_name) => get_platform_id(&state, platform_name),
        None => None,
    };
    payload.serial = machine.dmi_information.system_information.serial.clone();
    // payload.asset_tag = todo!();
    payload.site = config_data.system.site_id; // TODO: Check if this exists.
    payload.rack = config_data.system.rack;
    payload.face = config_data.system.face;
    // payload.position = todo!();
    // payload.longitude = todo!();
    // payload.latitude = todo!();
    payload.status = config_data.system.status;
    payload.airflow = config_data.system.airflow;
    payload.comments = config_data.system.comments;
    // payload.config_template = todo!();
    payload.custom_fields = Some(HashMap::new());
    // payload.description = todo!();
    // payload.local_context_data = todo!();
    // payload.oob_ip = todo!();
    // TODO payload.primary_ip4 = todo!();
    // TODO payload.primary_ip6 = todo!();
    // payload.tags = todo!();
    // payload.virtual_chassis = todo!();
    // payload.vc_position = todo!();
    // payload.vc_priority = todo!();

    payload
}

/// Translate gathered information about the virtual machine into a usable Payload.
///
/// # Parameters
///
/// * state: `&ThanixClient` - The client instance to be used for communication.
/// * machine: `&Machine` - The collected information about the virtual machine.
/// * config_data: `&ConfigData` - Data parsed from the `nazar-config.toml`.
///
/// # Returns
///
/// * payload: `WritableVirtualMachineWithConfigContextRequest` - Payload for the VM POST or UPDATE
/// request.
pub fn information_to_vm(
    state: &ThanixClient,
    machine: &Machine,
    config_data: &ConfigData,
) -> WritableVirtualMachineWithConfigContextRequest {
    todo!("Translation of collected information to VM not implemented yet!")
}

/// Returns the ID of the platform this machine uses.
///
/// # Parameters
///
/// * state: `&ThanixClient` - The client required for searching for the platform.
fn get_platform_id(state: &ThanixClient, platform_name: String) -> Option<i64> {
    let platform_list: Vec<Platform>;

    match paths::dcim_platforms_list(&state, DcimPlatformsListQuery::default()) {
        Ok(response) => {
            println!("List received. Analyzing...");

            platform_list = match response {
                paths::DcimPlatformsListResponse::Http200(platforms) => platforms.results,
                _ => {
                    todo!("Handling of non 200 Response code when getting platforms not implemented yet.")
                }
            };
        }
        Err(e) => {
            eprintln!(
                "\x1b[31m[error]\x1b[0m Failure while receiving list of platforms.\n{}",
                e
            );
            process::exit(1);
        }
    };

    for platform in platform_list {
        if platform.name == platform_name {
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
///
/// The "primary_network_interface" paramter specified in the `nazara_config.toml`
/// will be used to specify which adress to search for.
///
/// # Parameters
///
/// * state: `&ThanixClient` - The client required for making API requests.
/// * machine: `&Machine` - The collected machine information.
fn get_primary_ip4(state: &ThanixClient, machine: &Machine, preferred_nwi: String) -> i64 {
    println!("Retrieving list of IPv4 adresses...");
    let ip4_list: Vec<IPAddress>;

    match paths::ipam_ip_addresses_list(&state, IpamIpAddressesListQuery::default()) {
        Ok(response) => {
            println!("List received. Analyzing...");

            ip4_list = match response {
                paths::IpamIpAddressesListResponse::Http200(adresses) => adresses.results,
                _ => {
                    todo!("Handling of non 200 Response code for Ipv4 retrieval not yet implemented.")
                }
            };
        }
        Err(e) => {
            eprintln!(
                "\x1b[31m[error]\x1b[0m Failure while retrieving list of IPv4 Adresses.\n{}",
                e
            );
            process::exit(1);
        }
    }

    for address in ip4_list {
        // TODO search for fitting IP Adress
    }
    return 0;
}

/// Get the id of the location provided by the config file.
///
/// Parameters
///
/// * state: `&ThanixClient` - The client required for searching for the location.
/// * location_id: `i64` - The id of the location the system is located at.
fn get_location_id(state: &ThanixClient, location_id: i64) -> Option<i64> {
    todo!("Getting device location not implemented yet.")
}   
