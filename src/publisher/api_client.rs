//! # Client Module
//!
//! This module contains logic for the HTTP NetBox API client.
//!
//! This client allows us to get a list of all machines and create or update machines or VMs.
//! We use the `reqwest` crate for blocking HTTP requests and `serde` together with `serde_json` to serialize and
//! deserialize our data.
extern crate thanix_client;

use reqwest::{blocking::Client, Error as ReqwestError};
use thanix_client::{
    paths::{dcim_devices_create, dcim_interfaces_create, DcimDevicesCreateResponse},
    types::{WritableDeviceWithConfigContextRequest, WritableInterfaceRequest},
    util::ThanixClient,
};

use super::{
    publisher,
    publisher_exceptions::{self, NetBoxApiError},
};

/// Tests connection to the NetBox API.
///
/// This method attempts to retrieve the API root from the NetBox API to affirm that it is reachable.
///
/// # Returns
///
/// Returns `Ok(())` if the connection to the API is successful.
/// Returns an `Err` with `publisher_exceptions::NetBoxApiError` if the connection fails.
pub fn test_connection(client: &ThanixClient) -> Result<(), publisher_exceptions::NetBoxApiError> {
    let url: String = format!("{}/api/", client.base_url);

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
                Ok(())
            } else {
                Err(publisher_exceptions::NetBoxApiError::Reqwest(
                    resp.error_for_status().unwrap_err(),
                ))
            }
        }
        Err(e) => Err(publisher_exceptions::NetBoxApiError::Reqwest(e)),
    }
}

/// Send request to create a new device in NetBox.
///
/// # Parameters
///
/// * client: `&ThanixClient` - The `ThanixClient` instance to use for communication.
/// * payload: `&WritableDeviceWithConfigContextRequest` - The information about the device serving
/// as a request body.
///
pub fn create_device(
    client: &ThanixClient,
    payload: &WritableDeviceWithConfigContextRequest,
) -> Result<i64, NetBoxApiError> {
    println!("Creating Device in NetBox...");

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
    println!("Creating interface in NetBox...");

    match dcim_interfaces_create(client, payload) {
        Ok(response) => {
            match response {
                thanix_client::paths::DcimInterfacesCreateResponse::Http201(result) => {
                    println!("\x1b[32m[success]\x1b[0m Interface created successfully. New Interface ID: {}", result.id);
                    Ok(result.id)
                }
                thanix_client::paths::DcimInterfacesCreateResponse::Other(other_response) => {
                    let exc: NetBoxApiError = NetBoxApiError::Other(other_response.text().unwrap());
                    Err(exc)
                }
            }
        }
        Err(e) => {
            panic!("[FATAL] NetBox connection failed. {}", e);
        }
    }
}
