//! # Client Module
//!
//! This module contains logic for the HTTP NetBox API client.
//!
//! This client allows us to get a list of all machines and create or update machines or VMs.
//! We use the `reqwest` crate for blocking HTTP requests and `serde` together with `serde_json` to serialize and
//! deserialize our data.
extern crate thanix_client;

use reqwest::{blocking::Client, Error as ReqwestError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use thanix_client::util::ThanixClient;

use crate::collectors::{dmi_collector::DmiInformation, network_collector::NetworkInformation};

use super::{publisher, publisher_exceptions};

/// Represents the combined system information required to create a new machine or update an existing one.
///
/// # Members
///
/// * `dmi_information: DmiInformation` - DMI (Desktop Management Interface) details of the system such as model and manufacturer.
/// * `network_information: NetworkInformation` - A list of network interface details.
/// * `system_location: String` A string representing the location or office the system is located at. Read from the config file.
#[derive(Serialize)]
pub struct SystemData {
    pub dmi_information: DmiInformation,
    pub network_information: Vec<NetworkInformation>,
    pub system_name: String,
    pub system_location: String,
    pub device_role: String,
}

/// Encapsulates the payload required to create a new machine in NetBox.
///
/// This struct is serialized into JSON format and sent as the body of the HTTP request to create a new machine.
///
/// # Members
///
/// * `system_information: SystemData` - System Information to include in the request.
#[derive(Serialize)]
pub struct CreateMachinePayload {
    pub system_information: SystemData,
}

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
