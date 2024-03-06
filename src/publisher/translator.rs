//! # Translator Module
//!
//! This module handles the translation and processing of the data sent to or received from NetBox.
//!
//! TODO:
//! - Identify primary IPv4 or IPv6 using the primary_network_interface field from `ConfigData`.
use thanix_client::types::{
    WritableDeviceWithConfigContextRequest, WritableVirtualMachineWithConfigContextRequest,
};

use crate::{configuration::config_parser::ConfigData, Machine};

/// Translate the machine information to a `WritableDeviceWithConfigContextRequest` required by
/// NetBox's API.
pub fn information_to_device(machine: &Machine, config_data: ConfigData) -> WritableDeviceWithConfigContextRequest {
    let device: WritableDeviceWithConfigContext = WritableDeviceWithConfigContext {
        name: machine.name,
        device_type: config_data.device_type,
        role: config_data.role,
        # TODO!
    };

    device
}

pub fn information_to_vm(machine: &Machine) -> WritableVirtualMachineWithConfigContextRequest {
    todo!()
}
