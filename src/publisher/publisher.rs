//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The `api_client` module will provide the actual client and request logic.
use super::{api_client::NetBoxClient, publisher_exceptions::NetBoxApiError};

pub struct Publisher {}

// Create singleton
static mut _PUBLISHER: Publisher = Publisher::create();

impl Publisher {
    pub const fn create() -> Publisher {
        Publisher {}
    }

    pub fn probe(base_url: &str, auth_token: &str) -> () {
        println!("Probing connection to Netbox...");

        match create_client(base_url, auth_token).test_connection() {
            Ok(()) => println!("Connection established!"),
            Err(err) => println!("{:?}", err),
        }
    }
}

fn create_client(base_url: &str, auth_token: &str) -> NetBoxClient {
    NetBoxClient::new(base_url, auth_token)
}
