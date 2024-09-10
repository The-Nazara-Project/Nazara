//! # Client Module
//!
//! This module contains logic for the HTTP NetBox API client.
//!
//! This client allows us to get a list of all machines and create or update machines or VMs.
//! We use the `reqwest` crate for blocking HTTP requests and `serde` together with `serde_json` to serialize and
//! deserialize our data.
extern crate thanix_client;

use reqwest::Error as ReqwestError;
use serde_json::Value;
use thanix_client::{
    paths::{
        dcim_devices_create, dcim_interfaces_create, dcim_interfaces_list,
        ipam_ip_addresses_create, DcimDevicesCreateResponse, DcimInterfacesListQuery,
    },
    types::{
        Interface, WritableDeviceWithConfigContextRequest, WritableIPAddressRequest,
        WritableInterfaceRequest,
    },
    util::ThanixClient,
};

use super::publisher_exceptions::{self, NetBoxApiError};

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
pub fn test_connection(client: &ThanixClient) -> Result<(), publisher_exceptions::NetBoxApiError> {
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
                let json: Value = resp
                    .json::<Value>()
                    .map_err(|e| NetBoxApiError::Reqwest(e))?;

                if let Some(netbox_ver) = json.get("netbox-version").and_then(Value::as_str) {
                    // Compare netbox version for compatibility
                    if check_version_compatiblity(netbox_ver, thanix_client::version::VERSION) {
                        println!("\x1b[32m[success]\x1b[0m API client version compatible with NetBox version.");
                        Ok(())
                    } else {
                        Err(publisher_exceptions::NetBoxApiError::VersionMismatch(String::from("Client version incompatible with NetBox version! Use client v1.x for NetBox v3.6.x and above, and v2.x for NetBox 4.x.")))
                    }
                } else {
                    Err(publisher_exceptions::NetBoxApiError::MissingVersion(
                        String::from(
                            "NetBox version missing from response. Please check your installation.",
                        ),
                    ))
                }
            } else {
                Err(publisher_exceptions::NetBoxApiError::Reqwest(
                    resp.error_for_status().unwrap_err(),
                ))
            }
        }
        Err(e) => Err(publisher_exceptions::NetBoxApiError::Reqwest(e)),
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
/// `bool` depending if the `thanix_client` dependency is compatible with the running NetBox
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

fn get_major_verison(version: &str) -> Option<u32> {
    version.split('.').next()?.parse().ok()
}

/// Send request to create a new device in NetBox.
///
/// # Parameters
///
/// * `client: &ThanixClient` - The `ThanixClient` instance to use for communication.
/// * `payload: &WritableDeviceWithConfigContextRequest` - The information about the device serving
/// as a request body.
///
pub fn create_device(
    client: &ThanixClient,
    payload: &WritableDeviceWithConfigContextRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating device in NetBox...");

    match dcim_devices_create(client, payload.clone()) {
        Ok(response) => {
            match response {
                DcimDevicesCreateResponse::Http201(created_device) => {
                    println!(
                        "\x1b[32m[success]\x1b[0m Device creation successful! New Device-ID: '{}'.",
                        created_device.id
                    );
                    Ok(created_device.id)
                }
                DcimDevicesCreateResponse::Other(other_response) => {
                    // TODO
                    todo!(
                        "Other Response codes from creation not yet implemented! {}",
                        other_response.text().unwrap()
                    );
                }
            }
        }
        Err(err) => {
            panic!("{}", err); // Handle this better
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
///
/// # Panics
///
/// Panics if NetBox become unreachable.
pub fn create_interface(
    client: &ThanixClient,
    payload: WritableInterfaceRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating network interface in NetBox...");

    match dcim_interfaces_create(client, payload) {
        Ok(response) => match response {
            thanix_client::paths::DcimInterfacesCreateResponse::Http201(result) => {
                println!("\x1b[32m[success]\x1b[0m Interface created successfully. New Interface-ID: '{}'", result.id);
                Ok(result.id)
            }
            thanix_client::paths::DcimInterfacesCreateResponse::Other(other_response) => {
                let exc: NetBoxApiError = NetBoxApiError::Other(other_response.text().unwrap());
                Err(exc)
            }
        },
        Err(e) => {
            eprintln!("\x1b[33m[warning]\x1b[0m Error while decoding NetBox Response while creating network interface. This is probably still fine and a problem with NetBox.\nError: {}", e);
            let exc = NetBoxApiError::Other(e.to_string());
            Err(exc)
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
            eprintln!("\x1b[33m[warning]\x1b[0m Error while decoding NetBox response while creating IP address. This probably is still fine and a problem with NetBox.\nError: {}", e);
            let exc = NetBoxApiError::Other(e.to_string());
            Err(exc)
        }
    }
}

/// Attempt to retrieve an interface by its name.
///
/// # Parameters
///
/// * `state: &ThanixClient` - The API client instance to use.
/// * `payload: &WritableInterfaceRequest` - The device to create.
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
        payload.name.as_ref().unwrap()
    );

    match dcim_interfaces_list(state, DcimInterfacesListQuery::default()) {
        Ok(response) => {
            let interface_list: Vec<Interface> = match response {
                thanix_client::paths::DcimInterfacesListResponse::Http200(interfaces) => {
                    interfaces.results.unwrap()
                }
                thanix_client::paths::DcimInterfacesListResponse::Other(response) => {
                    let err: NetBoxApiError = NetBoxApiError::Other(response.text().unwrap());
                    return Err(err);
                }
            };

            for interface in interface_list {
                if interface.name == payload.name {
                    return Ok(interface);
                }
            }
            Err(NetBoxApiError::Other(format!(
                "No Inteface '{}' with name found. Creation possibly failed.",
                payload.name.as_ref().unwrap()
            )))
        }
        Err(e) => {
            let err: NetBoxApiError = NetBoxApiError::Reqwest(e);
            Err(err)
        }
    }
}
