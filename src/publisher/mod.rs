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
pub mod translator;

use crate::configuration::parser::{CommonConfig, MachineConfig};
use crate::error::NazaraResult;
use crate::publisher::api_client::{
    create_mac_address, create_vm, create_vm_interface, patch_ip, search_mac_address, search_vm,
    search_vm_interface, search_vm_ip, update_vm, update_vm_interface,
};
use crate::publisher::translator::{
    compute_effective_name, information_to_device, information_to_existing_device,
    information_to_existing_vm, information_to_vm,
};
use crate::{
    Machine,
    collectors::network::NetworkInformation,
    configuration::parser::ConfigData,
    publisher::api_client::{
        create_device, create_interface, create_ip, search_device, search_interface, search_ip,
        update_device,
    },
};
use crate::{NazaraError, info, success};
pub use api_client::test_connection;
use api_client::update_interface;
use serde_json::Value;
use std::collections::HashMap;
use thanix_client::paths::{
    IpamIpAddressesRetrieveResponse, dcim_interfaces_partial_update,
    dcim_mac_addresses_partial_update, ipam_ip_addresses_retrieve,
    virtualization_interfaces_partial_update,
};
use thanix_client::types::{
    MACAddressRequest, PatchedMACAddressRequest, PatchedWritableDeviceWithConfigContextRequest,
    PatchedWritableIPAddressRequest, PatchedWritableInterfaceRequest,
    PatchedWritableVMInterfaceRequest, PatchedWritableVirtualMachineWithConfigContextRequest,
};
use thanix_client::{types::WritableIPAddressRequest, util::ThanixClient};

/// Register new device or virtual machine.
///
/// # Parameters
/// * `client: &ThanixClient` - A client instance.
/// * `machine: Machine` - The Machine to create.
/// * `config_data: ConfigData` - Nazara's configuration data.
///
/// # Returns
/// An empty `Ok()` or a [`NazaraError::NetBoxApiError`] depending on operation outcome.
pub fn register_machine(
    client: &ThanixClient,
    machine: Machine,
    config_data: ConfigData,
) -> NazaraResult<()> {
    println!("Starting registration process. This may take a while...");

    match &config_data.machine {
        MachineConfig::Device(config) => {
            println!("Registering device...");

            let payload = information_to_device(&machine, &config_data.common, config);
            let device_id = create_device(client, payload)?;

            // Create new interface object if no interface ID is given, or the given ID doesn not exist.
            for interface in &machine.network_information {
                let interface_id: i64 =
                    create_nwi(client, device_id, interface, &config_data.common)?;

                create_ips(client, interface, interface_id, false)?;
            }

            // Patch new device with primary IPs if they are set and not empty.
            if config_data
                .common
                .primary_ip4
                .as_ref()
                .map_or(false, |s| !s.is_empty())
                || config_data
                    .common
                    .primary_ip6
                    .as_ref()
                    .map_or(false, |s| !s.is_empty())
            {
                patch_device_primary_ips(client, &config_data, &machine, device_id)?;
            }

            success!("Registration processs completed!");
            return Ok(());
        }
        MachineConfig::VM(config) => {
            println!("Registering virtual machine");

            let payload = information_to_vm(&machine, &config_data.common, config);
            let vm_id = create_vm(client, payload)?;

            for interface in &machine.network_information {
                let interface_id = create_vm_nwi(client, vm_id, interface, &config_data.common)?;

                create_ips(client, interface, interface_id, true)?;
            }

            // Patch new device with primary IPs if they are set and not empty.
            if config_data
                .common
                .primary_ip4
                .as_ref()
                .map_or(false, |s| !s.is_empty())
                || config_data
                    .common
                    .primary_ip6
                    .as_ref()
                    .map_or(false, |s| !s.is_empty())
            {
                patch_vm_primary_ips(client, &config_data, &machine, vm_id)?;
            }

            success!("Registration process completed!");
            return Ok(());
        }
    }
}

