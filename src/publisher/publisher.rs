//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The actual request logic will be provided by the `thanix_client` crate.
use std::process;

/// TODO: 1. Implement Creation/update logic 2. Denest by splitting query logic off 3. Do not panic upon request fail
use thanix_client::{
    paths::{
        self, DcimDevicesCreateResponse, DcimDevicesListQuery, DcimDevicesListResponse,
        VirtualizationVirtualMachinesListQuery, VirtualizationVirtualMachinesListResponse,
    },
    types::{
        DeviceWithConfigContext, VirtualMachineWithConfigContext,
        WritableDeviceWithConfigContextRequest,
    },
    util::ThanixClient,
};

use crate::{
    configuration::config_parser::ConfigData,
    publisher::{
        api_client::{create_device, test_connection},
        translator,
    },
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
            println!("\x1b[32m[success] Connection established!\x1b[0m");
            Ok(())
        }
        Err(err) => panic!("Client unable to reach NetBox! {}", err),
    }
}

/// Register this machine or VM in NetBox.
///
/// # Parameters
///
/// - `client: &ThanixClient` - Reference to a `thanix_client` instance
///
/// # Returns
///
/// Empty Result object upon successful completeion. Otherwise a `NetBoxApiError`.
pub fn register_machine(
    client: &ThanixClient,
    machine: Machine,
    config_data: ConfigData,
) -> Result<(), NetBoxApiError> {
    println!("Starting registration process. This may take a while...");

    let nb_devices: DeviceListOrVMList = get_machines(client, &machine);

    if machine.dmi_information.system_information.is_virtual {
        todo!("Virtual machine creation not yet implemented!") // TODO: VM Creation / Update
    } else {
        let payload: WritableDeviceWithConfigContextRequest =
            translator::information_to_device(&client, &machine, config_data);

        match search_for_matches(&machine, &nb_devices) {
            Some(device_id) => {
                todo!("Device update not yet implemented.") // TODO Implement machine update
            }
            None => {
                create_device(client, &payload);
            }
        }
    }

    // check if virtual machine, create or update virtual machine.
    // if machine.dmi_information.system_information.is_virtual {
    //     match search_for_matches(&machine, &nb_devices) {
    //         Some(vm_id) => {
    //             match paths::virtualization_virtual_machines_update(
    //                 &client,
    //                 VirtualizationVirtualMachinesUpdateQuery::default(),
    //                 vm_id,
    //             ) {
    //                 Ok(response) => {
    //                     todo!()
    //                 }
    //                 Err(err) => {
    //                     panic!("{}", err)
    //                 }
    //             }
    //         }
    //         None => {
    //             match paths::virtualization_virtual_machines_create(
    //                 &client,
    //                 VirtualizationVirtualMachinesCreateQuery::default(),
    //             ) {
    //                 Ok(response) => {
    //                     todo!()
    //                 }
    //                 Err(err) => {
    //                     panic!("{}", err)
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     // proper physical machines
    //     match search_for_matches(&machine, &nb_devices) {
    //         Some(id) => {
    //             match paths::dcim_devices_update(&client, DcimDevicesUpdateQuery::default(), id) {
    //                 Ok(response) => {
    //                     todo!()
    //                 }
    //                 Err(err) => {
    //                     panic!("{}", err)
    //                 }
    //             }
    //         }
    //         None => match paths::dcim_devices_create(&client, DcimDevicesCreateQuery::default()) {
    //             Ok(response) => {
    //                 todo!()
    //             }
    //             Err(err) => {
    //                 panic!("{}", err)
    //             }
    //         },
    //     }
    // }

    Ok(())
}

/// Creates a new machine in NetBox by calling the `translator` module to translate the `machine` parameter
/// struct into the correct data type required by the API.
fn create_machine(client: &ThanixClient, machine: &Machine) -> Result<(), NetBoxApiError> {
    println!("Creating new machine in NetBox...");
    // let payload = translator::information_to_device(machine);
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

                let vm_list: Vec<VirtualMachineWithConfigContext> = match response {
                    VirtualizationVirtualMachinesListResponse::Http200(virtual_machines) => {
                        virtual_machines.results
                    }
                    _ => {
                        // TODO change the way Nazara exits here
                        eprintln!("\x1b[31m[error]\x1b[0m Failure while retrieving list of virtual machines. Please make sure your NetBox database is set up correctly.");
                        process::exit(1);
                    }
                };

                DeviceListOrVMList::VmList(vm_list)
            }
            Err(e) => panic!("{}", e),
        }
    } else {
        println!("Retrieving list of machines...");

        match paths::dcim_devices_list(client, DcimDevicesListQuery::default()) {
            Ok(response) => {
                println!("List received. Analyzing...");

                let device_list: Vec<DeviceWithConfigContext> = match response {
                    DcimDevicesListResponse::Http200(devices) => devices.results,
                    _ => {
                        todo!("Handling of non 200 Response code when getting machines not implemented yet!");
                    }
                };

                DeviceListOrVMList::DeviceList(device_list)
            }
            Err(e) => {
                // TODO change the way Nazara exits here
                eprintln!("\x1b[31m[error]\x1b[0m Failure while retrieving list of devices. Please make sure your NetBox database is set up correctly.\n{}", e);
                process::exit(1);
            }
        }
    }
}

/// Searches for matching device in list of machines and returns the device id in case of a match.
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
/// - `Option<i64, None>` - Either returns the id of the device or Vm found, or None.
fn search_for_matches(machine: &Machine, device_list: &DeviceListOrVMList) -> Option<i64> {
    match device_list {
        DeviceListOrVMList::DeviceList(devices) => {
            if machine.name.is_none() {
                println!("\x1b[36m[info]\x1b[0m No machine name provided. Searching via serial number...");
                for device in devices {
                    if machine.dmi_information.system_information.serial == device.serial {
                        println!("\x1b[32m[success]\x1b[0m Machine found using serial number!");
                        return Some(device.id);
                    }
                }
                println!("\x1b[36m[info]\x1b[0m Machine not found using serial number.");
                return None;
            }
            for device in devices {
                if device.name == machine.name {
                    println!("\x1b[32m[success]\x1b[0m Machine found using name!");
                    return Some(device.id);
                }
            }
            println!("\x1b[36m[info]\x1b[0m Machine not found in registered machines using name.");
            None
        }
        DeviceListOrVMList::VmList(virtual_machines) => {
            for vm in virtual_machines {
                if machine.name.as_ref().unwrap() == &vm.name {
                    println!("\x1b[32m[success]\x1b[0m VM found found using serial number!");
                    return Some(vm.id);
                }
            }
            println!("\x1b[36m[info]\x1b[0m VM not found in registered machines.");
            None
        }
    }
}
