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
///
/// # Parameters
///
/// - `machine: &Machine` - Collected information about the device
/// - `config_data: ConfigData` - Additional information about the device provided by config file
/// or CLI
///
/// # Returns
///
/// - `device: WritableDeviceWithConfigContextRequest` - Payload for machine creation request
pub fn information_to_device(
    machine: &Machine,
    config_data: ConfigData,
) -> WritableDeviceWithConfigContextRequest {
    //    let device: WritableDeviceWithConfigContextRequest = WritableDeviceWithConfigContextRequest {
    //        name: Some(config_data.system.name),
    //        device_type: config_data.system.device_type,
    //        role: config_data.system.device_role,
    //        tenant: config_data.system.tenant,
    //    };
    //    device
    todo!()
}

fn get_platform_id(machine: &Machine) -> Option<i64> {
    todo!("Get platform by name an return platform id!")
}

pub fn information_to_vm(machine: &Machine) -> WritableVirtualMachineWithConfigContextRequest {
    todo!()
}
