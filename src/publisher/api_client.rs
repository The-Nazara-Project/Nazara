//! # API Client Module
//!
//! This module's single responsibility is to take the payloads from the
//! [`publisher`](crate::publisher) module - which it created using the
//! [`translator`](crate::publisher::translator) module - and execute the individual API requests.
//!
//! Errors are escalated upwards.
extern crate thanix_client;

use super::error::{self, NetBoxApiError};
use reqwest::Error as ReqwestError;
use serde_json::Value;
use thanix_client::paths::{
    DcimMacAddressesListQuery, DcimMacAddressesListResponse, dcim_mac_addresses_create,
    dcim_mac_addresses_list, dcim_mac_addresses_update,
};
use thanix_client::types::MACAddressRequest;
use thanix_client::{
    paths::{
        DcimDevicesCreateResponse, DcimDevicesListQuery, DcimDevicesUpdateResponse,
        DcimInterfacesListQuery, DcimInterfacesListResponse, IpamIpAddressesListQuery,
        IpamIpAddressesListResponse, dcim_devices_create, dcim_devices_list, dcim_devices_update,
        dcim_interfaces_create, dcim_interfaces_list, dcim_interfaces_retrieve,
        dcim_interfaces_update, ipam_ip_addresses_create, ipam_ip_addresses_list,
        ipam_ip_addresses_update,
    },
    types::{
        Interface, WritableDeviceWithConfigContextRequest, WritableIPAddressRequest,
        WritableInterfaceRequest,
    },
    util::ThanixClient,
};

/// Tests connection to the NetBox API and verifies if your `thanix_client` version is compatible
/// with your NetBox version.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The client instance to be used for communication.
///
/// # Returns
///
/// Returns `Ok(())` if the connection to the API is successful and the `thanix_client` version is
/// compatible with the used NetBox version.
/// Returns an `Err` with `publisher_exceptions::NetBoxApiError` if the connection fails or the
/// `thanix_client``version is not compatible with your NetBox version.
pub fn test_connection(client: &ThanixClient) -> Result<(), error::NetBoxApiError> {
    let url: String = format!("{}/api/status/", client.base_url);

    let response: Result<reqwest::blocking::Response, ReqwestError> = client
        .client
        .get(&url)
        .header(
            "Authorization",
            format!("Token {}", client.authentication_token),
        )
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: Value = resp.json::<Value>().map_err(NetBoxApiError::Reqwest)?;

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
                    resp.error_for_status().unwrap_err(),
                ))
            }
        }
        Err(e) => Err(error::NetBoxApiError::Reqwest(e)),
    }
}

/// Compare the NetBox version with the thanix_client version for compatibility.
///
/// Given the drastic differences in NetBox's API between `v3.x` and `v4.x` there are two different
/// release tracks for these two versions.
///
/// Version `v1.x` is compatible for NetBox Version `v3.6.x` and above, while `thanix_client`
/// version `v2.x` will be compatible with NetBox version `v4.x` and above.
///
/// # Parameters
///
/// * `netbox_version: &str` - The version of the NetBox instance extracted from the response.
/// * `thanix_version: &str` - The version of the installed `thanix_client` dependency.
///
/// # Returns
///
/// * `bool` depending if the `thanix_client` dependency is compatible with the running NetBox
/// version.
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

/// Get the major version from the given version String.
///
/// # Parameters
///
/// * `version: &str` - String representation of the application version.
///
/// # Returns
///
/// * `Some(u32)` if the Version can be parsed to an `u32`, if not, returns `None`.
fn get_major_verison(version: &str) -> Option<u32> {
    version.split('.').next()?.parse().ok()
}

