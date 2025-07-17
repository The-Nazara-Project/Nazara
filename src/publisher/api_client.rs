//! # API Client Module
//!
//! This module's single responsibility is to take the payloads from the
//! [`publisher`](crate::publisher) module - which it created using the
//! [`translator`](crate::publisher::translator) module - and execute the individual API requests.
//!
//! Errors are escalated upwards.
extern crate thanix_client;

use super::error::{self, NetBoxApiError};
use serde_json::Value;
use thanix_client::{
    paths::{
        DcimDevicesCreateResponse, DcimDevicesListQuery, DcimDevicesListResponse,
        DcimDevicesPartialUpdateResponse, DcimInterfacesListQuery, DcimInterfacesListResponse,
        DcimMacAddressesListQuery, DcimMacAddressesListResponse, IpamIpAddressesListQuery,
        IpamIpAddressesListResponse, IpamIpAddressesPartialUpdateResponse,
        VirtualizationInterfacesCreateResponse, VirtualizationInterfacesListQuery,
        VirtualizationInterfacesListResponse, VirtualizationInterfacesUpdateResponse,
        VirtualizationVirtualMachinesCreateResponse, VirtualizationVirtualMachinesListQuery,
        VirtualizationVirtualMachinesListResponse,
        VirtualizationVirtualMachinesPartialUpdateResponse, dcim_devices_create, dcim_devices_list,
        dcim_devices_partial_update, dcim_interfaces_create, dcim_interfaces_list,
        dcim_interfaces_retrieve, dcim_interfaces_update, dcim_mac_addresses_create,
        dcim_mac_addresses_list, dcim_mac_addresses_update, ipam_ip_addresses_create,
        ipam_ip_addresses_list, ipam_ip_addresses_partial_update, virtualization_interfaces_create,
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

/// Tests the connection to the NetBox API and verifies if your `thanix_client` version is compatible
/// with your NetBox version.
///
/// Returns `Ok(())` if the connection to the API is successful and the `thanix_client` version is
/// compatible with the used NetBox version.
/// Returns an `Err` with `publisher_exceptions::NetBoxApiError` if the connection fails or the
/// `thanix_client` version is not compatible with your NetBox version.
///
///	# Parameters
/// * `client: &ThanixClient` - The client instance to be used for communication.
pub fn test_connection(client: &ThanixClient) -> Result<(), error::NetBoxApiError> {
    let url: String = format!("{}/api/status/", client.base_url);

    let response = client
        .client
        .get(&url)
        .header(
            "Authorization",
            format!("Token {}", client.authentication_token),
        )
        .send()
        .map_err(error::NetBoxApiError::Reqwest)?;

    println!("Got response!");

    if response.status().is_success() {
        let json: Value = response.json::<Value>().map_err(NetBoxApiError::Reqwest)?;

        if let Some(netbox_ver) = json.get("netbox-version").and_then(Value::as_str) {
            // Compare netbox version for compatibility
            if check_version_compatiblity(netbox_ver, thanix_client::version::VERSION) {
                println!(
                    "\x1b[32m[success]\x1b[0m API client version compatible with NetBox version."
                );
                Ok(())
            } else {
                Err(error::NetBoxApiError::VersionMismatch(String::from(
                    "Client version incompatible with NetBox version! Use client v1.x for NetBox v3.6.x and above, and v2.x for NetBox 4.x.",
                )))
            }
        } else {
            Err(error::NetBoxApiError::MissingVersion(String::from(
                "NetBox version missing from response. Please check your installation.",
            )))
        }
    } else {
        Err(error::NetBoxApiError::Reqwest(
            response.error_for_status().unwrap_err(),
        ))
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
/// * `netbox_version: &str` - The version of the NetBox instance extracted from the response.
/// * `thanix_version: &str` - The version of the installed `thanix_client` dependency.
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
/// Returns `Some(u32)` if the version can be parsed to an `u32`.
///
/// # Parameters
/// - `version`: String representation of the application version.
fn get_major_verison(version: &str) -> Option<u32> {
    version.split('.').next()?.parse().ok()
}

/// Search a device by sending a `DcimDevicesListQuery` with given search parameters.
/// Returns the ID of the device if it exists.
///
///	# Parameters
/// - `client: &ThanixClient` - The API client instance to use.
/// - `name: &str` - The name of the machine to search for.
/// - `serial: &str` - The serial number of the machine.
///
/// # Returns
/// - `Option<i64>` - The ID of the device, if it exists. Else `None`.
///
/// # Panics
///
/// This function panics under following conditions:
/// - Search results are indecisive. E.g more than one result.
/// - The API request returns an unexpected response code.
/// - The API request fails (e.g the connection fails).
pub fn search_device(client: &ThanixClient, name: &str, serial: &str) -> Option<i64> {
    println!("Checking if device is already registered...");
    let payload = DcimDevicesListQuery {
        name: Some(vec![name.to_owned()]),
        serial: Some(vec![serial.to_owned()]),
        ..Default::default()
    };

    match dcim_devices_list(client, payload) {
        Ok(response) => match response {
            DcimDevicesListResponse::Http200(device_list) => {
                if device_list.results.as_ref()?.len() == 1 {
                    return Some(device_list.results?.first()?.id);
                }
                if device_list.results?.is_empty() {
                    return None;
                }
                panic!(
                    "Ambiguous search result. Device listed more than once. Please check your data."
                );
            }
            DcimDevicesListResponse::Other(other) => {
                panic!("{}", other.text().unwrap());
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn search_vm(client: &ThanixClient, name: &str, serial: &str) -> Option<i64> {
    println!("Checking if virtual machine is already registered...");
    let payload = VirtualizationVirtualMachinesListQuery {
        name: Some(vec![name.to_owned()]),
        serial: Some(vec![serial.to_owned()]),
        ..Default::default()
    };

    match virtualization_virtual_machines_list(client, payload) {
        Ok(response) => match response {
            VirtualizationVirtualMachinesListResponse::Http200(device_list) => {
                if device_list.results.as_ref()?.len() == 1 {
                    return Some(device_list.results?.first()?.id);
                }
                if device_list.results?.is_empty() {
                    return None;
                }
                panic!(
                    "Ambiguous search result. Device listed more than once. Please check your data."
                );
            }
            VirtualizationVirtualMachinesListResponse::Other(other) => {
                panic!("{}", other.text().unwrap());
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

/// Send request to create a new device in NetBox.
/// Returns the ID of the newly created Device object.
///
/// # Parameters
/// - `client: &ThanixClient` - The [`ThanixClient`] instance to use for communication.
/// - `payload: &WritableDeviceWithConfigContextRequest` - The information about the device serving as a request body.
///
/// # Panics
///
/// This function panics if the response code is not `201`.
pub fn create_device(
    client: &ThanixClient,
    payload: WritableDeviceWithConfigContextRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating device in NetBox...");

    match dcim_devices_create(client, payload) {
        Ok(response) => match response {
            DcimDevicesCreateResponse::Http201(created_device) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Device creation successful! New Device-ID: '{}'.",
                    created_device.id
                );
                Ok(created_device.id)
            }
            DcimDevicesCreateResponse::Other(other_response) => {
                panic!(
                    "Unexpected response code '{}' when trying to create a device!",
                    other_response.status()
                );
            }
        },
        Err(err) => {
            let exc: NetBoxApiError = NetBoxApiError::Reqwest(err);
            Err(exc)
        }
    }
}

/// Updates a device with a given ID.
/// Returns the ID of the updated device.
/// Will simply overwrite the given device object in NetBox with the collected information.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The payload for the API request.
/// - `id`: The ID of the device to update.
///
/// # Panics
///
/// This function may panic if NetBox doesn't return a `200` response code.
pub fn update_device(
    client: &ThanixClient,
    payload: PatchedWritableDeviceWithConfigContextRequest,
    id: i64,
) -> Result<i64, NetBoxApiError> {
    println!("Updating device in NetBox...");

    match dcim_devices_partial_update(client, payload, id) {
        Ok(response) => match response {
            DcimDevicesPartialUpdateResponse::Http200(updated_device) => {
                println!("\x1b[32m[success]\x1b[0m Device updated successfully!");
                Ok(updated_device.id)
            }
            DcimDevicesPartialUpdateResponse::Other(other_response) => {
                panic!(
                    "Unexpected response code '{:?}' when trying to update device!",
                    other_response.text()
                );
            }
        },
        Err(err) => {
            let exc: NetBoxApiError = NetBoxApiError::Reqwest(err);
            Err(exc)
        }
    }
}

/// Send request to create a new device in NetBox.
/// Returns the ID of the newly created Device object.
///
/// # Parameters
/// - `client: &ThanixClient` - The [`ThanixClient`] instance to use for communication.
/// - `payload: &WritableDeviceWithConfigContextRequest` - The information about the device serving as a request body.
///
/// # Panics
///
/// This function panics if the response code is not `201`.
pub fn create_vm(
    client: &ThanixClient,
    payload: WritableVirtualMachineWithConfigContextRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating virtual machine in NetBox...");

    match virtualization_virtual_machines_create(client, payload) {
        Ok(response) => match response {
            VirtualizationVirtualMachinesCreateResponse::Http201(created_device) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Virtual Machine creation successful! New ID: '{}'.",
                    created_device.id
                );
                Ok(created_device.id)
            }
            VirtualizationVirtualMachinesCreateResponse::Other(other_response) => {
                panic!(
                    "Unexpected response code '{}' when trying to create a virtual machine: {:?}!",
                    other_response.status(),
                    other_response.text()
                );
            }
        },
        Err(err) => {
            let exc: NetBoxApiError = NetBoxApiError::Reqwest(err);
            Err(exc)
        }
    }
}

/// Updates a device with a given ID.
/// Returns the ID of the updated device.
/// Will simply overwrite the given device object in NetBox with the collected information.
///
/// # Parameters
/// - `client`: The API client instance to use.
/// - `payload`: The payload for the API request.
/// - `id`: The ID of the device to update.
///
/// # Panics
///
/// This function may panic if NetBox doesn't return a `200` response code.
pub fn update_vm(
    client: &ThanixClient,
    payload: PatchedWritableVirtualMachineWithConfigContextRequest,
    id: i64,
) -> Result<i64, NetBoxApiError> {
    println!("Updating Virtual machine in NetBox...");

    match virtualization_virtual_machines_partial_update(client, payload, id) {
        Ok(response) => match response {
            VirtualizationVirtualMachinesPartialUpdateResponse::Http200(updated_device) => {
                println!("\x1b[32m[success]\x1b[0m Device updated successfully!");
                Ok(updated_device.id)
            }
            VirtualizationVirtualMachinesPartialUpdateResponse::Other(other_response) => {
                panic!(
                    "Unexpected response code '{}' when trying to update device!",
                    other_response.status()
                );
            }
        },
        Err(err) => {
            let exc: NetBoxApiError = NetBoxApiError::Reqwest(err);
            Err(exc)
        }
    }
}

/// Searches for a given MAC address object.
///
/// # Parameters
/// - `client: &ThanixClient` - The API client instance to use.
/// - `mac_address: &str` - The MAC address to search for.
///
/// # Returns
///
/// - `Option<i64>` - If it is found, will return the ID of the MAC address object in NetBox, else will return `None`.
///
/// # Panics
///
/// This function panics if the API request fails.
pub fn search_mac_address(client: &ThanixClient, mac_address: &str) -> Option<i64> {
    println!("Searching for mac address...");

    let mut payload = DcimMacAddressesListQuery::default();
    payload.mac_address__ic = Some(vec![mac_address.to_string()]);

    match dcim_mac_addresses_list(client, payload) {
        Ok(response) => match response {
            DcimMacAddressesListResponse::Http200(mut mac_addresses) => {
                if mac_addresses.results.as_mut().unwrap().len() == 1 {
                    let result = mac_addresses.results.unwrap();
                    return Some(result[0].id);
                }
                if mac_addresses.results?.is_empty() {
                    return None;
                }
                // FIXME: Remove this panic and swap with error.
                panic!("Ambiguous search result. MAC Address listed more then once.");
            }
            DcimMacAddressesListResponse::Other(other) => {
                panic!("{}", other.text().unwrap());
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

/// Creates new MAC address object.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `payload: MACAddressRequest` - The API request payload.
///
/// # Returns
///
/// - `Ok(i64)` - Returns the ID of the newly created MAC address.
/// - `Err(NetBoxAPIError)` - Returns an Error in case the request fails or get an unexpected
/// response.
pub fn create_mac_address(
    client: &ThanixClient,
    payload: MACAddressRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating MAC address in NetBox...");

    match dcim_mac_addresses_create(client, payload) {
        Ok(response) => match response {
            thanix_client::paths::DcimMacAddressesCreateResponse::Http201(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m MAC Address created successfully. New MAC Address-ID: '{}'",
                    result.id
                );
                Ok(result.id)
            }
            thanix_client::paths::DcimMacAddressesCreateResponse::Other(other_response) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other_response.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

/// Updates a MAC address object.
///
/// # Parameters
/// * `client: &ThanixClient` - The API client instance to use.
/// * `payload: MACAddressRequest` - The MAC address payload to update the MAC address with.
/// * `mac_address_id: i64` - The ID of the MAC address to update.
#[allow(unused)]
pub fn update_mac_address(
    client: &ThanixClient,
    payload: MACAddressRequest,
    mac_address_id: i64,
) -> Result<i64, NetBoxApiError> {
    match dcim_mac_addresses_update(client, payload, mac_address_id) {
        Ok(response) => match response {
            thanix_client::paths::DcimMacAddressesUpdateResponse::Http200(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m MAC Address '{}' updated successfully.",
                    result.id
                );
                Ok(result.id)
            }
            thanix_client::paths::DcimMacAddressesUpdateResponse::Other(other) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

/// Searches for interfaces with a given search parameters.
/// Returns the ID of the interface when it is found, else returns `None`
///
/// # Parameters
/// - `client: &ThanixClient`: The `ThanixClient` instance to use for communication.
/// - `device_id: i64`: The ID of the device this interface is linked to.
/// - `name: &String`: The name of this interface.
///
/// # Returns
///
/// `Some(i64)` as the ID of the interface object if found. If not, returns `None`.
///
/// # Panics
///
/// This function panics in these cases:
/// - The search results are inconclusive (interface is listed multiple times).
/// - The request returns an unexpected response code.
/// - The request fails (e.g the connection fails).
pub fn search_interface(client: &ThanixClient, device_id: i64, name: &String) -> Option<i64> {
    println!("Searching for interface '{name}'...");

    let payload: DcimInterfacesListQuery = DcimInterfacesListQuery {
        device_id: Some(vec![device_id]),
        name: Some(vec![name.clone()]),
        ..Default::default()
    };

    match dcim_interfaces_list(client, payload) {
        Ok(response) => match response {
            DcimInterfacesListResponse::Http200(mut interfaces) => {
                if interfaces.results.as_mut().unwrap().len() == 1 {
                    let result = interfaces.results.unwrap();
                    return Some(result[0].id);
                }
                if interfaces.results.unwrap().is_empty() {
                    return None;
                }
                panic!("Ambiguous search result. Interface listed more than once.");
            }
            DcimInterfacesListResponse::Other(res) => {
                panic!(
                    "Unexpected response code '{}' when trying to search for interface!",
                    res.status()
                );
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn search_vm_interface(client: &ThanixClient, vm_id: i64, name: &String) -> Option<i64> {
    println!("Searching for interface '{name}'...");

    let payload = VirtualizationInterfacesListQuery {
        virtual_machine_id: Some(vec![vm_id]),
        name: Some(vec![name.clone()]),
        ..Default::default()
    };

    match virtualization_interfaces_list(client, payload) {
        Ok(response) => match response {
            VirtualizationInterfacesListResponse::Http200(mut interfaces) => {
                if interfaces.results.as_mut().unwrap().len() == 1 {
                    let result = interfaces.results.unwrap();
                    return Some(result[0].id);
                }
                if interfaces.results.unwrap().is_empty() {
                    return None;
                }
                panic!("Ambiguous search result. Interface listed more than once.");
            }
            VirtualizationInterfacesListResponse::Other(res) => {
                panic!(
                    "Unexpected response code '{}' when trying to search for interface!",
                    res.status()
                );
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

/// Creates an interface object in NetBox.
/// Returns the ID of the interface object.
///
/// # Parameters
/// - `client: &ThanixClient` - The client instance necessary for communication.
/// - `payload: WritableInterfaceRequest` - The payload for the API request.
///
/// # Returns
/// - `Ok(i64)` - ID of the interface, If the creation was successful.
/// - `Err(NetBoxApiError)` - If the creation was unsuccessful or the request itself failed.
pub fn create_interface(
    client: &ThanixClient,
    payload: WritableInterfaceRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating network interface in NetBox...");

    match dcim_interfaces_create(client, payload) {
        Ok(response) => match response {
            thanix_client::paths::DcimInterfacesCreateResponse::Http201(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Interface created successfully. New Interface-ID: '{}'",
                    result.id
                );
                Ok(result.id)
            }
            thanix_client::paths::DcimInterfacesCreateResponse::Other(other_response) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other_response.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

pub fn create_vm_interface(
    client: &ThanixClient,
    payload: WritableVMInterfaceRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating network interface in NetBox...");

    match virtualization_interfaces_create(client, payload) {
        Ok(response) => match response {
            VirtualizationInterfacesCreateResponse::Http201(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Interface created successfully. New Interface-ID: '{}'",
                    result.id
                );
                Ok(result.id)
            }
            VirtualizationInterfacesCreateResponse::Other(other_response) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other_response.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

/// Updates a given interface object.
/// Returns the ID of the updated interface.
///
/// # Parameters
/// - `client: &ThanixClient` - The API client instance to use.
/// - `payload: WritableInterfaceRequest` - The API request payload to use.
/// - `interface_id: i64` - The ID of the interface to update.
///
/// # Returns
/// - `Ok(i64)` - ID of the updated interface if the update was successful.
/// - `Err(NetBoxApiError)` - If the update or request has failed.
pub fn update_interface(
    client: &ThanixClient,
    payload: WritableInterfaceRequest,
    interface_id: i64,
) -> Result<i64, NetBoxApiError> {
    match dcim_interfaces_update(client, payload, interface_id) {
        Ok(response) => match response {
            thanix_client::paths::DcimInterfacesUpdateResponse::Http200(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Interface '{}' updated successfully.",
                    result.id
                );
                Ok(result.id)
            }
            thanix_client::paths::DcimInterfacesUpdateResponse::Other(other) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

pub fn update_vm_interface(
    client: &ThanixClient,
    payload: WritableVMInterfaceRequest,
    interface_id: i64,
) -> Result<i64, NetBoxApiError> {
    match virtualization_interfaces_update(client, payload, interface_id) {
        Ok(response) => match response {
            VirtualizationInterfacesUpdateResponse::Http200(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m Interface '{}' updated successfully.",
                    result.id
                );
                Ok(result.id)
            }
            VirtualizationInterfacesUpdateResponse::Other(other) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

/// Search given IP Address.
///
/// # Parameters
/// * `client: &ThanixClient` - The API client instance to use.
/// * `address: &String` - The address to search for.
/// * `device_id: Option<i64>` - The ID of the device this address is linked to, if any.
///
/// # Returns
/// * `Option<i64>` - The ID of the IP address if it was found, `None` if it wasn't found.
///
/// # Panics
///
/// This function panics if the search result is ambiguous or an unexpected response code is
/// received.
pub fn search_ip(client: &ThanixClient, address: &String, device_id: Option<i64>) -> Option<i64> {
    println!("Searching for IP Address '{address}'...");
    let payload: IpamIpAddressesListQuery = IpamIpAddressesListQuery {
        address: Some(vec![address.clone()]),
        device_id: device_id.map(|x| vec![x]),
        ..Default::default()
    };

    // FIXME: Switch from panicking to returning a NetBoxApiError as is done everywhere else.
    // No panic should be used in cases the user can fix the problem.
    match ipam_ip_addresses_list(client, payload).unwrap() {
        IpamIpAddressesListResponse::Http200(addresses) => {
            if addresses.results.as_ref()?.len() == 1 {
                return Some(addresses.results?.first()?.id);
            }
            if addresses.results?.is_empty() {
                return None;
            }
            panic!(
                "Ambiguous search result. IP address listed more than once. Please check your data."
            );
        }
        IpamIpAddressesListResponse::Other(res) => {
            panic!(
                "Unexpected response code '{}' when trying to search for IP addresses!",
                res.status()
            );
        }
    }
}

pub fn search_vm_ip(client: &ThanixClient, address: &String, vm_ip: Option<i64>) -> Option<i64> {
    println!("Searching for IP Address '{address}'...");
    let payload: IpamIpAddressesListQuery = IpamIpAddressesListQuery {
        address: Some(vec![address.clone()]),
        virtual_machine_id: vm_ip.map(|x| vec![x]),
        ..Default::default()
    };

    // FIXME: Switch from panicking to returning a NetBoxApiError as is done everywhere else.
    // No panic should be used in cases the user can fix the problem.
    match ipam_ip_addresses_list(client, payload).unwrap() {
        IpamIpAddressesListResponse::Http200(addresses) => {
            if addresses.results.as_ref()?.len() == 1 {
                return Some(addresses.results?.first()?.id);
            }
            if addresses.results?.is_empty() {
                return None;
            }
            panic!(
                "Ambiguous search result. IP address listed more than once. Please check your data."
            );
        }
        IpamIpAddressesListResponse::Other(res) => {
            panic!(
                "Unexpected response code '{}' when trying to search for IP addresses!",
                res.status()
            );
        }
    }
}

/// Creates new IP adress object.
/// Returns the ID of the new IPAddress object.
///
/// # Parameters
/// - `client: &ThanixClient` - The client instance necessary for communication.
/// - `payload: WritableIPAddressRequest` - The payload to send.
///
/// # Returns
/// - `Ok(i64)` - If the creation of the IP address was successful.
/// - `Err(NetBoxApiError)` - If the creation or request itself fail.
pub fn create_ip(
    client: &ThanixClient,
    payload: WritableIPAddressRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating new IP address object...");

    match ipam_ip_addresses_create(client, payload) {
        Ok(response) => match response {
            thanix_client::paths::IpamIpAddressesCreateResponse::Http201(result) => {
                println!(
                    "\x1b[32m[success]\x1b[0m IP Address created successfully. New IP-ID: '{}'",
                    result.id
                );
                Ok(result.id)
            }
            thanix_client::paths::IpamIpAddressesCreateResponse::Other(other_response) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other_response.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            eprintln!(
                "\x1b[33m[warning]\x1b[0m Error while decoding NetBox response while creating IP address. This probably is still fine and a problem with NetBox.\nError: {e}"
            );
            let exc = NetBoxApiError::Other(e.to_string());
            Err(exc)
        }
    }
}

/// Patches a given IP address object.
/// Returns the ID of the updated object.
///
/// # Parameters
/// - `client: &ThanixClient` - The API client instance to use.
/// - `payload: PatchedWritableIPAddressRequest`: The API call payload.
/// - `id: i64`: The ID of the IP Address to update.
///
/// # Returns
///
/// - `Ok(i64)` - The ID of the patched IP address object, if successful.
/// - `Err(NetBoxApiError)` - If the request fails or an unexpected response is received.
pub fn patch_ip(
    client: &ThanixClient,
    payload: PatchedWritableIPAddressRequest,
    id: i64,
) -> Result<i64, NetBoxApiError> {
    println!("Patching IPs for given interface...");

    match ipam_ip_addresses_partial_update(client, payload, id) {
        Ok(response) => match response {
            IpamIpAddressesPartialUpdateResponse::Http200(result) => Ok(result.id),
            IpamIpAddressesPartialUpdateResponse::Other(other) => {
                Err(NetBoxApiError::Other(other.text().unwrap()))
            }
        },
        Err(e) => Err(NetBoxApiError::Reqwest(e)),
    }
}

/// Gets an interface by ID.
///
/// - `state`: The API client instance to use.
/// - `payload`: The payload to use.
#[allow(unused)]
pub fn get_interface(state: &ThanixClient, id: i64) -> Result<Interface, NetBoxApiError> {
    println!("Trying to get interface '{}'...", &id);

    match dcim_interfaces_retrieve(state, id) {
        Ok(response) => {
            let interface: Interface = match response {
                thanix_client::paths::DcimInterfacesRetrieveResponse::Http200(interface) => {
                    interface
                }
                thanix_client::paths::DcimInterfacesRetrieveResponse::Other(response) => {
                    let err: NetBoxApiError = NetBoxApiError::Other(response.text().unwrap());
                    return Err(err);
                }
            };
            Ok(interface)
        }
        Err(e) => {
            let err: NetBoxApiError = NetBoxApiError::Reqwest(e);
            Err(err)
        }
    }
}

/// Gets a list of Interfaces.
///
/// - `state`: The API client instance to use.
#[allow(unused)]
pub fn get_interface_list(state: &ThanixClient) -> Result<Option<Vec<Interface>>, NetBoxApiError> {
    println!("Retrieving list of interfaces...");

    match dcim_interfaces_list(state, DcimInterfacesListQuery::default()) {
        Ok(response) => {
            let interfaces = match response {
                thanix_client::paths::DcimInterfacesListResponse::Http200(interfaces) => {
                    interfaces.results
                }
                thanix_client::paths::DcimInterfacesListResponse::Other(other) => {
                    let err: NetBoxApiError = NetBoxApiError::Other(other.text().unwrap());
                    return Err(err);
                }
            };
            Ok(interfaces)
        }
        Err(e) => {
            let err: NetBoxApiError = NetBoxApiError::Reqwest(e);
            Err(err)
        }
    }
}

/// Attempts to retrieve an interface by its name.
///
/// - `state`: The API client instance to use.
/// - `payload`: The payload to send.
#[allow(unused)]
pub fn get_interface_by_name(
    state: &ThanixClient,
    payload: &WritableInterfaceRequest,
) -> Result<Interface, NetBoxApiError> {
    println!(
        "Trying to retrieve interface by name '{}'...",
        &payload.name
    );

    match dcim_interfaces_list(state, DcimInterfacesListQuery::default()) {
        Ok(response) => {
            let interface_list = match response {
                thanix_client::paths::DcimInterfacesListResponse::Http200(interfaces) => {
                    interfaces.results.unwrap()
                }
                thanix_client::paths::DcimInterfacesListResponse::Other(response) => {
                    let err: NetBoxApiError = NetBoxApiError::Other(response.text().unwrap());
                    return Err(err);
                }
            };

            for interface in interface_list {
                if interface.name == Some(payload.clone().name) {
                    return Ok(interface);
                }
            }
            Err(NetBoxApiError::Other(format!(
                "No interface '{}' with name found. Creation possibly failed.",
                &payload.name
            )))
        }
        Err(e) => {
            let err: NetBoxApiError = NetBoxApiError::Reqwest(e);
            Err(err)
        }
    }
}
