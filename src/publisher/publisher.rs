//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The actual request logic will be provided by the `thanix_client` crate.
//!
//! The `api_client` module will provide the actual client and request logic.
use serde::{Deserialize, Serialize};
use thanix_client::{
    paths::{self, DcimDevicesListQuery},
    types::{
        DeviceWithConfigContext, PaginatedDeviceWithConfigContextList,
        WritableDeviceWithConfigContextRequest,
    },
    util::ThanixClient,
};

use crate::{
    collectors::{dmi_collector::DmiInformation, network_collector::NetworkInformation},
    publisher::api_client::test_connection,
    Machine,
};

use super::publisher_exceptions::NetBoxApiError;

/// Test connection to NetBox.
///
/// # Paramters
///
/// - `client: &ThanixClient` - Reference to a `thanix_client` instance
///
/// # Returns
///
/// - `Result<(), NetBoxApiError` - Either returns an empty Ok() or a new instance of `NetBoxApiError`
pub fn probe(client: &ThanixClient) -> Result<(), NetBoxApiError> {
    println!("Probing connection to NetBox...");

    match test_connection(&client) {
        Ok(()) => {
            println!("\x1b[32mConnection established!\x1b[0m");
            Ok(())
        }
        Err(err) => panic!("Client unable to reach NetBox! {}", err),
    }
}

/// Register this machine in NetBox.
///
/// # Parameters
///
/// - `client: &ThanixClient` - Reference to a `thanix_client` instance
///
/// # Returns
///
/// TODO
pub fn register_machine(client: &ThanixClient, machine: Machine) -> Result<(), NetBoxApiError> {
    println!("Starting registration process. This may take a while...");
    let machines: Vec<DeviceWithConfigContext> = get_machines(client);
    search_for_matches(machine, &machines);
    Ok(())
}

/// Get list of machines.
///
/// Sends a `GET` request to the `/dcim/devices` endpoint to retrieve
/// a list of machines.
///
/// This is later needed to search for the current machine in the response to decide
/// whether to register a new one or update an existing one.
///
/// # Arguments
///
/// - `client: &ThanixClient` - Instance of the current API client.
///
/// # Returns
///
/// - `device_list: Vec<DeviceWithConfigContext>` - Returns a list of `DeviceWithConfigContext` objects.
///
/// # Panics
///
/// The function panics, when the request returns an error.
fn get_machines(client: &ThanixClient) -> Vec<DeviceWithConfigContext> {
    println!("Retrieving list of machines...");

    match paths::dcim_devices_list(client, DcimDevicesListQuery::default()) {
        Ok(response) => {
            println!("List received. Analyzing...");
            let debug_json = response.text().unwrap();

            let response_content: PaginatedDeviceWithConfigContextList =
                serde_json::from_str(&debug_json).unwrap();

            let device_list: Vec<DeviceWithConfigContext> = response_content.results;

            return device_list;
        }
        Err(err) => panic!("{}", err),
    }
}

/// Searches for matching device in list of machines.
///
/// Primary search parameters are the device's **serial number** and **UUID** acquired by `dmidecode`.
///
/// If a name has been provided, it is assumed that you do want to use this as primary search vector.
/// (Maybe because for your use case serial numbers or UUIDs are not reliable.)
///
/// # Parameters
///
/// - `machine: `Machine`` - Instance of a `Machine` containing all the local machines information.
/// - `device_list: &Vec<DeviceWithConfigContext>` - List of all devices.
///
/// # Returns
///
/// - `bool` - Depending on if the device has been found or not.
fn search_for_matches(machine: Machine, device_list: &Vec<DeviceWithConfigContext>) -> bool {
    if machine.name.is_none() {
        for device in device_list {
            if machine.dmi_information.system_information.serial == device.serial {
                println!("\x1b[32m[success]\x1b[0m Machine found using serial number!");
                return true;
            }
        }
        println!("\x1b[32m[info]\x1b[0m Machine not found using serial number.");
        return false;
    }
    for device in device_list {
        if device.name == machine.name {
            println!("\x1b[32m[success]\x1b[0m Machine found using name!");
            return true;
        }
    }
    println!("\x1b[32m[info]\x1b[0m Machine not found in registered machines.");
    false
}