/// Search a device by sending a `DcimDevicesListQuery` with given search parameters.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `name: String` - The name of the machine to search for.
/// * `serial: String` - The serial number of the machine.
///
/// # Returns
///
/// * `Option<i64>` - The ID of the device, if it exists, if not, returns `None`.
///
/// # Aborts
///
/// This function terminates the process under following conditions:
///
/// * Search results are indecisive. E.g more than one result.
/// * The API request returns an unexpected response code.
/// * The API request fails (e.g the connection fails).
pub fn search_device(client: &ThanixClient, name: &String, serial: &String) -> Option<i64> {
    println!("Checking if device is already registered...");
    let payload: DcimDevicesListQuery = DcimDevicesListQuery {
        name: Some(vec![name.clone()]),
        serial: Some(vec![serial.clone()]),
        ..Default::default()
    };

    match dcim_devices_list(client, payload) {
        Ok(response) => match response {
            thanix_client::paths::DcimDevicesListResponse::Http200(device_list) => {
                if device_list.results.as_ref()?.len() == 1 {
                    return Some(device_list.results?.get(0)?.id);
                }
                if device_list.results?.is_empty() {
                    return None;
                }
                panic!(
                    "Ambiguous search result. Device listed more than once. Please check your data."
                );
            }
            thanix_client::paths::DcimDevicesListResponse::Other(other) => {
                panic!("{}", other.text().unwrap());
            }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

/// Send request to create a new device in NetBox.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The `ThanixClient` instance to use for communication.
/// * `payload: &WritableDeviceWithConfigContextRequest` - The information about the device serving
/// as a request body.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the newly created Device object.
/// * `NetBoxApiError` - When the request returns an error.
///
/// # Aborts
///
/// This function terminates the process if the response code is not the expected `201`.
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

/// Update a device with a given ID.
///
/// Will simply overwrite the given `Device` object in NetBox with the collected information.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `payload: WritableDeviceWithConfigContextRequest` - The payload for the API request.
/// * `id: i64` - The ID of the device to update.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the updated device.
/// * `Err(NetBoxApiError)` - Returns a `NetBoxApiError` when the request fails.
///
/// # Aborts
///
/// This function may terminate the process if NetBox returns an unexpected non-200 response code.
pub fn update_device(
    client: &ThanixClient,
    payload: WritableDeviceWithConfigContextRequest,
    id: i64,
) -> Result<i64, NetBoxApiError> {
    println!("Updating device in NetBox...");

    match dcim_devices_update(client, payload, id) {
        Ok(response) => match response {
            DcimDevicesUpdateResponse::Http200(updated_device) => {
                println!("\x1b[32m[success]\x1b[0m Device updated successfully!");
                Ok(updated_device.id)
            }
            DcimDevicesUpdateResponse::Other(other_response) => {
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

/// Search for interfaces with a given search parameters.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The `ThanixClient` instance to use for communication.
/// * `device_id: &i64` - The ID of the device this interface is linked to.
/// * `name: &String` - The name of this interface.
///
/// # Returns
///
/// `Some(i64)` the ID of the interface when it is found, else returns `None`
///
/// # Aborts
///
/// This function aborts in these cases:
///
/// * The search results are inconclusive (interface is listed multiple times).
/// * The request returns an unexpected response code.
/// * The request fails (e.g the connection fails).
pub fn search_interface(client: &ThanixClient, device_id: &i64, name: &String) -> Option<i64> {
    println!("Searching for interface '{}'...", name);

    let payload: DcimInterfacesListQuery = DcimInterfacesListQuery {
        device_id: Some(vec![*device_id]),
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

/// Create an interface object in NetBox.
///
/// # Parameters
///
/// * client: `&ThanixClient` - The client instance necessary for communication.
/// * payload: `&WritableInterfaceRequest` - The payload for the API request.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the interface object.
/// * `Err(NetBoxApiError)` - Error will be passed back to `publisher`.
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

/// Update a given interface object.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `payload: WritableInterfaceRequest` - The API request payload to use.
/// * `interface_id: i64` - The ID of the interface to update.
///
/// # Returns
///
/// * `Ok(i64)` - Returns the ID of the updated interface.
/// * `Err(NetBoxApiError)` - If the connection fails or a unexpected response is returned.
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

pub fn search_ip(client: &ThanixClient, address: &String, device_id: &i64) -> Option<i64> {
    println!("Searching for IP Address '{}'...", address);
    let payload: IpamIpAddressesListQuery = IpamIpAddressesListQuery {
        address: Some(vec![address.clone()]),
        device_id: Some(vec![*device_id]),
        ..Default::default()
    };

    match ipam_ip_addresses_list(client, payload).unwrap() {
        IpamIpAddressesListResponse::Http200(addresses) => {
            if addresses.results.as_ref()?.len() == 1 {
                return Some(addresses.results?.get(0)?.id);
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

/// Create new IP adress object.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The client instance necessary for communication.
/// * `payload: WritableIPAddressRequest` - The payload to send.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the new IPAddress object.
/// * `Err(NetBoxApiError)` - A variant of `NetBoxAPiError` if the creation fails.
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
                "\x1b[33m[warning]\x1b[0m Error while decoding NetBox response while creating IP address. This probably is still fine and a problem with NetBox.\nError: {}",
                e
            );
            let exc = NetBoxApiError::Other(e.to_string());
            Err(exc)
        }
    }
}

/// Update a given IP address object.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The API client instance to use.
/// * `payload: WritableIPAddressRequest` - The API call payload.
/// * `id: i64` - The ID of the IP Address to update.
///
/// # Returns
///
/// * `Ok(i64)` - The ID of the updated object.
/// * `Err(NetBoxApiError)` - In case the connection fails or an unexpected response is returned.
pub fn update_ip(
    client: &ThanixClient,
    payload: WritableIPAddressRequest,
    id: i64,
) -> Result<i64, NetBoxApiError> {
    println!("Updating IPs for given interface...");

    match ipam_ip_addresses_update(client, payload, id) {
        Ok(response) => match response {
            thanix_client::paths::IpamIpAddressesUpdateResponse::Http200(result) => Ok(result.id),
            thanix_client::paths::IpamIpAddressesUpdateResponse::Other(other) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            let exc: NetBoxApiError = NetBoxApiError::Reqwest(e);
            Err(exc)
        }
    }
}

/// Get Interface by ID.
///
/// # Parameters
///
/// * `state: &ThanixClient` - The API client instance to use.
/// * `payload: &WritableInterfaceRequest` - The payload to use.
///
/// # Returns
///
/// * `Ok(Interface)` - The Interface we are looking for.
/// * `Err(NetBoxApiError)` - When the Interface does not exist.
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

/// Get List of Interfaces.
///
/// # Parameters
///
/// * `state: &ThanixClient` - The API client instance to use.
///
/// # Returns
///
/// * `Ok(Vec<Interface>)` - A list of Interface objects.
/// * `Err(NetBoxApiError)` - A `NetBoxApiError` when something goes wrong.
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

/// Attempt to retrieve an interface by its name.
///
/// # Parameters
///
/// * `state: &ThanixClient` - The API client instance to use.
/// * `payload: &WritableInterfaceRequest` - The payload to send.
///
/// # Returns
///
/// * `Ok(Interface)` - Returns the `Interface` if it exists.
/// * `Err(NetBoxApiError)` - Returns variant of `NetBoxApiError` if the interface does not exist.
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
