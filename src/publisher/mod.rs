//! # Publisher Module
//!
//! This module serves as a controller for publishing information to NetBox.
//! It handles conditionals, error handling and everything else outside of actually sending API
//! requests.
//!
//! It "steers" which endpoints are used when, which payload is created when and hands these
//! responsibilities off to the [`crate::publisher::translator`] and
//! [`crate::publisher::api_client`] modules respectively.

mod api_client;
pub mod error;
pub mod trans_validation;
pub mod translator;

use std::collections::HashMap;

use crate::publisher::api_client::{create_mac_address, patch_ip, search_mac_address};
use crate::{
    Machine,
    collectors::network::NetworkInformation,
    configuration::parser::ConfigData,
    publisher::api_client::{
        create_device, create_interface, create_ip, search_device, search_interface, search_ip,
        test_connection, update_device,
    },
};
use api_client::update_interface;
use error::NetBoxApiError;
use serde_json::Value;
use thanix_client::paths::{
    IpamIpAddressesRetrieveResponse, dcim_interfaces_partial_update,
    dcim_mac_addresses_partial_update, ipam_ip_addresses_retrieve,
};
use thanix_client::types::{
    MACAddressRequest, PatchedMACAddressRequest, PatchedWritableIPAddressRequest,
    PatchedWritableInterfaceRequest,
};
use thanix_client::{
    types::{WritableDeviceWithConfigContextRequest, WritableIPAddressRequest},
    util::ThanixClient,
};

/// Test connection to NetBox.
///
/// - `client`: Reference to a `thanix_client` instance.
pub fn probe(client: &ThanixClient) -> Result<(), NetBoxApiError> {
    println!("Probing connection to NetBox...");

    match test_connection(client) {
        Ok(()) => {
            println!("\x1b[32m[success] Connection established!\x1b[0m");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

/// Register this machine or VM in NetBox.
///
/// - `client`: A client instance.
/// - `machine`: Information about the host machine collected by the [`super::collectors`] module.
/// - `config_data`: Nazara's configuration.
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
            translator::information_to_device(client, &machine, config_data.clone());

        match search_device(
            client,
            &config_data.system.name,
            &machine.dmi_information.system_information.serial,
        ) {
            Some(device_id) => {
                let updated_id = update_device(client, device_payload, device_id)?;

                let mut nwi_id;
                for interface in &machine.network_information {
                    match search_interface(client, updated_id, &interface.name) {
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
                            nwi_id = create_nwi(client, updated_id, interface, &config_data)?;
                        }
                    }
                    let (ipv4, ipv6) = search_ips(client, interface, None)?;

                    // If the collected interface reports an IP address, the address must exist in NetBox.
                    // If it is not assigned an ID, claim it to this interface.
                    // If it is assigned an ID, it *must* be our interface ID. If it is not, the data is bogus.
                    // Updating interfaces that don't belong to us is undefined behavior because it may interfere with
                    // external services that provide these IP addresses.

                    if let Some(ip) = interface.v4ip {
                        let ipv4 = ipv4.expect(&format!(
                            "IPv4 address \"{}\" was not registered in NetBox",
                            ip
                        ));

                        if let IpamIpAddressesRetrieveResponse::Http200(a) =
                            ipam_ip_addresses_retrieve(client, ipv4)?
                        {
                            if let Some(b) = a.assigned_object_id {
                                assert_eq!(b, nwi_id as u64);
                            } else {
                                patch_ip(
                                    client,
                                    PatchedWritableIPAddressRequest {
                                        status: Some("active".to_string()),
                                        assigned_object_type: Some(Some(
                                            "dcim.interface".to_string(),
                                        )),
                                        assigned_object_id: Some(Some(nwi_id as u64)),
                                        ..Default::default()
                                    },
                                    ipv4,
                                )?;
                            }
                        }
                    }

                    if let Some(ip) = interface.v6ip {
                        let ipv6 = ipv6.expect(&format!(
                            "IPv6 address \"{}\" was not registered in NetBox",
                            ip
                        ));

                        if let IpamIpAddressesRetrieveResponse::Http200(a) =
                            ipam_ip_addresses_retrieve(client, ipv6)?
                        {
                            if let Some(b) = a.assigned_object_id {
                                assert_eq!(b, nwi_id as u64);
                            } else {
                                patch_ip(
                                    client,
                                    PatchedWritableIPAddressRequest {
                                        status: Some("active".to_string()),
                                        assigned_object_type: Some(Some(
                                            "dcim.interface".to_string(),
                                        )),
                                        assigned_object_id: Some(Some(nwi_id as u64)),
                                        ..Default::default()
                                    },
                                    ipv6,
                                )?;
                            }
                        }
                    }
                }
                println!("\x1b[32m[success]\x1b[0m Update process completed!");
            }
            None => {
                let device_id = create_device(client, device_payload)?;

                // Create new interface object if no interface ID is given, or the given ID does
                // not exist.
                for interface in &machine.network_information {
                    let interface_id = create_nwi(client, device_id, interface, &config_data)?;

                    create_ips(client, interface, interface_id)?;
                }
                println!("\x1b[32m[success]\x1b[0m Registration process completed!")
            }
        }
    }
    Ok(())
}

