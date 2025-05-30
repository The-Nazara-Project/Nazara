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

/// Validates a device payload.
#[allow(unused)]
fn validate_device_payload(payload: WritableDeviceWithConfigContextRequest) -> bool {
    println!("Validating device payload...");

    todo!("Device payload validation not implemented yet!");

    false
}
