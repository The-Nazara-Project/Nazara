//! # Publisher Module
//!
//! This module contains logic to "steer" the API requests.
//! Based on the conditions in this logic it is decided whether to use the machine or VM endpoints, to create a new
//! machine or update an existing one.
//!
//! The `api_client` module will provide the actual client and request logic.
use reqwest::blocking::Client;
use thanix_client::util::ThanixClient;

use crate::publisher::api_client::{self, test_connection};

use super::publisher_exceptions::NetBoxApiError;

pub struct Publisher {}

// Create singleton
static mut _PUBLISHER: Publisher = Publisher::create();

impl Publisher {
    pub const fn create() -> Publisher {
        Publisher {}
    }

    pub fn probe(base_url: &str, auth_token: &str) -> () {
        println!("Probing connection to Netbox...");

        let client = create_client(base_url, auth_token);

        match test_connection(&client) {
            Ok(()) => println!("Connection established!"),
            Err(err) => panic!("Client unable to reach NetBox!"),
        }
    }
}

fn create_client(base_url: &str, auth_token: &str) -> ThanixClient {
    return ThanixClient {
        base_url: base_url.to_string(),
        authentication_token: auth_token.to_string(),
        client: Client::new(),
    };
}
