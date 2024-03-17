//! # Client Module
//!
//! This module contains logic for the HTTP NetBox API client.
//!
//! This client allows us to get a list of all machines and create or update machines or VMs.
//! We use the `reqwest` crate for blocking HTTP requests and `serde` together with `serde_json` to serialize and
//! deserialize our data.
extern crate thanix_client;

use reqwest::{blocking::Client, Error as ReqwestError};
use thanix_client::{paths::{dcim_devices_create, DcimDevicesCreateResponse}, types::WritableDeviceWithConfigContextRequest, util::ThanixClient};

use super::{publisher, publisher_exceptions};

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

pub fn create_device(client: &ThanixClient, payload: &WritableDeviceWithConfigContextRequest) {
    println!("Creating Device in NetBox...");

    match dcim_devices_create(client, payload.clone()) {
        Ok(response) => {
            match response {
                DcimDevicesCreateResponse::Http201(created_device) => {
                    // TODO
                    println!(
                    "[success] Device creation successful!\nYour machine can be found under the ID {}.", created_device.id
                );
                }
                DcimDevicesCreateResponse::Other(other_response) => {
                    // TODO
                    todo!("Other Response codes from creation not yet implemented! {}", other_response.text().unwrap());
                }
            }
        }
        Err(err) => {
            panic!("{}", err); // Handle this better
        }
    }
}
