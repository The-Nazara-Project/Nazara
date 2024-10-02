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

use thanix_client::types::WritableDeviceWithConfigContextRequest;


/// Validate the system information found in the config file.
///
/// Checks that parameters such as IDs and other system parameters entered in the config file
/// correspond to an existing NetBox object.
/// Otherwise, return an Error.
///
/// # Parameters
///
/// * state: `&ThanixClient` - The API client instance to use for communication.
/// * config_data: `&ConfigData` - The configuration file contents.
///
/// # Returns
///
/// Returns `Ok(())` when all relevant config parameters correspond to existing objects. Otherwise
/// return `Error`.
///
/// # Panics
///
/// This function panics if connection to NetBox fails.
// pub fn validate_config_data(state: &ThanixClient, config_data: &ConfigData) -> Result<(), Error> {
//     Ok(())
// }

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