/// Create new Network Interface object in NetBox.
/// Returns the ID of the newly created interface.
///
/// - `client`: The API client instance to use.
/// - `device_id`: The device this interface belongs to.
/// - `interface`: The interface to create.
/// - `config_data`: The configuration read from the config file.
fn create_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: &ConfigData,
) -> Result<i64, NetBoxApiError> {
    // Check if MAC exists before creating the interface, create the MAC if it doesn't exist
    let mac = match search_mac_address(client, interface.mac_addr.clone().unwrap().as_str()) {
        Some(x) => x,
        None => create_mac_address(
            client,
            MACAddressRequest {
                mac_address: interface.mac_addr.clone().unwrap(),
                custom_fields: Some(HashMap::new()),
                ..Default::default()
            },
        )?,
    };

    let mut payload = translator::information_to_interface(config_data, interface, &device_id);

    payload.primary_mac_address = None;
    let intf_id = create_interface(client, payload)?;

    // Assign Intf to MAC
    let mut patch = PatchedMACAddressRequest::default();
    patch.mac_address = Some(interface.mac_addr.clone().unwrap());
    patch.assigned_object_id = Some(Some(intf_id as u64));
    patch.assigned_object_type = Some(Some("dcim.interface".to_string()));
    dcim_mac_addresses_partial_update(client, patch, mac).map_err(NetBoxApiError::from)?;

    // Update Intf with primary MAC
    let mut intf_patch = PatchedWritableInterfaceRequest::default();
    intf_patch.primary_mac_address = Some(Some(Value::from(mac)));
    dcim_interfaces_partial_update(client, intf_patch, intf_id)?;

    Ok(intf_id)
}

/// Update a given NWI.
/// Creates a new Interface API payload and invokes the API call to update the interface.
/// Returns the ID of the interface.
///
/// - `client`: The API client instance to use.
/// - `device_id`: The ID of the device this NWI belongs to.
/// - `interface`: The information of the interface to update.
/// - `config_data`: The configuration data.
/// - `interface_id`: The ID of the interface to update.
fn update_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: ConfigData,
    interface_id: &i64,
) -> Result<i64, NetBoxApiError> {
    println!("Updating interface '{interface_id}' belonging to device '{device_id}'");

    // Check if MAC exists before creating the interface, otherwise create the MAC.
    let mac = match search_mac_address(client, interface.mac_addr.clone().unwrap().as_str()) {
        Some(x) => x,
        None => create_mac_address(
            client,
            MACAddressRequest {
                mac_address: interface.mac_addr.clone().unwrap(),
                assigned_object_id: Some(device_id as u64),
                assigned_object_type: Some("dcim.interface".to_string()),
                custom_fields: Some(HashMap::new()),
                ..Default::default()
            },
        )?,
    };

    let mut payload = translator::information_to_interface(&config_data, interface, &device_id);

    payload.primary_mac_address = None;
    update_interface(client, payload, *interface_id)?;

    // Assign Intf to MAC
    let mut patch = PatchedMACAddressRequest::default();
    patch.assigned_object_id = Some(Some(*interface_id as u64));
    patch.assigned_object_type = Some(Some("dcim.interface".to_string()));
    dcim_mac_addresses_partial_update(client, patch, mac).map_err(NetBoxApiError::from)?;

    // Update Intf with primary MAC
    let mut intf_patch = PatchedWritableInterfaceRequest::default();
    intf_patch.primary_mac_address = Some(Some(Value::from(mac)));
    dcim_interfaces_partial_update(client, intf_patch, *interface_id)?;

    Ok(*interface_id)
}

/// Search for a pair of IP addresses.
///
/// Checks if the `ipv4` and/or `ìpv6` addresses of the given `NetworkInformation` are set and
/// invokes `search_ip` on each or both of these addresses.
///
/// Returns a tuple of the IDs of each the IPv4 and IPv6 addresses if they are registered.
/// The first field represents the IPv4 Address, the second the IPv6 address.
/// If one or both are not already registered the value will be `None`.
///
/// - `client`: The API client instance to use.
/// - `interface`: The interface this address belongs to.
/// - `device_id`: The ID of the device these Addresses are linked to.
fn search_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    device_id: Option<i64>,
) -> Result<(Option<i64>, Option<i64>), NetBoxApiError> {
    let mut result: (Option<i64>, Option<i64>) = (None, None);
    if let Some(ipv4_address) = interface.v4ip {
        result = (
            search_ip(client, &ipv4_address.to_string(), device_id),
            result.1,
        );
    }

    if let Some(ipv6_address) = interface.v6ip {
        result = (
            result.0,
            search_ip(client, &ipv6_address.to_string(), device_id),
        );
    }
    Ok(result)
}

/// Creates the given interface's IPv4 and/or IPv6 address(es).
///
/// - `client`: The API client instance to use.
/// - `interface`: The interface to get the IP Addresses from.
/// - `interface_id`: The ID of the interface these addresses belong to.
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
