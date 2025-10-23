//! # API Client Module
//!
//! This module's single responsibility is to take the payloads from the
//! [`publisher`](crate::publisher) module - which it created using the
//! [`translator`](crate::publisher::translator) module - and execute the individual API requests.
//!
//! Errors are escalated upwards.
extern crate thanix_client;

use crate::{NazaraError, error::NazaraResult, info, success};

use serde_json::Value;
use thanix_client::{
    paths::{
        DcimDevicesCreateResponse, DcimDevicesListQuery, DcimDevicesListResponse,
        DcimDevicesPartialUpdateResponse, DcimInterfacesCreateResponse, DcimInterfacesListQuery,
        DcimInterfacesListResponse, DcimInterfacesUpdateResponse, DcimMacAddressesListQuery,
        DcimMacAddressesListResponse, DcimMacAddressesUpdateResponse,
        IpamIpAddressesCreateResponse, IpamIpAddressesListQuery, IpamIpAddressesListResponse,
        IpamIpAddressesPartialUpdateResponse, VirtualizationInterfacesCreateResponse,
        VirtualizationInterfacesListQuery, VirtualizationInterfacesListResponse,
        VirtualizationInterfacesUpdateResponse, VirtualizationVirtualMachinesCreateResponse,
        VirtualizationVirtualMachinesListQuery, VirtualizationVirtualMachinesListResponse,
        VirtualizationVirtualMachinesPartialUpdateResponse, dcim_devices_create, dcim_devices_list,
        dcim_devices_partial_update, dcim_interfaces_create, dcim_interfaces_list,
        dcim_interfaces_update, dcim_mac_addresses_create, dcim_mac_addresses_list,
        dcim_mac_addresses_update, ipam_ip_addresses_create, ipam_ip_addresses_list,
        ipam_ip_addresses_partial_update, virtualization_interfaces_create,
        virtualization_interfaces_list, virtualization_interfaces_update,
        virtualization_virtual_machines_create, virtualization_virtual_machines_list,
        virtualization_virtual_machines_partial_update,
    },
    types::{
        Interface, MACAddressRequest, PatchedWritableDeviceWithConfigContextRequest,
        PatchedWritableIPAddressRequest, PatchedWritableVirtualMachineWithConfigContextRequest,
        WritableDeviceWithConfigContextRequest, WritableIPAddressRequest, WritableInterfaceRequest,
        WritableVMInterfaceRequest, WritableVirtualMachineWithConfigContextRequest,
    },
    util::ThanixClient,
};

/// Tests the connection to the NetBox API and verifies if your [`thanix_client`] version is compatible
/// with your NetBox version.
///
///	# Parameters
/// - `client`: The client instance to be used for communication.
///
/// # Returns
/// `Ok(())` if the connection to the API is successful and the [`thanix_client`] version is
/// compatible with the used NetBox version.
/// Otherwise an `Err` with [`NazaraError::NetBoxApiError`] if the connection fails or the
/// [`thanix_client`] version is not compatible with your NetBox version.
pub fn test_connection(client: &ThanixClient) -> Result<(), NazaraError> {
    let url: String = format!("{}/api/status/", client.base_url);

    let response = client
        .client
        .get(&url)
        .header(
            "Authorization",
            format!("Token {}", client.authentication_token),
        )
        .send()?;

    success!("Got response!");
    let json: Value = response.json::<Value>()?;

    if let Some(netbox_ver) = json.get("netbox-version").and_then(Value::as_str) {
        // Compare netbox version for compatibility
        if check_version_compatiblity(netbox_ver, thanix_client::version::VERSION) {
            info!("API client version compatible with NetBox version.");
            Ok(())
        } else {
            Err(NazaraError::VersionMismatch)
        }
    } else {
        Err(NazaraError::MissingVersion)
    }
}

