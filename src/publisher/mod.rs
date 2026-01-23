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
use crate::{IpAssignmentMode, NazaraError, failure, info, success};
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
/// * `ip_mode: IpAssignmentMode` - Decide how to handle IPs with DHCP.
///
/// # Returns
/// An empty `Ok()` or a [`NazaraError::NetBoxApiError`] depending on operation outcome.
pub fn register_machine(
    client: &ThanixClient,
    machine: Machine,
    config_data: ConfigData,
    ip_mode: IpAssignmentMode,
) -> NazaraResult<()> {
    println!("Starting registration process. This may take a while...");

    match &config_data.machine {
        MachineConfig::Device(config) => {
            let payload = information_to_device(&machine, &config_data.common, config);
            let device_id = create_device(client, payload)?;

            for interface in &machine.network_information {
                let nwi_id = create_nwi(client, device_id, interface, &config_data.common)?;

                match ip_mode {
                    IpAssignmentMode::Static => {
                        create_ips(client, interface, nwi_id, false)?;
                    }
                    IpAssignmentMode::DhcpObserved => {
                        if let Some(ip) = interface.v4ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, false, device_id)?;
                        }
                        if let Some(ip) = interface.v6ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, false, device_id)?;
                        }
                    }
                    IpAssignmentMode::DhcpIgnore => {}
                }
            }

            if matches!(ip_mode, IpAssignmentMode::Static) {
                patch_device_primary_ips(client, &config_data, &machine, device_id)?;
            }

            success!("Registration processs completed!");
            return Ok(());
        }

        MachineConfig::VM(config) => {
            let payload = information_to_vm(&machine, &config_data.common, config);
            let vm_id = create_vm(client, payload)?;

            for interface in &machine.network_information {
                let nwi_id = create_vm_nwi(client, vm_id, interface, &config_data.common)?;

                match ip_mode {
                    IpAssignmentMode::Static => {
                        create_ips(client, interface, nwi_id, true)?;
                    }
                    IpAssignmentMode::DhcpObserved => {
                        if let Some(ip) = interface.v4ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, true, vm_id)?;
                        }
                        if let Some(ip) = interface.v6ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, true, vm_id)?;
                        }
                    }
                    IpAssignmentMode::DhcpIgnore => {}
                }
            }

            if matches!(ip_mode, IpAssignmentMode::Static) {
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
    ip_mode: IpAssignmentMode,
) -> NazaraResult<()> {
    match &config_data.machine {
        MachineConfig::Device(config) => {
            let payload = information_to_existing_device(&machine, &config_data.common, config);
            let device_id = update_device(client, payload, machine_id)?;

            for interface in &machine.network_information {
                let nwi_id = match search_interface(client, device_id, &interface.name)? {
                    Some(id) => update_nwi(client, device_id, interface, &config_data.common, &id)?,
                    None => create_nwi(client, device_id, interface, &config_data.common)?,
                };

                match ip_mode {
                    IpAssignmentMode::Static => {
                        reconcile_static_device_ips(client, interface, nwi_id)?;
                    }
                    IpAssignmentMode::DhcpObserved => {
                        if let Some(ip) = interface.v4ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, false, device_id)?;
                        }
                        if let Some(ip) = interface.v6ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, false, device_id)?;
                        }
                    }
                    IpAssignmentMode::DhcpIgnore => {}
                }
            }

            if matches!(ip_mode, IpAssignmentMode::Static) {
                patch_device_primary_ips(client, &config_data, &machine, device_id)?;
            }
        }

        MachineConfig::VM(config) => {
            let payload = information_to_existing_vm(&machine, &config_data.common, config);
            let vm_id = update_vm(client, payload, machine_id)?;

            for interface in &machine.network_information {
                let nwi_id = match search_vm_interface(client, vm_id, &interface.name)? {
                    Some(id) => update_vm_nwi(client, vm_id, interface, &config_data.common, &id)?,
                    None => create_vm_nwi(client, vm_id, interface, &config_data.common)?,
                };

                match ip_mode {
                    IpAssignmentMode::Static => {
                        reconcile_static_vm_ips(client, interface, nwi_id)?;
                    }
                    IpAssignmentMode::DhcpObserved => {
                        if let Some(ip) = interface.v4ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, true, vm_id)?;
                        }
                        if let Some(ip) = interface.v6ip {
                            ensure_dhcp_ip(client, &ip.to_string(), nwi_id, true, vm_id)?;
                        }
                    }
                    IpAssignmentMode::DhcpIgnore => {}
                }
            }

            if matches!(ip_mode, IpAssignmentMode::Static) {
                patch_vm_primary_ips(client, &config_data, &machine, vm_id)?;
            }
        }
    }

    success!("Update completed");
    Ok(())
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
    ip_mode: IpAssignmentMode,
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
                update_machine(client, machine, config_data, device_id, ip_mode)?;
            } else {
                info!("Device not found in NetBox, registering new entry");
                register_machine(client, machine, config_data, ip_mode)?;
            }
        }
        MachineConfig::VM(_) => {
            if let Some(vm_id) = search_vm(
                client,
                &search_name,
                &machine.dmi_information.system_information.serial,
            )? {
                info!("VM found in NetBox with entry ID '{}', updating...", vm_id);
                update_machine(client, machine, config_data, vm_id, ip_mode)?;
            } else {
                info!("VM not found in NetBox, registering new entry");
                register_machine(client, machine, config_data, ip_mode)?;
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

/// Patch device "primary-ip" fields after registration or on update.
fn patch_device_primary_ips(
    client: &ThanixClient,
    config_data: &ConfigData,
    machine: &Machine,
    device_id: i64,
) -> NazaraResult<()> {
    println!("Applying primary IPs to entry...");

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

/// Patch VM "primary-ip" fields after registration or on update.
fn patch_vm_primary_ips(
    client: &ThanixClient,
    config_data: &ConfigData,
    machine: &Machine,
    vm_id: i64,
) -> NazaraResult<()> {
    println!("Applying primary IPs to entry...");

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
    info!("Patched primary IPs for VM '{vm_id}'");
    Ok(())
}

/// Ensure a DHCP-managed IP address exists and is correctly assigned to an interface.
///
/// Guarantees that the given IP address string `ip_str` is present in NetBox
/// and linked to the correct interface. It supports both devices and vm interfaces.
/// If the IP already exists but is assigned to a different interface, it will be reassigned.
/// If the IP does not exist yet, it will be created.
///
/// # Parameters
///
/// * `client: &ThanixClient` - Reference client used for API operations.
/// * `ip_str: &str` - The IPv4 or IPv6 address as a string.
/// * `interface_id: i64` - NetBox ID of the interface that the IP should belong to.
/// * `is_vm: bool` - Whether we look for a vm or device interface.
/// * `parent_id: i64` - NetBox ID of the parent object:
///     - For devices: the `dcim.device` ID.
///     - For VMs: the `virtualization.virtual_machine` ID
///
/// # Returns
///
/// * `Ok(())` - If the IP is successfully verified or created and correctly assigned.
/// * `Err(NazaraError)` if:
///     - The IP string is invalid
///     - NetBox API call fails
fn ensure_dhcp_ip(
    client: &ThanixClient,
    ip_str: &str,
    interface_id: i64,
    is_vm: bool,
    parent_id: i64,
) -> NazaraResult<()> {
    println!("[DHCP-Mode] Ensuring DHCP IP assignments...");
    let existing_id = if is_vm {
        search_vm_ip(client, &ip_str.to_owned(), Some(parent_id))?
    } else {
        search_ip(client, &ip_str.to_owned(), Some(parent_id))?
    };

    match existing_id {
        Some(id) => {
            if let IpamIpAddressesRetrieveResponse::Http200(ip) =
                ipam_ip_addresses_retrieve(client, id)?
            {
                if ip.assigned_object_id != Some(interface_id as u64) {
                    info!(
                        "[DHCP-Mode] Reassigning IP '{ip_str}' from interface '{:?}' to '{interface_id}'",
                        ip.assigned_object_id
                    );
                    patch_ip(
                        client,
                        PatchedWritableIPAddressRequest {
                            assigned_object_type: Some(Some(if is_vm {
                                "virtualization.vminterface".to_string()
                            } else {
                                "dcim.interface".to_string()
                            })),
                            assigned_object_id: Some(Some(interface_id as u64)),
                            status: Some("active".to_string()),
                            ..Default::default()
                        },
                        id,
                    )?;
                    success!("[DHCP-Mode] IP address reassignment successful!");
                } else {
                    info!(
                        "[DHCP-Mode] IP '{ip_str}' is already correctly assigned to interface '{interface_id}'."
                    );
                }
            }
        }
        None => {
            info!("[DHCP-Mode] IP address '{ip_str}' is not yet registered. Registering...");
            let addr = ip_str
                .parse()
                .map_err(|_| NazaraError::NetBoxApiError(format!("Invalid IP: {ip_str}")))?;

            let payload = translator::information_to_dhcp_ip(addr, interface_id, is_vm);
            create_ip(client, payload)?;
            success!(
                "[DHCP-Mode] IP '{}' created and assigned to interface '{}'",
                ip_str,
                interface_id
            );
        }
    }

    Ok(())
}

/// Reconcile statically configured IP addresses for a device interface.
///
/// This function enforces the following invariants for static IP mode:
///
/// - IP reported by the machine **must exist in NetBox**
/// - If an IP exists, but is **unassigned**, it will be assigned to this interface
/// - If an IP exists and is **already assigned**, it must belong to this interface
/// - New IP addresses are **never created implicitly**
///
/// This is used during device updates, when DHCP mode is used.
///
/// # Parameters
///
/// * `client: &ThanixClient` - Reference client used for API connection.
/// * `interface: &NetworkInformation` - The local network interface with its statically configured IPv4 and IPv6 addresses.
/// * `interface_id: i64` - The ID of the corresponding network interface in NetBox.
///
/// # Returns
///
/// * `Ok(())` - If all Ips are valid and correctly assigned.
/// * `Err(NazaraError)` - if:
///     - A referenced IP does not exist in NetBox.
///     - A referenced IP is assigned to a **different** interface
///     - Any NetBox API operation fails
fn reconcile_static_device_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    interface_id: i64,
) -> NazaraResult<()> {
    // IPv4
    if let Some(ip) = interface.v4ip {
        info!("[DHCP-Mode] Reconciling static IPv4: '{ip}'");

        let ipv4_id = search_ip(client, &ip.to_string(), None)?.ok_or_else(|| {
            failure!("[DHCP-Mode] IPv4 {ip} not found in NetBox!");
            NazaraError::NetBoxApiError(format!(
                "IPv4 address \"{}\" was not registered in NetBox",
                ip
            ))
        })?;

        if let IpamIpAddressesRetrieveResponse::Http200(a) =
            ipam_ip_addresses_retrieve(client, ipv4_id)?
        {
            if let Some(assigned) = a.assigned_object_id {
                assert_eq!(assigned, interface_id as u64);
                info!("[DHCP-Mode] IPv4 '{ip}' correctly assigned to interface {interface_id}");
            } else {
                info!(
                    "[DHCP-Mode] Patching IPv4 '{ip}' to assign to interface '{interface_id}'..."
                );
                patch_ip(
                    client,
                    PatchedWritableIPAddressRequest {
                        status: Some("active".to_string()),
                        assigned_object_type: Some(Some("dcim.interface".to_string())),
                        assigned_object_id: Some(Some(interface_id as u64)),
                        ..Default::default()
                    },
                    ipv4_id,
                )?;
                success!("[DHCP-Mode] IPv4 '{ip}' assigned successfully!");
            }
        }
    }

    // IPv6
    if let Some(ip) = interface.v6ip {
        info!("[DHCP-Mode] Reconciling static IPv6: '{ip}'");

        let ipv6_id = search_ip(client, &ip.to_string(), None)?.ok_or_else(|| {
            failure!("[DHCP-Mode] IPv6 '{ip}' not found in NetBox!");
            NazaraError::NetBoxApiError(format!(
                "IPv6 address \"{}\" was not registered in NetBox",
                ip
            ))
        })?;

        if let IpamIpAddressesRetrieveResponse::Http200(a) =
            ipam_ip_addresses_retrieve(client, ipv6_id)?
        {
            if let Some(assigned) = a.assigned_object_id {
                assert_eq!(assigned, interface_id as u64);
                info!("[DHCP-Mode] IPv6 '{ip}' correctly assigned to interface '{interface_id}'");
            } else {
                info!("[DHCP-Mode] Patching IPv6 '{ip}' to assign to interface '{interface_id}'");
                patch_ip(
                    client,
                    PatchedWritableIPAddressRequest {
                        status: Some("active".to_string()),
                        assigned_object_type: Some(Some("dcim.interface".to_string())),
                        assigned_object_id: Some(Some(interface_id as u64)),
                        ..Default::default()
                    },
                    ipv6_id,
                )?;
                success!("[DHCP-Mode] IPv6 '{ip}' assigned successfully!");
            }
        }
    }

    Ok(())
}

/// Reconcile statically configured IP addresses for a VM interface.
///
/// This function enforces the following invariants for static IP mode:
///
/// - IP reported by the machine **must exist in NetBox**
/// - If an IP exists, but is **unassigned**, it will be assigned to this interface
/// - If an IP exists and is **already assigned**, it must belong to this interface
/// - New IP addresses are **never created implicitly**
///
/// This is used during VM updates, when DHCP mode is used.
///
/// # Parameters
///
/// * `client: &ThanixClient` - Reference client used for API connection.
/// * `interface: &NetworkInformation` - The local network interface with its statically configured IPv4 and IPv6 addresses.
/// * `interface_id: i64` - The ID of the corresponding network interface in NetBox.
///
/// # Returns
///
/// * `Ok(())` - If all Ips are valid and correctly assigned.
/// * `Err(NazaraError)` - if:
///     - A referenced IP does not exist in NetBox.
///     - A referenced IP is assigned to a **different** interface
///     - Any NetBox API operation fails
fn reconcile_static_vm_ips(
    client: &ThanixClient,
    interface: &NetworkInformation,
    interface_id: i64,
) -> NazaraResult<()> {
    // IPv4
    if let Some(ip) = interface.v4ip {
        info!("[DHCP-Mode] Reconciling VM static IPv4: {ip}");

        let ipv4_id = search_vm_ip(client, &ip.to_string(), None)?.ok_or_else(|| {
            failure!("[DHCP-Mode] VM IPv4 '{ip}' not found in NetBox");
            NazaraError::NetBoxApiError(format!(
                "IPv4 address \"{}\" was not registered in NetBox",
                ip
            ))
        })?;

        if let IpamIpAddressesRetrieveResponse::Http200(a) =
            ipam_ip_addresses_retrieve(client, ipv4_id)?
        {
            if let Some(assigned) = a.assigned_object_id {
                assert_eq!(assigned, interface_id as u64);
                info!(
                    "[DHCP-Mode] VM IPv4 '{ip}' correctly assigned to interface '{interface_id}'"
                );
            } else {
                patch_ip(
                    client,
                    PatchedWritableIPAddressRequest {
                        status: Some("active".to_string()),
                        assigned_object_type: Some(Some("virtualization.vminterface".to_string())),
                        assigned_object_id: Some(Some(interface_id as u64)),
                        ..Default::default()
                    },
                    ipv4_id,
                )?;
                success!("[DHCP-Mode] VM IPv4 '{ip}' assigned successfully!");
            }
        }
    }

    // IPv6
    if let Some(ip) = interface.v6ip {
        info!("[DHCP-Mode] Reconciling VM static IPv6: '{ip}'");

        let ipv6_id = search_vm_ip(client, &ip.to_string(), None)?.ok_or_else(|| {
            failure!("[DHCP-Mode] VM IPv6 '{ip}' not found in NetBox!");
            NazaraError::NetBoxApiError(format!(
                "IPv6 address \"{}\" was not registered in NetBox",
                ip
            ))
        })?;

        if let IpamIpAddressesRetrieveResponse::Http200(a) =
            ipam_ip_addresses_retrieve(client, ipv6_id)?
        {
            if let Some(assigned) = a.assigned_object_id {
                assert_eq!(assigned, interface_id as u64);
                info!(
                    "[DHCP-Mode] VM IPv6 '{ip}' correctly assigned to interface '{interface_id}'"
                );
            } else {
                info!(
                    "[DHCP-Mode] Patching VM IPv6 '{ip}' to assign to interface '{interface_id}'"
                );
                patch_ip(
                    client,
                    PatchedWritableIPAddressRequest {
                        status: Some("active".to_string()),
                        assigned_object_type: Some(Some("virtualization.vminterface".to_string())),
                        assigned_object_id: Some(Some(interface_id as u64)),
                        ..Default::default()
                    },
                    ipv6_id,
                )?;
                success!("[DHCP-Mode] VM IPv6 '{ip}' assigned successfully!");
            }
        }
    }

    Ok(())
}
