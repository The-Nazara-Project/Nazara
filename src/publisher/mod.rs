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

use crate::configuration::parser::{CommonConfig, MachineConfig};
use crate::publisher::api_client::{
    create_mac_address, create_vm, create_vm_interface, patch_ip, search_mac_address, search_vm,
    search_vm_interface, search_vm_ip, update_vm, update_vm_interface,
};
use crate::publisher::translator::{information_to_device, information_to_vm};
use crate::{
    Machine,
    collectors::network::NetworkInformation,
    configuration::parser::ConfigData,
    publisher::api_client::{
        create_device, create_interface, create_ip, search_device, search_interface, search_ip,
        update_device,
    },
};
pub use api_client::test_connection;
use api_client::update_interface;
use error::NetBoxApiError;
use serde_json::Value;
use std::collections::HashMap;
use thanix_client::paths::{
    IpamIpAddressesRetrieveResponse, dcim_interfaces_partial_update,
    dcim_mac_addresses_partial_update, ipam_ip_addresses_retrieve,
    virtualization_interfaces_partial_update,
};
use thanix_client::types::{
    MACAddressRequest, PatchedMACAddressRequest, PatchedWritableIPAddressRequest,
    PatchedWritableInterfaceRequest, PatchedWritableVMInterfaceRequest,
};
use thanix_client::{types::WritableIPAddressRequest, util::ThanixClient};