/// Compares the NetBox version with the thanix_client version for compatibility.
///
/// Given the drastic differences in NetBox's API between `v3.x` and `v4.x` there are two different
/// release tracks for these two versions.
///
/// Version `v1.x` is compatible for NetBox Version `v3.6.x` and above, while `thanix_client`
/// version `v2.x` will be compatible with NetBox version `v4.x` and above.
///
/// # Parameters
/// - `netbox_version`: The version of the NetBox instance extracted from the response.
/// - `thanix_version`: The version of the installed `thanix_client` dependency.
fn check_version_compatiblity(netbox_version: &str, thanix_version: &str) -> bool {
    println!("Checking API client compatibility with used NetBox version...");
    let netbox_major = get_major_verison(netbox_version);
    let thanix_major = get_major_verison(thanix_version);

    match netbox_major {
        Some(3) => thanix_major == Some(1),
        Some(4) => thanix_major == Some(2),
        _ => false, // Unsupported version
    }
}

/// Gets the major version from the given version String.
///
/// # Parameters
/// - `version`: String representation of the application version.
///
/// # Returns
/// `Some(u32)` if the version can be parsed to an `u32`.
fn get_major_verison(version: &str) -> Option<u32> {
    version.split('.').next()?.parse().ok()
}

/// Search a device by sending a `DcimDevicesListQuery` with given search parameters.
/// Returns the ID of the device if it exists.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `name`: The name of the machine to search for.
/// - `serial:` The serial number of the machine.
///
/// # Returns
/// The ID of the device, if it exists. Else `None`.
pub fn search_device(client: &ThanixClient, name: &str, serial: &str) -> NazaraResult<Option<i64>> {
    println!("Checking if device is already registered...");
    let payload = DcimDevicesListQuery {
        name: Some(vec![name.to_owned()]),
        serial: Some(vec![serial.to_owned()]),
        ..Default::default()
    };
    match dcim_devices_list(client, payload)? {
        DcimDevicesListResponse::Http200(device_list) => Ok(device_list
            .results
            .ok_or(NazaraError::NetBoxMissingField(
                "PaginatedDeviceWithConfigContextList".into(),
                "results".into(),
            ))?
            .first()
            .map(|x| x.id)),
        DcimDevicesListResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

pub fn search_vm(client: &ThanixClient, name: &str, serial: &str) -> NazaraResult<Option<i64>> {
    println!("Checking if virtual machine is already registered...");
    let payload = VirtualizationVirtualMachinesListQuery {
        name: Some(vec![name.to_owned()]),
        serial: Some(vec![serial.to_owned()]),
        ..Default::default()
    };
    match virtualization_virtual_machines_list(client, payload)? {
        VirtualizationVirtualMachinesListResponse::Http200(device_list) => Ok(device_list
            .results
            .ok_or(NazaraError::NetBoxMissingField(
                "PaginatedVirtualMachineWithConfigContextList".into(),
                "results".into(),
            ))?
            .first()
            .map(|x| x.id)),
        VirtualizationVirtualMachinesListResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Send request to create a new device in NetBox.
/// Returns the ID of the newly created Device object.
///
/// # Parameters
/// - `client`: The [`ThanixClient`] instance to use for communication.
/// - `payload`: The information about the device serving as a request body.
pub fn create_device(
    client: &ThanixClient,
    payload: WritableDeviceWithConfigContextRequest,
) -> NazaraResult<i64> {
    println!("Creating device in NetBox...");

    match dcim_devices_create(client, payload)? {
        DcimDevicesCreateResponse::Http201(created_device) => {
            success!(
                "Device creation successful! New Device-ID: '{}'.",
                created_device.id
            );
            Ok(created_device.id)
        }
        DcimDevicesCreateResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Updates a device with a given ID.
/// Will simply overwrite the given device object in NetBox with the collected information.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The payload for the API request.
/// - `id`: The ID of the device to update.
///
/// # Returns
/// The ID of the updated device.
pub fn update_device(
    client: &ThanixClient,
    payload: PatchedWritableDeviceWithConfigContextRequest,
    id: i64,
) -> NazaraResult<i64> {
    println!("Updating device in NetBox...");
    match dcim_devices_partial_update(client, payload, id)? {
        DcimDevicesPartialUpdateResponse::Http200(updated_device) => {
            success!("Device updated successfully!");
            Ok(updated_device.id)
        }
        DcimDevicesPartialUpdateResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Send request to create a new device in NetBox.
///
/// # Parameters
/// - `client`: The [`ThanixClient`] instance to use for communication.
/// - `payload`: The information about the device serving as a request body.
///
/// # Returns
/// Returns the ID of the newly created Device object.
pub fn create_vm(
    client: &ThanixClient,
    payload: WritableVirtualMachineWithConfigContextRequest,
) -> NazaraResult<i64> {
    println!("Creating virtual machine in NetBox...");
    match virtualization_virtual_machines_create(client, payload)? {
        VirtualizationVirtualMachinesCreateResponse::Http201(created_device) => {
            success!(
                " Virtual Machine creation successful! New ID: '{}'.",
                created_device.id
            );
            Ok(created_device.id)
        }
        VirtualizationVirtualMachinesCreateResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Updates a device with a given ID.
/// Will simply overwrite the given device object in NetBox with the collected information.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The payload for the API request.
/// - `id`: The ID of the device to update.
///
/// # Returns
/// Returns the ID of the updated device.
pub fn update_vm(
    client: &ThanixClient,
    payload: PatchedWritableVirtualMachineWithConfigContextRequest,
    id: i64,
) -> NazaraResult<i64> {
    println!("Updating Virtual machine in NetBox...");

    match virtualization_virtual_machines_partial_update(client, payload, id)? {
        VirtualizationVirtualMachinesPartialUpdateResponse::Http200(updated_device) => {
            success!("VM updated successfully!");
            Ok(updated_device.id)
        }
        VirtualizationVirtualMachinesPartialUpdateResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Searches for a given MAC address object.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `mac_address`: The MAC address to search for.
///
/// # Returns
/// If it is found, the ID of the MAC address object in NetBox, else will return `None`.
pub fn search_mac_address(client: &ThanixClient, mac_address: &str) -> NazaraResult<Option<i64>> {
    println!("Searching for mac address...");

    let mut payload = DcimMacAddressesListQuery::default();
    payload.mac_address__ic = Some(vec![mac_address.to_string()]);

    match dcim_mac_addresses_list(client, payload)? {
        DcimMacAddressesListResponse::Http200(mac_addresses) => Ok(mac_addresses
            .results
            .ok_or(NazaraError::NetBoxMissingField(
                "PaginatedMACAddressList".into(),
                "results".into(),
            ))?
            .first()
            .map(|x| x.id)),
        DcimMacAddressesListResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Creates new MAC address object.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The API request payload.
///
/// # Returns
/// The ID of the newly created MAC address.
pub fn create_mac_address(client: &ThanixClient, payload: MACAddressRequest) -> NazaraResult<i64> {
    println!("Creating MAC address in NetBox...");
    match dcim_mac_addresses_create(client, payload)? {
        thanix_client::paths::DcimMacAddressesCreateResponse::Http201(result) => {
            success!(
                " MAC Address created successfully. New MAC Address-ID: '{}'",
                result.id
            );
            Ok(result.id)
        }
        thanix_client::paths::DcimMacAddressesCreateResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Updates a MAC address object.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The MAC address payload to update the MAC address with.
/// - `mac_address_id`: The ID of the MAC address to update.
#[allow(unused)]
pub fn update_mac_address(
    client: &ThanixClient,
    payload: MACAddressRequest,
    mac_address_id: i64,
) -> NazaraResult<i64> {
    match dcim_mac_addresses_update(client, payload, mac_address_id)? {
        DcimMacAddressesUpdateResponse::Http200(result) => {
            success!("MAC Address '{}' updated successfully.", result.id);
            Ok(result.id)
        }
        DcimMacAddressesUpdateResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Searches for interfaces with a given search parameters.
///
/// # Parameters
/// - `client`: The `ThanixClient` instance to use for communication.
/// - `device_id`: The ID of the device this interface is linked to.
/// - `name`: The name of this interface.
///
/// # Returns
/// Returns the ID of the interface when it is found, else returns `None`.
pub fn search_interface(
    client: &ThanixClient,
    device_id: i64,
    name: &String,
) -> NazaraResult<Option<i64>> {
    println!("Searching for interface '{name}'...");

    let payload = DcimInterfacesListQuery {
        device_id: Some(vec![device_id]),
        name: Some(vec![name.clone()]),
        ..Default::default()
    };

    match dcim_interfaces_list(client, payload)? {
        DcimInterfacesListResponse::Http200(interfaces) => Ok(interfaces
            .results
            .ok_or(NazaraError::NetBoxMissingField(
                "PaginatedInterfaceList".into(),
                "results".into(),
            ))?
            .first()
            .map(|x| x.id)),
        DcimInterfacesListResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

pub fn search_vm_interface(
    client: &ThanixClient,
    vm_id: i64,
    name: &String,
) -> NazaraResult<Option<i64>> {
    println!("Searching for VM interface '{name}'...");

    let payload = VirtualizationInterfacesListQuery {
        virtual_machine_id: Some(vec![vm_id]),
        name: Some(vec![name.clone()]),
        ..Default::default()
    };

    match virtualization_interfaces_list(client, payload)? {
        VirtualizationInterfacesListResponse::Http200(interfaces) => Ok(interfaces
            .results
            .ok_or(NazaraError::NetBoxMissingField(
                "PaginatedVMInterfaceList".into(),
                "results".into(),
            ))?
            .first()
            .map(|x| x.id)),
        VirtualizationInterfacesListResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Creates an interface object in NetBox.
///
/// # Parameters
/// - `client`: The client instance necessary for communication.
/// - `payload`: The payload for the API request.
///
/// # Returns
/// The ID of the interface object if the creation was successful.
pub fn create_interface(
    client: &ThanixClient,
    payload: WritableInterfaceRequest,
) -> NazaraResult<i64> {
    println!("Creating network interface in NetBox...");

    match dcim_interfaces_create(client, payload)? {
        DcimInterfacesCreateResponse::Http201(result) => {
            success!(
                " Interface created successfully. New Interface-ID: '{}'",
                result.id
            );
            Ok(result.id)
        }
        DcimInterfacesCreateResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

pub fn create_vm_interface(
    client: &ThanixClient,
    payload: WritableVMInterfaceRequest,
) -> NazaraResult<i64> {
    println!("Creating network interface in NetBox...");
    match virtualization_interfaces_create(client, payload)? {
        VirtualizationInterfacesCreateResponse::Http201(result) => {
            success!(
                " Interface created successfully. New Interface-ID: '{}'",
                result.id
            );
            Ok(result.id)
        }
        VirtualizationInterfacesCreateResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Updates a given interface object.
/// Returns the ID of the updated interface.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The API request payload to use.
/// - `interface_id`: The ID of the interface to update.
///
/// # Returns
/// The ID of the updated interface if the update was successful.
pub fn update_interface(
    client: &ThanixClient,
    payload: WritableInterfaceRequest,
    interface_id: i64,
) -> NazaraResult<i64> {
    match dcim_interfaces_update(client, payload, interface_id)? {
        DcimInterfacesUpdateResponse::Http200(result) => {
            success!("Interface '{}' updated successfully.", result.id);
            Ok(result.id)
        }
        DcimInterfacesUpdateResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

pub fn update_vm_interface(
    client: &ThanixClient,
    payload: WritableVMInterfaceRequest,
    interface_id: i64,
) -> NazaraResult<i64> {
    match virtualization_interfaces_update(client, payload, interface_id)? {
        VirtualizationInterfacesUpdateResponse::Http200(result) => {
            success!("Interface '{}' updated successfully.", result.id);
            Ok(result.id)
        }
        VirtualizationInterfacesUpdateResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Search given IP Address.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `address`: The address to search for.
/// - `device_id`: The ID of the device this address is linked to, if any.
///
/// # Returns
/// The ID of the IP address if it was found, otherwise `None`.
pub fn search_ip(
    client: &ThanixClient,
    address: &String,
    device_id: Option<i64>,
) -> NazaraResult<Option<i64>> {
    println!("Searching for IP Address '{address}'...");
    let payload = IpamIpAddressesListQuery {
        address: Some(vec![address.clone()]),
        device_id: device_id.map(|x| vec![x]),
        ..Default::default()
    };
    submit_ip_query(client, payload)
}

pub fn search_vm_ip(
    client: &ThanixClient,
    address: &String,
    vm_ip: Option<i64>,
) -> NazaraResult<Option<i64>> {
    println!("Searching for IP Address '{address}'...");
    let payload = IpamIpAddressesListQuery {
        address: Some(vec![address.clone()]),
        virtual_machine_id: vm_ip.map(|x| vec![x]),
        ..Default::default()
    };
    submit_ip_query(client, payload)
}

fn submit_ip_query(
    client: &ThanixClient,
    payload: IpamIpAddressesListQuery,
) -> NazaraResult<Option<i64>> {
    match ipam_ip_addresses_list(client, payload)? {
        IpamIpAddressesListResponse::Http200(addresses) => Ok(addresses
            .results
            .ok_or(NazaraError::NetBoxMissingField(
                "PaginatedIPAddressList".into(),
                "results".into(),
            ))?
            .first()
            .map(|x| x.id)),
        IpamIpAddressesListResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Creates new IP adress object.
///
/// # Parameters
/// - `client`: The client instance necessary for communication.
/// - `payload`: The payload to send.
///
/// # Returns
/// Returns the ID of the new IPAddress object if the creation of the IP address was successful.
pub fn create_ip(client: &ThanixClient, payload: WritableIPAddressRequest) -> NazaraResult<i64> {
    println!("Creating new IP address object...");
    match ipam_ip_addresses_create(client, payload)? {
        IpamIpAddressesCreateResponse::Http201(result) => {
            success!(
                " IP Address created successfully. New IP-ID: '{}'",
                result.id
            );
            Ok(result.id)
        }
        IpamIpAddressesCreateResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Patches a given IP address object.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The API call payload.
/// - `id`: The ID of the IP Address to update.
///
/// # Returns
/// The ID of the patched IP address object, if successful.
pub fn patch_ip(
    client: &ThanixClient,
    payload: PatchedWritableIPAddressRequest,
    id: i64,
) -> NazaraResult<i64> {
    println!("Patching IPs for given interface...");

    match ipam_ip_addresses_partial_update(client, payload, id)? {
        IpamIpAddressesPartialUpdateResponse::Http200(result) => Ok(result.id),
        IpamIpAddressesPartialUpdateResponse::Other(res) => {
            Err(NazaraError::UnexpectedResponse(res))
        }
    }
}

/// Gets a list of Interfaces.
///
/// # Parameters
/// - `state`: The API client instance to use.
#[allow(unused)]
pub fn get_interface_list(state: &ThanixClient) -> NazaraResult<Vec<Interface>> {
    println!("Retrieving list of interfaces...");

    match dcim_interfaces_list(state, DcimInterfacesListQuery::default())? {
        DcimInterfacesListResponse::Http200(interfaces) => interfaces.results.ok_or(
            NazaraError::NetBoxMissingField("PaginatedInterfaceList".into(), "results".into()),
        ),
        DcimInterfacesListResponse::Other(res) => Err(NazaraError::UnexpectedResponse(res)),
    }
}

/// Attempts to retrieve an interface by its name.
///
/// # Parameters
/// - `state`: The API client instance to use.
/// - `payload`: The payload to send.
#[allow(unused)]
pub fn get_interface_by_name(
    state: &ThanixClient,
    payload: &WritableInterfaceRequest,
) -> NazaraResult<Interface> {
    println!(
        "Trying to retrieve interface by name '{}'...",
        &payload.name
    );
    get_interface_list(state)?
        .into_iter()
        .find(|x| x.name.clone().is_some_and(|n| n == payload.name))
        .ok_or(NazaraError::Other(format!(
            "No interface '{}' with name found. Creation possibly failed.",
            &payload.name
        )))
}