/// Update device or virtual machine.
///
/// # Parameters
/// * `client: &ThanixClient` - A client instance.
/// * `machine: Machine` - The information about the machine to update.
/// * `config_data: ConfigData` - Nazara's configuration data.
/// * `machine_id: i64` - The ID of the device or VM to update handed over via CLI.
///
/// # Returns
/// An empty `Ok()` or a [`NazaraError::NetBoxApiError`] depending on operation outcome.
pub fn update_machine(
    client: &ThanixClient,
    machine: Machine,
    config_data: ConfigData,
    machine_id: i64,
) -> NazaraResult<()> {
    println!("Starting update process. This may take a while...");

    match &config_data.machine {
        MachineConfig::Device(config) => {
            let payload = information_to_existing_device(&machine, &config_data.common, config);
            let updated_id = update_device(client, payload, machine_id)?;

            let mut nwi_id;
            for interface in &machine.network_information {
                match search_interface(client, updated_id, &interface.name)? {
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
                    let ipv4 = ipv4.ok_or(NazaraError::NetBoxApiError(format!(
                        "IPv4 address \"{}\" was not registered in NetBox",
                        ip
                    )))?;

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
                    let ipv6 = ipv6.ok_or(NazaraError::NetBoxApiError(format!(
                        "IPv6 address \"{}\" was not registered in NetBox",
                        ip
                    )))?;

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

            // Patch primary IPs when one is specified in config.
            if config_data
                .common
                .primary_ip4
                .as_deref()
                .map_or(false, |s| !s.is_empty())
                || config_data
                    .common
                    .primary_ip6
                    .as_deref()
                    .map_or(false, |s| !s.is_empty())
            {
                patch_device_primary_ips(client, &config_data, &machine, updated_id)?;
            }

            success!("Device update process completed!");
            return Ok(());
        }
        MachineConfig::VM(config) => {
            let payload = information_to_existing_vm(&machine, &config_data.common, config);
            let updated_id = update_vm(client, payload, machine_id)?;
            let mut nwi_id;
            for interface in &machine.network_information {
                match search_vm_interface(client, updated_id, &interface.name)? {
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

            // Patch primary IPs when one is specified in config.
            if config_data
                .common
                .primary_ip4
                .as_deref()
                .map_or(false, |s| !s.is_empty())
                || config_data
                    .common
                    .primary_ip6
                    .as_deref()
                    .map_or(false, |s| !s.is_empty())
            {
                patch_vm_primary_ips(client, &config_data, &machine, updated_id)?;
            }

            success!("VM update process completed!");
            return Ok(());
        }
    }
}

/// Register or update machine depending on search results.
///
/// This is Nazara's previous default behaviour, yet it may be unreliable and is therefore discouraged.
/// And will be completely deprecated in the future.
///
/// # Parameters
/// - `client`: A client instance.
/// - `machine`: Information about the host machine collected by the [`super::collectors`] module.
/// - `config_data`: Nazara's configuration.
///
/// # Returns
/// An empty `Ok()` or a [`NazaraError::NetBoxApiError`] depending on operation outcome.
pub fn auto_register_or_update_machine(
    client: &ThanixClient,
    machine: Machine,
    config_data: ConfigData,
) -> NazaraResult<()> {
    println!("Starting auto register/update process. This may take a while...");

    // Compute effective name once (includes hostname fallback or @ expansion)
    let search_name = compute_effective_name(
        &config_data.common.name,
        &machine.dmi_information.system_information.hostname,
    );

    // Determine whether we are dealing with a Device or VM and search for it
    match &config_data.machine {
        MachineConfig::Device(_) => {
            if let Some(device_id) = search_device(
                client,
                &search_name,
                &machine.dmi_information.system_information.serial,
            )? {
                info!(
                    "Device found in NetBoxentry with entry ID '{}', updating...",
                    device_id
                );
                update_machine(client, machine, config_data, device_id)?;
            } else {
                info!("Device not found in NetBox, registering new entry");
                register_machine(client, machine, config_data)?;
            }
        }
        MachineConfig::VM(_) => {
            if let Some(vm_id) = search_vm(
                client,
                &search_name,
                &machine.dmi_information.system_information.serial,
            )? {
                info!("VM found in NetBox with entry ID '{}', updating...", vm_id);
                update_machine(client, machine, config_data, vm_id)?;
            } else {
                info!("VM not found in NetBox, registering new entry");
                register_machine(client, machine, config_data)?;
            }
        }
    }

    success!("Auto register/update process completed successfully!");
    Ok(())
}

/// Create new Network Interface object in NetBox.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `device_id`: The device this interface belongs to.
/// - `interface`: The interface to create.
/// - `config_data`: The configuration read from the config file.
///
/// # Returns
/// The ID of the newly created interface.
fn create_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    common: &CommonConfig,
) -> NazaraResult<i64> {
    // Check if MAC exists before creating the interface, create the MAC if it doesn't exist
    let intf = interface
        .mac_addr
        .clone()
        .ok_or(NazaraError::NetBoxApiError(
            "Missing \"mac_addr\" field".into(),
        ))?;
    let mac = match search_mac_address(client, &intf)? {
        Some(x) => x,
        None => create_mac_address(
            client,
            MACAddressRequest {
                mac_address: intf.clone(),
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
    patch.mac_address = Some(intf);
    patch.assigned_object_id = Some(Some(intf_id as u64));
    patch.assigned_object_type = Some(Some("dcim.interface".to_string()));
    dcim_mac_addresses_partial_update(client, patch, mac)?;

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
) -> NazaraResult<i64> {
    // Check if MAC exists before creating the interface, create the MAC if it doesn't exist
    let mac_addr = interface
        .mac_addr
        .clone()
        .ok_or(NazaraError::NetBoxApiError(
            "Missing \"mac_addr\" field".into(),
        ))?;

    let mac = match search_mac_address(client, &mac_addr)? {
        Some(x) => x,
        None => create_mac_address(
            client,
            MACAddressRequest {
                mac_address: mac_addr.clone(),
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
    patch.mac_address = Some(mac_addr);
    patch.assigned_object_id = Some(Some(intf_id as u64));
    patch.assigned_object_type = Some(Some("virtualization.vminterface".to_string()));
    dcim_mac_addresses_partial_update(client, patch, mac)?;

    // Update Intf with primary MAC
    let mut intf_patch = PatchedWritableVMInterfaceRequest::default();
    intf_patch.primary_mac_address = Some(Some(Value::from(mac)));
    virtualization_interfaces_partial_update(client, intf_patch, intf_id)?;

    Ok(intf_id)
}

/// Update a given NWI.
/// Creates a new Interface API payload and invokes the API call to update the interface.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `device_id`: The ID of the device this NWI belongs to.
/// - `interface`: The information of the interface to update.
/// - `config_data`: The configuration data.
/// - `interface_id`: The ID of the interface to update.
///
/// # Returns
/// The ID of the interface.
fn update_nwi(
    client: &ThanixClient,
    device_id: i64,
    interface: &NetworkInformation,
    config_data: &CommonConfig,
    interface_id: &i64,
) -> NazaraResult<i64> {
    info!("Updating interface '{interface_id}' belonging to device '{device_id}'");

    let mac_addr = interface
        .mac_addr
        .clone()
        .ok_or(NazaraError::NetBoxApiError(
            "Missing \"mac_addr\" field".into(),
        ))?;

    // Check if MAC exists before creating the interface, otherwise create the MAC.
    let mac = match search_mac_address(client, &mac_addr)? {
        Some(x) => x,
        None => create_mac_address(
            client,
            MACAddressRequest {
                mac_address: mac_addr.clone(),
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
    dcim_mac_addresses_partial_update(client, patch, mac)?;

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
) -> NazaraResult<i64> {
    info!("Updating interface '{interface_id}' belonging to device '{device_id}'");

    let mac_addr = interface
        .mac_addr
        .clone()
        .ok_or(NazaraError::NetBoxApiError(
            "Missing \"mac_addr\" field".into(),
        ))?;

    // Check if MAC exists before creating the interface, otherwise create the MAC.
    let mac = match search_mac_address(client, &mac_addr)? {
        Some(x) => x,
        None => create_mac_address(
            client,
            MACAddressRequest {
                mac_address: mac_addr,
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
    dcim_mac_addresses_partial_update(client, patch, mac)?;

    // Update Intf with primary MAC
    let mut intf_patch = PatchedWritableVMInterfaceRequest::default();
    intf_patch.primary_mac_address = Some(Some(Value::from(mac)));
    virtualization_interfaces_partial_update(client, intf_patch, *interface_id)?;

    Ok(*interface_id)
}

/// Search for a pair of IP addresses.
/// Checks if the `ipv4` and/or `ìpv6` addresses of the given [`NetworkInformation`] are set and
/// invokes [`search_ip`] on each or both of these addresses.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `interface`: The interface this address belongs to.
/// - `device_id`: The ID of the device these Addresses are linked to.
///
/// # Returns
/// A tuple of the IDs of each the IPv4 and IPv6 addresses if they are registered.
/// The first field represents the IPv4 Address, the second the IPv6 address.
/// If one or both are not already registered the value will be `None`.
fn search_device_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    device_id: Option<i64>,
) -> NazaraResult<(Option<i64>, Option<i64>)> {
    let mut result: (Option<i64>, Option<i64>) = (None, None);
    if let Some(ipv4_address) = interface.v4ip {
        result = (
            search_ip(client, &ipv4_address.to_string(), device_id)?,
            result.1,
        );
    }

    if let Some(ipv6_address) = interface.v6ip {
        result = (
            result.0,
            search_ip(client, &ipv6_address.to_string(), device_id)?,
        );
    }
    Ok(result)
}

fn search_vm_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    vm_id: Option<i64>,
) -> NazaraResult<(Option<i64>, Option<i64>)> {
    let mut result: (Option<i64>, Option<i64>) = (None, None);
    if let Some(ipv4_address) = interface.v4ip {
        result = (
            search_vm_ip(client, &ipv4_address.to_string(), vm_id)?,
            result.1,
        );
    }

    if let Some(ipv6_address) = interface.v6ip {
        result = (
            result.0,
            search_vm_ip(client, &ipv6_address.to_string(), vm_id)?,
        );
    }
    Ok(result)
}

/// Creates the given interface's IPv4 and/or IPv6 address(es).
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `interface`: The interface to get the IP Addresses from.
/// - `interface_id`: The ID of the interface these addresses belong to.
fn create_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    interface_id: i64,
    is_vm: bool,
) -> NazaraResult<()> {
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

fn patch_device_primary_ips(
    client: &ThanixClient,
    config_data: &ConfigData,
    machine: &Machine,
    device_id: i64,
) -> NazaraResult<()> {
    let mut patch = PatchedWritableDeviceWithConfigContextRequest::default();

    if let Some(primary_v4_str) = &config_data.common.primary_ip4 {
        for interface in &machine.network_information {
            if let Some(ipv4) = interface.v4ip {
                if ipv4.to_string() == *primary_v4_str {
                    let (ipv4_id, _) = search_device_ips(client, interface, Some(device_id))?;
                    if let Some(v4_id) = ipv4_id {
                        patch.primary_ip4 = Some(Some(Value::from(v4_id)));
                    }
                    break;
                }
            }
        }
    }

    if let Some(primary_v6_str) = &config_data.common.primary_ip6 {
        for interface in &machine.network_information {
            if let Some(ipv6) = interface.v6ip {
                if ipv6.to_string() == *primary_v6_str {
                    let (_, ipv6_id) = search_device_ips(client, interface, Some(device_id))?;
                    if let Some(v6_id) = ipv6_id {
                        patch.primary_ip6 = Some(Some(Value::from(v6_id)));
                    }
                    break;
                }
            }
        }
    }

    update_device(client, patch, device_id)?;
    info!("Patched primary IPs for device {device_id}");
    Ok(())
}

fn patch_vm_primary_ips(
    client: &ThanixClient,
    config_data: &ConfigData,
    machine: &Machine,
    vm_id: i64,
) -> NazaraResult<()> {
    let mut patch = PatchedWritableVirtualMachineWithConfigContextRequest::default();

    if let Some(primary_v4_str) = &config_data.common.primary_ip4 {
        for interface in &machine.network_information {
            if let Some(ipv4) = interface.v4ip {
                if ipv4.to_string() == *primary_v4_str {
                    let (ipv4_id, _) = search_vm_ips(client, interface, Some(vm_id))?;
                    if let Some(v4_id) = ipv4_id {
                        patch.primary_ip4 = Some(Some(Value::from(v4_id)));
                    }
                    break;
                }
            }
        }
    }

    if let Some(primary_v6_str) = &config_data.common.primary_ip6 {
        for interface in &machine.network_information {
            if let Some(ipv6) = interface.v6ip {
                if ipv6.to_string() == *primary_v6_str {
                    let (_, ipv6_id) = search_vm_ips(client, interface, Some(vm_id))?;
                    if let Some(v6_id) = ipv6_id {
                        patch.primary_ip6 = Some(Some(Value::from(v6_id)));
                    }
                    break;
                }
            }
        }
    }

    update_vm(client, patch, vm_id)?;
    info!("Patched primary IPs for VM {vm_id}");
    Ok(())
}