/// Register this machine or VM in NetBox.
///
/// # Parameters
/// * `client: &ThanixClient` - A client instance.
/// * `machine: Machine` - Information about the host machine collected by the [`super::collectors`] module.
/// * `config_data: ConfigData` - Nazara's configuration.
///
/// # Returns
///
/// An empty `Ok()` or a `NetBoxApiError` instance depending on operation outcome.
pub fn register_machine(
    client: &ThanixClient,
    machine: Machine,
    config_data: ConfigData,
) -> Result<(), NetBoxApiError> {
    println!("Starting registration process. This may take a while...");

    match &config_data.machine {
        MachineConfig::Device(x) => {
            println!("Registering device");

            let payload = information_to_device(&machine, &config_data.common, x);

            let Some(device_id) = search_device(
                client,
                &config_data.common.name,
                &machine.dmi_information.system_information.serial,
            ) else {
                let device_id = create_device(client, payload)?;

                // Create new interface object if no interface ID is given, or the given ID does
                // not exist.
                for interface in &machine.network_information {
                    let interface_id =
                        create_nwi(client, device_id, interface, &config_data.common)?;

                    create_ips(client, interface, interface_id, false)?;
                }
                println!("\x1b[32m[success]\x1b[0m Registration process completed!");
                return Ok(());
            };

            let updated_id = update_device(client, payload, device_id)?;

            let mut nwi_id;
            for interface in &machine.network_information {
                match search_interface(client, updated_id, &interface.name) {
                    Some(interface_id) => {
                        nwi_id = update_nwi(
                            client,
                            updated_id,
                            interface,
                            &config_data.common,
                            &interface_id,
                        )?;
                    }
                    None => {
                        nwi_id = create_nwi(client, updated_id, interface, &config_data.common)?;
                    }
                }

                // If the collected interface reports an IP address, the address must exist in NetBox.
                // If it is not assigned an ID, claim it to this interface.
                // If it is assigned an ID, it *must* be our interface ID. If it is not, the data is bogus.
                // Updating interfaces that don't belong to us is undefined behavior because it may interfere with
                // external services that provide these IP addresses.

                let (ipv4, ipv6) = search_device_ips(client, interface, None)?;
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
                                    assigned_object_type: Some(Some("dcim.interface".to_string())),
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
                                    assigned_object_type: Some(Some("dcim.interface".to_string())),
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
        MachineConfig::VM(x) => {
            println!("Registering virtual machine");
            let payload = information_to_vm(&machine, &config_data.common, x);

            let Some(vm_id) = search_vm(
                client,
                &config_data.common.name,
                &machine.dmi_information.system_information.serial,
            ) else {
                let vm_id = create_vm(client, payload)?;

                for interface in &machine.network_information {
                    let interface_id =
                        create_vm_nwi(client, vm_id, interface, &config_data.common)?;

                    create_ips(client, interface, interface_id, true)?;
                }
                return Ok(());
            };

            let updated_id = update_vm(client, payload, vm_id)?;
            let mut nwi_id;
            for interface in &machine.network_information {
                match search_vm_interface(client, updated_id, &interface.name) {
                    Some(interface_id) => {
                        nwi_id = update_vm_nwi(
                            client,
                            updated_id,
                            interface,
                            &config_data.common,
                            &interface_id,
                        )?;
                    }
                    None => {
                        nwi_id = create_vm_nwi(client, updated_id, interface, &config_data.common)?;
                    }
                }

                // If the collected interface reports an IP address, the address must exist in NetBox.
                // If it is not assigned an ID, claim it to this interface.
                // If it is assigned an ID, it *must* be our interface ID. If it is not, the data is bogus.
                // Updating interfaces that don't belong to us is undefined behavior because it may interfere with
                // external services that provide these IP addresses.

                let (ipv4, ipv6) = search_vm_ips(client, interface, None)?;
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
                                        "virtualization.vminterface".to_string(),
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
                                        "virtualization.vminterface".to_string(),
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

            return Ok(());
        }
    }

    Ok(())
}

/// Create new Network Interface object in NetBox.
/// Returns the ID of the newly created interface.
///
/// # Parameters
/// * `client: &ThanixClient` - The API client instance to use.
/// * `device_id: i64` - The device this interface belongs to.
/// * `interface: &NetworkInformation` - The interface to create.
/// * `config_data: &ConfigData` - The configuration read from the config file.
///
/// # Returns
/// * `Ok(i64)` - The ID of the newly created Interface.
/// * `Err(NetBoxApiError)` - Error with information about the operation's failure.
fn create_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    common: &CommonConfig,
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

    let mut payload = translator::information_to_interface(common, interface, &device_id);

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

fn create_vm_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    common: &CommonConfig,
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

    let mut payload = translator::information_to_vm_interface(common, interface, &device_id);

    payload.primary_mac_address = None;
    let intf_id = create_vm_interface(client, payload)?;

    // Assign Intf to MAC
    let mut patch = PatchedMACAddressRequest::default();
    patch.mac_address = Some(interface.mac_addr.clone().unwrap());
    patch.assigned_object_id = Some(Some(intf_id as u64));
    patch.assigned_object_type = Some(Some("virtualization.vminterface".to_string()));
    dcim_mac_addresses_partial_update(client, patch, mac).map_err(NetBoxApiError::from)?;

    // Update Intf with primary MAC
    let mut intf_patch = PatchedWritableVMInterfaceRequest::default();
    intf_patch.primary_mac_address = Some(Some(Value::from(mac)));
    virtualization_interfaces_partial_update(client, intf_patch, intf_id)?;

    Ok(intf_id)
}

/// Update a given NWI.
/// Creates a new Interface API payload and invokes the API call to update the interface.
/// Returns the ID of the interface.
///
/// # Parameters
/// * `client: &ThanixClient` - The API client instance to use.
/// * `device_id: i64` - The ID of the device this NWI belongs to.
/// * `interface: &NetworkInformation` - The information of the interface to update.
/// * `config_data &ConfigData` - The configuration data.
/// * `interface_id: &i64` - The ID of the interface to update.
fn update_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: &CommonConfig,
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

fn update_vm_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: &CommonConfig,
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
                assigned_object_type: Some("virtualization.vminterface".to_string()),
                custom_fields: Some(HashMap::new()),
                ..Default::default()
            },
        )?,
    };

    let mut payload = translator::information_to_vm_interface(&config_data, interface, &device_id);

    payload.primary_mac_address = None;
    update_vm_interface(client, payload, *interface_id)?;

    // Assign Intf to MAC
    let mut patch = PatchedMACAddressRequest::default();
    patch.assigned_object_id = Some(Some(*interface_id as u64));
    patch.assigned_object_type = Some(Some("virtualization.vminterface".to_string()));
    dcim_mac_addresses_partial_update(client, patch, mac).map_err(NetBoxApiError::from)?;

    // Update Intf with primary MAC
    let mut intf_patch = PatchedWritableVMInterfaceRequest::default();
    intf_patch.primary_mac_address = Some(Some(Value::from(mac)));
    virtualization_interfaces_partial_update(client, intf_patch, *interface_id)?;

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
/// # Parameters
/// * `client` - The API client instance to use.
/// * `interface` - The interface this address belongs to.
/// * `device_id` - The ID of the device these Addresses are linked to.
fn search_device_ips(
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

fn search_vm_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    vm_id: Option<i64>,
) -> Result<(Option<i64>, Option<i64>), NetBoxApiError> {
    let mut result: (Option<i64>, Option<i64>) = (None, None);
    if let Some(ipv4_address) = interface.v4ip {
        result = (
            search_vm_ip(client, &ipv4_address.to_string(), vm_id),
            result.1,
        );
    }

    if let Some(ipv6_address) = interface.v6ip {
        result = (
            result.0,
            search_vm_ip(client, &ipv6_address.to_string(), vm_id),
        );
    }
    Ok(result)
}

/// Creates the given interface's IPv4 and/or IPv6 address(es).
///
/// # Parameters
/// * `client: &ThanixClient` - The API client instance to use.
/// * `interface: &NetworkInformation` - The interface to get the IP Addresses from.
/// * `interface_id: i64` - The ID of the interface these addresses belong to.
fn create_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    interface_id: i64,
    is_vm: bool,
) -> Result<(), NetBoxApiError> {
    if let Some(ipv4_address) = interface.v4ip {
        let ipv4_payload: WritableIPAddressRequest =
            translator::information_to_ip(ipv4_address, interface_id, is_vm);

        create_ip(client, ipv4_payload)?;
    }

    if let Some(ipv6_address) = interface.v6ip {
        let ipv6_payload: WritableIPAddressRequest =
            translator::information_to_ip(ipv6_address, interface_id, is_vm);

        create_ip(client, ipv6_payload)?;
    };
    Ok(())
}
