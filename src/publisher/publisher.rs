//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The `api_client` module will provide the actual client and request logic.
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use thanix_client::{
    paths::{self, DcimDevicesListQuery},
    types::{DeviceWithConfigContext, PaginatedDeviceWithConfigContextList},
    util::ThanixClient,
};

use crate::publisher::api_client::test_connection;

use super::publisher_exceptions::NetBoxApiError;

pub fn probe(client: &ThanixClient) -> Result<(), NetBoxApiError> {
    println!("Probing connection to NetBox...");

    match test_connection(&client) {
        Ok(()) => {
            println!("\x1b[32mConnection established!\x1b[0m");
            Ok(())
        }
        Err(err) => panic!("Client unable to reach NetBox! {}", err),
    }
}

pub fn register_machine(client: &ThanixClient) -> Result<(), NetBoxApiError> {
    println!("Starting registration process. This may take a while...");
    get_machines(client);
    Ok(())
}

/// Get list of machines.
///
/// Sends a `GET` request to the `/dcim/devices` endpoint to retrieve
/// a list of machines.
///
/// This is later needed to search for the current machine in the response to decide
/// whether to register a new one or update an existing one.
///
/// # Arguments
///
/// - `client: &ThanixClient` - Instance of the current API client.
///
/// # Returns
///
/// # Panics
///
/// The function panics, when the request returns an error.
fn get_machines(client: &ThanixClient) {
    println!("Retrieving list of machines...");

    match paths::dcim_devices_list(client, paths::DcimDevicesListQuery::default()) {
        Ok(response) => {
            println!("List received. Analyzing...");
            let debug_json = response.text().unwrap();

            // Write the JSON string into a file
            std::fs::write("output.txt", &debug_json).unwrap();
            //println!("{:?}", &debug_json);
            let response_text: PaginatedDeviceWithConfigContextList =
                serde_json::from_str(&debug_json).unwrap();

            // Convert the Rust object back into a JSON string
            let json_string = serde_json::to_string_pretty(&response_text).unwrap();

            // println!("Response \n -------\n\t{:?}", &json_string)
        }
        Err(err) => panic!("{}", err),
    }
}

/// Searches for matching device in list of machines.
///
/// # Returns
///
/// - `bool` - Depending on if the device has been found or not.
fn search_for_matches(list: &Vec<DeviceWithConfigContext>) -> bool {
    true
}

/// Determine Error code based on response.
fn determine_resp_code() {}
