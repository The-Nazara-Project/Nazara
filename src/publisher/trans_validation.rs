//! # Translator Validation Module
//!
//! This module is responsible for validating all system parameters before
//! a creation or update request is sent to NetBox.
//!
//! This includes:
//!
//! * Confirming certain API objects (like IP Adresses) are in fact present in NetBox.
//! * Validating that the Names of certain API objects are valid and correspond to an ID.
use thanix_client::{
    types::{
        WritableDeviceWithConfigContextRequest, WritableVirtualMachineWithConfigContextRequest,
    },
    util::ThanixClient,
};

use super::publisher_exceptions::PayloadValidationError;

/// Validate the `Device` Payload.
///
/// # Parameters
///
/// * payload: `&WritableDeviceWithConfigContextRequest` - The struct to validate.
///
/// # Returns
///
/// - Ok(())
///
/// # Panics
///
/// This function panics if the connection to NetBox fails.
pub fn validate_device_payload(
    state: &ThanixClient,
    payload: &WritableDeviceWithConfigContextRequest,
) -> Result<(), PayloadValidationError> {
    println!("Validating device payload...");

    todo!("Device Payload validation not yet implemented!")
}

/// Validate the `VM` Payload.
///
/// # Parameters
///
/// * state: `&ThanixClient` - The ThanixClient instance to use for communication with NetBox.
/// * payload: `&WritableVirtualMachineWithConfigContextRequest` - The payload to validate.
///
/// # Returns
///
/// Returns `Ok(())` when all relevant fields of the request payload are valid. Otherwise
/// return `Err(PayloadValidationError)`.
///
/// # Panics
///
/// This function panics if the connection to NetBox fails.
pub fn validate_vm_payload(
    state: &ThanixClient,
    payload: &WritableVirtualMachineWithConfigContextRequest,
) -> Result<(), PayloadValidationError> {
    println!("Validating VM payload...");

    todo!("Virtual Machine payload validation not yet implemented!");
}
