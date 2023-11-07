//! This module contains logic for the HTTP NetBox API client.

use reqwest::{blocking::Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::collectors::{dmi_collector::DmiInformation, network_collector::NetworkInformation};

use super::publisher_exceptions;

/// NetBox API client.
///
/// # Members
///
/// * base_url: `String` - The base url of your netbox instance. Read from the config file or command line. E.g: `https://netbox.company.de`
/// * api_token: `String` - The authentication token for the NetBox API.
/// * client: `reqwest::Client` - The Client object from the `reqwest` crate.
pub struct NetBoxClient {
    pub base_url: String,
    pub api_token: String,
    client: Client,
}

/// System Information.
#[derive(Serialize)]
pub struct SystemData {
    pub dmi_information: DmiInformation,
    pub network_information: Vec<NetworkInformation>,
    pub system_location: String,
}

/// Request Payload for machine creation.
#[derive(Serialize)]
pub struct CreateMachinePayload {
    pub system_information: SystemData,
}

impl NetBoxClient {
    pub fn new(base_url: &str, api_token: &str) -> Self {
        let client: Client = Client::new();
        NetBoxClient {
            base_url: base_url.to_string(),
            api_token: api_token.to_string(),
            client,
        }
    }

    /// Create machine in NetBox.
    pub fn create_machine(
        &self,
        payload: &CreateMachinePayload,
    ) -> Result<(), publisher_exceptions::NetBoxApiError> {
        // Construct the URL for creating a machine
        let url: String = format!("{}/api/dcim/devices/", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_token))
            .json(payload)
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

        // TODO: Implement other methods to interact with NetBox.
    }
}
