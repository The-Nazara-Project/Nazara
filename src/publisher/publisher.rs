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
        self, DcimDevicesListQuery, DcimDevicesListResponse,
        VirtualizationVirtualMachinesListQuery, VirtualizationVirtualMachinesListResponse,
    },
    types::{
        DeviceWithConfigContext, VirtualMachineWithConfigContext,
        WritableDeviceWithConfigContextRequest, WritableIPAddressRequest, WritableInterfaceRequest,
    },
    util::ThanixClient,
};

use crate::{
    configuration::config_parser::ConfigData,
    publisher::{
        api_client::{
            create_device, create_interface, create_ip, get_interface, get_interface_by_name,
            test_connection,
        },
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
        Err(err) => Err(err),
    }
}

/// Register this machine or VM in NetBox.
///
/// # Parameters
///
/// - `client: &ThanixClient` - Reference to a `thanix_client` instance.
/// - `machine: Machine` - Information about the host machine collected by the `collector` module.
/// - `config_data: ConfigData` - Nazara's configuration.
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
        let device_payload: WritableDeviceWithConfigContextRequest =
            translator::information_to_device(&client, &machine, config_data.clone());

        match search_for_matches(&machine, &nb_devices) {
            Some(device_id) => {
                todo!("Device update not yet implemented.") // TODO Implement machine update
            }
            None => {
                // Creates Device. Will need to be updated after IP Adress creation.
                let device_id = match create_device(client, &device_payload) {
                    Ok(id) => id,
                    Err(e) => {
                        println!("{}", e);
                        process::exit(1);
                    }
                };

                // Create new interface object if no interface ID is given, or the given ID does
                // not exist.
                for interface in &machine.network_information {
					println!("Interace: {}", interface.name);
                    let interface_payload: WritableInterfaceRequest =
                        translator::information_to_interface(
                            &machine,
                            config_data.clone(),
							interface,
                            &device_id,
                        );

                    let interface_id = match create_interface(client, interface_payload) {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("{}", e.to_string());
                            e.abort(None);
                        }
                    };

                    if let Some(ipv4_address) = interface.v4ip {
                        let ipv4_payload: WritableIPAddressRequest =
                            translator::information_to_ip(&machine, &config_data, ipv4_address, interface_id);

                        let _ = match create_ip(client, ipv4_payload) {
                            Ok(id) => id,
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                                e.abort(None);
                            }
                        };
                    }

                    if let Some(ipv6_address) = interface.v6ip {
                        let ipv6_payload: WritableIPAddressRequest =
                            translator::information_to_ip(&machine, &config_data, ipv6_address, interface_id);

                        let _ = match create_ip(client, ipv6_payload) {
                            Ok(id) => id,
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                                e.abort(None);
                            }
                        };
                    };
                }
            }
        }
    }
    Ok(())
}

/// Checks if a given network interface ID corresponds to a interface which already exsists.
///
/// # Parameter
///
/// * state: `&ThanixClient` - Client instance to use for communication.
/// * id: `&Option<i64>` - ID parameter retrieved from the config file.
///
/// # Returns
///
/// True/False depending on whether the interface exists.
fn interface_exists(state: &ThanixClient, id: i64) -> bool {
    println!("Trying to retrieve Interface '{}'", id);

    if get_interface(state, id).is_ok() {
        return true;
    }
    false
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
                        let exc = NetBoxApiError::Other(String::from("\x1b[31m[error]\x1b[0m Failure while retrieving list of virtual machines. Please make sure your NetBox database is set up correctly."));
                        exc.abort(Some(35))
                    }
                };

                DeviceListOrVMList::VmList(vm_list)
            }
            Err(e) => {
                let exc = NetBoxApiError::Reqwest(e);
                exc.abort(Some(34));
            }
        }
    } else {
        println!("Retrieving list of machines...");

        match paths::dcim_devices_list(client, DcimDevicesListQuery::default()) {
            Ok(response) => {
                println!("List received. Analyzing...");

                let device_list: Vec<DeviceWithConfigContext> = match response {
                    DcimDevicesListResponse::Http200(devices) => devices.results,
                    _ => {
                        let exc = NetBoxApiError::Other(String::from("\x1b[31m[error]\x1b[0m Failure while retrieving list of machines. Please make sure your NetBox database is set up correctly."));
                        exc.abort(Some(35));
                    }
                };

                DeviceListOrVMList::DeviceList(device_list)
            }
            Err(e) => {
                eprintln!("\x1b[31m[error]\x1b[0m Failure while retrieving list of devices. Please make sure your NetBox database is set up correctly.\n{}", e);
                let exc = NetBoxApiError::Reqwest(e);
                exc.abort(Some(34));
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
