//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The actual request logic will be provided by the `thanix_client` crate.
//!
//! The `api_client` module will provide the actual client and request logic.
use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use thanix_client::{
    paths::{self, DcimDevicesListQuery, VirtualizationVirtualMachinesListQuery},
    types::{
        DeviceWithConfigContext, PaginatedDeviceWithConfigContextList,
        PaginatedVirtualMachineWithConfigContextList, VirtualMachineWithConfigContext,
        WritableDeviceWithConfigContextRequest,
    },
    util::ThanixClient,
};

use crate::{
    collectors::{dmi_collector::DmiInformation, network_collector::NetworkInformation},
    publisher::api_client::test_connection,
    publisher::translator,
    Machine,
};

use super::publisher_exceptions::NetBoxApiError;

/// Workaround until rust supports return type polymorphism on stable
/// to allow for generics to be used instead.
enum DeviceListOrVMList {
    DeviceList(Vec<DeviceWithConfigContext>),
    VmList(Vec<VirtualMachineWithConfigContext>),
}

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

    let nb_devices: DeviceListOrVMList = get_machines(client, &machine);

    search_for_matches(&machine, &nb_devices);

    Ok(())
}

/// Get list of machines.
///
/// Sends a `GET` request to either the `/dcim/devices` endpoint, in case of physical machines,
/// or to the `virtualization/virtual-machines` endpoint to retrieve either
/// a list of machines or of virtual machines.
///
/// This depends on whether the `collector` has detected that the current device is pyhsical or virtual.
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
fn get_machines(client: &ThanixClient, machine: &Machine) -> DeviceListOrVMList {
    if machine.dmi_information.system_information.is_virtual {
        println!("Retrieving list of virtual machines...");

        match paths::virtualization_virtual_machines_list(
            client,
            VirtualizationVirtualMachinesListQuery::default(),
        ) {
            Ok(response) => {
                println!("List received. Analyzing...");
                let debug_json = response.text().unwrap();

                let response_content: PaginatedVirtualMachineWithConfigContextList =
                    serde_json::from_str(&debug_json).unwrap();

                let vm_list: Vec<VirtualMachineWithConfigContext> = response_content.results;

                DeviceListOrVMList::VmList(vm_list)
            }
            Err(err) => panic!("{}", err),
        }
    } else {
        println!("Retrieving list of machines...");

        match paths::dcim_devices_list(client, DcimDevicesListQuery::default()) {
            Ok(response) => {
                println!("List received. Analyzing...");
                let debug_json = response.text().unwrap();

                let response_content: PaginatedDeviceWithConfigContextList =
                    serde_json::from_str(&debug_json).unwrap();

                let device_list: Vec<DeviceWithConfigContext> = response_content.results;

                DeviceListOrVMList::DeviceList(device_list)
            }
            Err(err) => panic!("{}", err),
        }
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
fn search_for_matches(machine: &Machine, device_list: &DeviceListOrVMList) -> bool {
    match device_list {
        DeviceListOrVMList::DeviceList(devices) => {
            if machine.name.is_none() {
                println!("\x1b[36m[info]\x1b[0m No machine name provided. Searching via serial number...");
                for device in devices {
                    if machine.dmi_information.system_information.serial == device.serial {
                        println!("\x1b[32m[success]\x1b[0m Machine found using serial number!");
                        return true;
                    }
                }
                println!("\x1b[36m[info]\x1b[0m Machine not found using serial number.");
                return false;
            }
            for device in devices {
                if device.name == machine.name {
                    println!("\x1b[32m[success]\x1b[0m Machine found using name!");
                    return true;
                }
            }
            println!("\x1b[36m[info]\x1b[0m Machine not found in registered machines using name.");
            false
        }
        DeviceListOrVMList::VmList(virtual_machines) => {
            for vm in virtual_machines {
                if machine.name.as_ref().unwrap() == &vm.name {
                    println!("\x1b[32m[success]\x1b[0m VM found found using serial number!");
                    return true;
                }
            }
            println!("\x1b[36m[info]\x1b[0m VM not found in registered machines.");
            false
        }
    }
}
