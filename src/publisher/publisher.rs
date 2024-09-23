//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The actual request logic will be provided by the `thanix_client` crate.
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
    collectors::network_collector::NetworkInformation,
    configuration::config_parser::ConfigData,
    publisher::{
        api_client::{
            create_device, create_interface, create_ip, get_interface, get_interface_list,
            search_device, test_connection, update_device,
        },
        translator,
    },
    Machine,
};

use super::{api_client::update_interface, publisher_exceptions::NetBoxApiError};

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

        match search_device(
            client,
            &config_data.system.name,
            &machine.dmi_information.system_information.serial,
        ) {
            Some(device_id) => {
                let updated_id = match update_device(client, device_payload, device_id) {
                    Ok(id) => id,
                    Err(e) => e.abort(None),
                };

                // TODO:
                // For every interface collected:
                // 1. Check if interface already exists,
                //    If no: Create new
                //    If yes: Update/Overwrite
                // 2. Check if IP Address(es) linked to this device already exist.
                //    If no: Create new
                //    If yes: Update/Overwrite (delete old)
                let registered_nwis = get_interface_list(client)?;
                for interface in &machine.network_information {
                    if let Some(registered_nwis) = &registered_nwis {
                        // Check if any registered interface has the same name as the current `interface`
                        if registered_nwis
                            .iter()
                            .any(|nwi| nwi.name.as_ref() == Some(&interface.name))
                        {
                            // Interface is already registered, update it
                            let nwi_id: i64 = registered_nwis
                                .iter()
                                .find(|nwi| nwi.name.as_ref() == Some(&interface.name))
                                .unwrap()
                                .id; // Assume `id` exists or handle appropriately
                            update_nwi(client, updated_id, interface, config_data.clone(), nwi_id)?;

                            update_ips(client, interface, nwi_id)?;
                        } else {
                            // Interface not found, create a new one
                            let nwi_id: i64 =
                                create_nwi(client, updated_id, interface, config_data.clone())?;
                            create_ips(client, interface, nwi_id)?;
                        }
                    } else {
                        // No registered interfaces, create a new one
                        let nwi_id: i64 =
                            create_nwi(client, updated_id, interface, config_data.clone())?;
                        create_ips(client, interface, nwi_id)?;
                    }
                }
            }
            None => {
                let device_id = match create_device(client, device_payload) {
                    Ok(id) => id,
                    Err(e) => {
                        e.abort(None);
                    }
                };

                // Create new interface object if no interface ID is given, or the given ID does
                // not exist.
                for interface in &machine.network_information {
                    let interface_id =
                        match create_nwi(client, device_id, interface, config_data.clone()) {
                            Ok(id) => id,
                            Err(e) => e.abort(None),
                        };

                    create_ips(client, interface, interface_id)?;
                }
            }
        }
    }
    Ok(())
}

/// Create new Network Interface object in NetBox.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `device_id: i64` - The device this interface belongs to.
/// * `interface: &NetworkInformation` - The interface to create.
/// * `config_data: ConfigData` - The configuration read from the config file.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the newly created interface.
/// * `Err(NetBoxApiError)` - In case the API request fails.
fn create_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: ConfigData,
) -> Result<i64, NetBoxApiError> {
    let payload: WritableInterfaceRequest =
        translator::information_to_interface(config_data, interface, &device_id);

    create_interface(client, payload)
}

/// Update a given NWI.
///
/// Creates a new Interface API payload and invokes the API call to update the interface.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `device_id: i64` - The ID of the device this NWI belongs to.
/// * `interface: &NetworkInformation` - The information of the interface to update.
/// * `config_data: ConfigData` - The configuration data.
/// * `interface_id: i64` - The ID of the interface to update.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the updated interface.
/// * `Err(NetboxApiError)` - In case the connection or API request fails.
fn update_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: ConfigData,
    interface_id: i64,
) -> Result<i64, NetBoxApiError> {
    println!(
        "Updating interface '{}' belonging to device '{}'",
        interface_id, device_id
    );
    let payload: WritableInterfaceRequest =
        translator::information_to_interface(config_data, interface, &device_id);

    update_interface(client, payload, interface_id)
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

/// Creates the given interface's IPv4 and/or IPv6 address(es).
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `interface: &NetworkInformation` - The interface to get the IP Addresses from.
/// * `interface_id: i64` - The ID of the interface these addresses belong to.
///
/// # Returns
///
/// * `Ok(())` - If the registration has successfully been completed.
/// * `Err(NetboxApiError)` - If the creation failed.
fn create_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    interface_id: i64,
) -> Result<(), NetBoxApiError> {
    if let Some(ipv4_address) = interface.v4ip {
        let ipv4_payload: WritableIPAddressRequest =
            translator::information_to_ip(ipv4_address, interface_id);

        create_ip(client, ipv4_payload)?;
    }

    if let Some(ipv6_address) = interface.v6ip {
        let ipv6_payload: WritableIPAddressRequest =
            translator::information_to_ip(ipv6_address, interface_id);

        create_ip(client, ipv6_payload)?;
    };
    Ok(())
}

/// Update all IPs of a given interface.
///
/// # Parameters
///
/// * `client, &ThanixClient` - The API client instance to use.
/// * `interface: &NetworkInformation` - The interface this address belongs to.
/// * `interface_id: i64` - The ID of the interface object in NetBox.
///
/// # Returns
///
/// * `Ok(())` - If the operation was successful.
/// * `Err(NetBoxApiError)` - In case something unforseen happens.
fn update_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    interface_id: i64,
) -> Result<(), NetBoxApiError> {
    if let Some(ipv4_address) = interface.v4ip {
        let ipv4_payload: WritableIPAddressRequest =
            translator::information_to_ip(ipv4_address, interface_id);

        // update_ip() ?
    }

    if let Some(ipv6_address) = interface.v6ip {
        let ipv6_payload: WritableIPAddressRequest =
            translator::information_to_ip(ipv6_address, interface_id);

        // update_ip() ?
    }
    println!(
        "\x1b[32m[success]\x1b[0m IP Addresses of interface '{} (ID: '{}')' updated successfully!",
        interface.name, interface_id
    );
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
fn search_for_matches(
    machine: &Machine,
    device_list: &DeviceListOrVMList,
    config_data: &ConfigData,
) -> Option<i64> {
    match device_list {
        DeviceListOrVMList::DeviceList(devices) => {
            if machine.name.is_none() {
                println!("\x1b[36m[info]\x1b[0m No machine name provided. Searching via serial number...");
                for device in devices {
                    if let Some(device_name) = &device.name {
                        if config_data.system.name == *device_name {
                            println!(
                                "\x1b[32m[success]\x1b[0m Found machine using configured name!"
                            );
                            return Some(device.id);
                        }
                    }
                    if device.serial == machine.dmi_information.system_information.serial {
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
