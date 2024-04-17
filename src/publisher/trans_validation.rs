//! # Translator Validation Module
//!
//! This module is responsible for validating all system parameters before
//! a creation or update request is sent to NetBox.
//!
//! This includes:
//!
//! * Confirming certain API objects (like IP Adresses) are in fact present in NetBox.
//! * Validating that the Names of certain API objects are valid and correspond to an ID.
//!
//! > Also, given the filename here is room for a statement:
//! > Trans rights are human rights!

use thanix_client::{types::WritableDeviceWithConfigContextRequest, util::ThanixClient};

use crate::{configuration::config_parser::ConfigData, Machine};

/// Validate the `Device` Payload.
///
/// # Parameters
/// * payload: `WritableDeviceWithConfigContextRequest` - The struct to validate.
///
/// # Returns
///
/// - Ok(())
fn validate_device_payload(payload: WritableDeviceWithConfigContextRequest) -> bool {
    println!("Validating device payload...");

    todo!("Device payload validation not implemented yet!");

    false
}

pub fn get_ip_adresses(state: &ThanixClient, config_data: &ConfigData) {
    todo!("getting ip adresses is still todo");
}

// Create a new IP-Adress object in NetBox if the collected IP Adresses for the preferred interface
// do not exist yet.
//
// # Parameters
//
// * state: `&ThanixClient` - The `ThanixClient` object used for API connection.
// * config_data: `&ConfigData` - The config information which identifies the preferred network
// interface.
// * sys_info: `&Machine` - Collected system information which contains the IP Adresses to create.
//
// # Returns
//
// * Ok()
//
// # Panics
//
// This function panics if the creation if the IP Adresses return an error code other than "Ok".
fn create_ip_adresses(state: &ThanixClient, config_data: &ConfigData, sys_info: &Machine) {}
