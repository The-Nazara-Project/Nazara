//! # Translator Module
//!
//! This module handles the translation and processing of the data sent to or received from NetBox.
//!
//! TODO:
//! - Identify primary IPv4 or IPv6 using the primary_network_interface field from `ConfigData`.
use std::process;
use thanix_client::paths::{self, DcimPlatformsListQuery};
use thanix_client::types::{
    Platform, WritableDeviceWithConfigContextRequest,
    WritableVirtualMachineWithConfigContextRequest,
};
use thanix_client::util::ThanixClient;

use crate::{configuration::config_parser::ConfigData, Machine};

use super::publisher_exceptions::NetBoxApiError;

/// Translate the machine information to a `WritableDeviceWithConfigContextRequest` required by
/// NetBox's API.
///
/// *Certain information provided in the config file, like the CPU platform, will be overwritten
/// if another one is detected by the collector!*
///
/// # Parameters
///
/// - `machine: &Machine` - Collected information about the device
/// - `config_data: ConfigData` - Additional information about the device provided by config file
/// or CLI
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

    let device: WritableDeviceWithConfigContextRequest = WritableDeviceWithConfigContextRequest {
        name: Some(config_data.system.name),
        device_type: config_data.system.device_type,
        role: config_data.system.device_role,
        tenant: config_data.system.tenant,
        platform: match wanted_platform {
            Some(platform_name) => get_platform_id(&state, platform_name),
            None => None,
        },
        serial: machine.dmi_information.system_information.serial,
        asset_tag: todo!(),
        site: todo!("Implement search logic for site id"),
        location: todo!(),
        rack: todo!(),
        face: todo!(),

    };
    device
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

/// Translate gathered information about the virtual machine into a usable Payload.
pub fn information_to_vm(
    state: &ThanixClient,
    machine: &Machine,
    config_data: &ConfigData,
) -> WritableVirtualMachineWithConfigContextRequest {
    todo!()
}

/// Validate the `Device` Payload.
///
/// # Parameters
/// * payload: `WritableDeviceWithConfigContextRequest` - The struct to validate.
///
/// # Returns
///
/// - Ok(())
fn validate_device_payload(payload: WritableDeviceWithConfigContextRequest) -> bool {
    println!("Validating device payload...");

    todo!("Device payload validation not implemented yet!");

    false
}
