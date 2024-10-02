//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The actual request logic will be provided by the `thanix_client` crate.
use thanix_client::{
    types::{
        WritableDeviceWithConfigContextRequest, WritableIPAddressRequest, WritableInterfaceRequest,
    },
    util::ThanixClient,
};

use crate::{
    collectors::network_collector::NetworkInformation,
    configuration::config_parser::ConfigData,
    publisher::{
        api_client::{
            create_device, create_interface, create_ip, search_device, search_interface, search_ip,
            test_connection, update_device, update_ip,
        },
        translator,
    },
    Machine,
};

use super::{api_client::update_interface, publisher_exceptions::NetBoxApiError};

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
                let mut nwi_id: i64;
                for interface in &machine.network_information {
                    match search_interface(client, &updated_id, &interface.name) {
                        Some(interface_id) => {
                            nwi_id = update_nwi(
                                client,
                                updated_id,
                                interface,
                                config_data.clone(),
                                &interface_id,
                            )?;
                        }
                        None => {
                            nwi_id =
                                create_nwi(client, updated_id, interface, config_data.clone())?;
                        }
                    }

                    match search_ips(client, interface, updated_id) {
                        Ok((Some(ipv4), Some(ipv6))) => {
                            update_ips(client, interface, nwi_id, Some(ipv4), Some(ipv6))?;
                        }
                        Ok((Some(ipv4), None)) => {
                            update_ips(client, interface, nwi_id, Some(ipv4), None)?;
                        }
                        Ok((None, Some(ipv6))) => {
                            update_ips(client, interface, nwi_id, None, Some(ipv6))?;
                        }
                        Ok((None, None)) => {
                            create_ips(client, interface, nwi_id)?;
                        }
                        Err(e) => {
                            e.abort(None);
                        }
                    }
                }
                println!("\x1b[32m[success] Update process completed!");
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
                println!("\x1b[32m[success]\x1b[0m Registration process completed!")
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
    interface_id: &i64,
) -> Result<i64, NetBoxApiError> {
    println!(
        "Updating interface '{}' belonging to device '{}'",
        interface_id, device_id
    );
    let payload: WritableInterfaceRequest =
        translator::information_to_interface(config_data, interface, &device_id);

    update_interface(client, payload, *interface_id)
}

/// Search for a pair of IP addresses.
///
/// Checks if the `ipv4` and/or `ìpv6` addresses of the given `NetworkInformation` are set and
/// invokes `search_ip` on each or both of these addresses.
///
/// # Parameters
///
/// * `client, &ThanixClient` - The API client instance to use.
/// * `interface: &NetworkInformation` - The interface this address belongs to.
/// * `device_id: i64` - The ID of the device these Addresses are linked to.
///
/// # Returns
///
/// * `Ok((Option<i64>, Option<i64>))` - A tuple of the IDs of each the IPv4 and IPv6 addresses if
/// they are registered. The first field represents the IPv4 Address, the second the IPv6 address.
/// If one or both are not already registered the value will be `None`.
/// * `Err(NetBoxApiError)` - In case something unforseen happens.
fn search_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    device_id: i64,
) -> Result<(Option<i64>, Option<i64>), NetBoxApiError> {
    let mut result: (Option<i64>, Option<i64>) = (None, None);
    if let Some(ipv4_address) = interface.v4ip {
        result = (
            search_ip(client, &ipv4_address.to_string(), &device_id),
            result.1,
        );
    }

    if let Some(ipv6_address) = interface.v6ip {
        result = (
            result.0,
            search_ip(client, &ipv6_address.to_string(), &device_id),
        );
    }
    Ok(result)
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
    ip_id_v4: Option<i64>,
    ip_id_v6: Option<i64>,
) -> Result<(), NetBoxApiError> {
    match (interface.v4ip, interface.v6ip) {
        (Some(ipv4_address), Some(ipv6_address)) => {
            // Update both IPv4 and IPv6 addresses
            if let Some(ipv4) = ip_id_v4 {
                let ipv4_payload = translator::information_to_ip(ipv4_address, interface_id);
                update_ip(client, ipv4_payload, ipv4)?;
            }

            if let Some(ipv6) = ip_id_v6 {
                let ipv6_payload = translator::information_to_ip(ipv6_address, interface_id);
                update_ip(client, ipv6_payload, ipv6)?;
            }
        }
        (Some(ipv4_address), None) => {
            // Only update IPv4
            if let Some(ipv4) = ip_id_v4 {
                let ipv4_payload = translator::information_to_ip(ipv4_address, interface_id);
                update_ip(client, ipv4_payload, ipv4)?;
            }
        }
        (None, Some(ipv6_address)) => {
            // Only update IPv6
            if let Some(ipv6) = ip_id_v6 {
                let ipv6_payload = translator::information_to_ip(ipv6_address, interface_id);
                update_ip(client, ipv6_payload, ipv6)?;
            }
        }
        (None, None) => {
            // No IPs to update
            create_ips(client, interface, interface_id)?;
        }
    }

    println!(
        "\x1b[32m[success]\x1b[0m IP Addresses of interface '{} (ID: '{}')' updated successfully!",
        interface.name, interface_id
    );
    Ok(())
}
