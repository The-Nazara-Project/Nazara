//! # Client Module
//!
//! This module contains logic for the HTTP NetBox API client.
//!
//! This client allows us to get a list of all machines and create or update machines or VMs.
//! We use the `reqwest` crate for blocking HTTP calles and `serde` together with `serde_json` to serialize and
//! deserialize our data.
use reqwest::{blocking::Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::collectors::{dmi_collector::DmiInformation, network_collector::NetworkInformation};

use super::publisher_exceptions;

/// NetBox API client.
///
/// This client encapsulates the base URL and API token necessary for authenticating with the NetBox API
/// as well as a `reqwest` client for making HTTP requests.
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
    pub system_location: String,
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

impl NetBoxClient {
    /// Constructs a new `NetBoxClient` with the given base URL and API token.
    ///
    /// This constructor initializes the internal reqwest client and stores the provided base URL or API token read
    /// from the command line or configuration file.
    ///
    /// # Arguments
    ///
    /// * `base_url: &str` - A string slice that holds the base URL of the NetBox API.
    /// * `api_token: &str` - A string slice that holds the API token for authenticating with the NetBox API
    ///
    /// # Returns
    ///
    /// A new instance of the `NetBoxClient`
    pub fn new(base_url: &str, api_token: &str) -> Self {
        let client: Client = Client::new();
        NetBoxClient {
            base_url: base_url.to_string(),
            api_token: api_token.to_string(),
            client,
        }
    }

    /// Tests connection to the NetBox API.
    ///
    /// This method attempts to retrieve the API root from the NetBox API to affirm that it is reachable.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the connection to the API is successful.
    /// Returns an `Err` with `publisher_exceptions::NetBoxApiError` if the connection fails.
    pub fn test_connection(&self) -> Result<(), publisher_exceptions::NetBoxApiError> {
        let url: String = format!("{}/api/", self.base_url);

        let response: Result<reqwest::blocking::Response, ReqwestError> = self
            .client
            .get(&url)
            .header("Authorization", format!("Token {}", self.api_token))
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

    /// Sends a request to the NetBox API to create a new machine.
    ///
    /// This method posts the provided payload to the machine creation endpoint of the API.
    /// It handles the serialization of the payload to JSON format, sets the necessary HTTP headers for authentication,
    /// and checks the API response to determine if the request was successful.
    ///
    /// # Arguments
    ///
    /// * `payload: &CreateMachinePayload` - A reference to a `CreateMachinePayload` instance containing all necessary payload information.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the machine was created successfully.
    /// Returns an `Err` with `publisher_exceptions::NetBoxApiError` if the request failed.
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
    }

    // TODO: Implement function to get and update machines and VMs.
    pub fn get_machines(&self) {
        todo!()
    }

    pub fn update_machine(&self) -> Result<(), publisher_exceptions::NetBoxApiError> {
        todo!()
    }
}
