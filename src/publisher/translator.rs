//! # Translator Module
//!
//! This module handles the translation and processing of the data sent to or received from NetBox.
use thanix_client::types::{DeviceWithConfigContext, WritableDeviceWithConfigContextRequest};

use crate::Machine;

/// Translates a Request into a Vector of Objects.
pub fn translate_response_to_vec() -> Vec<WritableDeviceWithConfigContextRequest> {
    todo!()
}

pub fn information_to_device(machine: &Machine) -> WritableDeviceWithConfigContextRequest {
    todo!()
}
