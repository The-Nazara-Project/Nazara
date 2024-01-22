//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The `api_client` module will provide the actual client and request logic.
use serde::{Deserialize, Serialize};
use thanix_client::{paths::{self, DcimDevicesListQuery}, types::PaginatedDeviceWithConfigContextList, util::ThanixClient};

use crate::publisher::api_client::test_connection;

use super::publisher_exceptions::NetBoxApiError;

pub fn probe(client: &ThanixClient) -> Result<(), NetBoxApiError> {
    println!("Probing connection to NetBox...");

    match test_connection(&client) {
        Ok(()) => {
            println!("\x1b[32mConnection established!\x1b[0m");
            Ok(())
        },
        Err(err) => panic!("Client unable to reach NetBox! {}", err)
    }
}

pub fn register_machine(client: &ThanixClient) -> Result<(), NetBoxApiError> {
    println!("Registering your machine. This may take a while...");
    get_machines(client);
    Ok(())
}

/// Get list of machines
fn get_machines(client: &ThanixClient) {
    println!("Retrieving list of machines...");

    match paths::dcim_devices_list(client, paths::DcimDevicesListQuery::default()) {
        Ok(response) => {
            dbg!(response.json::<String>());
            let response_text: PaginatedDeviceWithConfigContextList = response.json().unwrap();

            // Convert the Rust object back into a JSON string
            let json_string = serde_json::to_string_pretty(&response_text).unwrap();

            // Write the JSON string into a file
            std::fs::write("output.json", &json_string).unwrap();

            println!("Response \n -------\n\t{:?}", &json_string)
        },
        Err(err) => panic!("{}", err)
    }
}

/// Determine Error code based on response.
fn determine_resp_code() {

}
