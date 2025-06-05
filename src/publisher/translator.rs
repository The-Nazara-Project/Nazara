//! # Translator Module
//!
//! This module's sole responsibility is translating system information into payloads usable by the
//! [`api_client`](crate::publisher::api_client) module to register the machine in NetBox.
//!
//! The information comes from the two collectors (both
//! [`crate::collectors::dmi`] and
//! [`crate::collectors::network`] as well as the
//! [`crate::configuration::parser`] module.
//! It is then formed into data structures that NetBox can understand.
//!
//! This approach has been chosen, so the collectors and configuration parser can remain relatively
//! unchanged in case NetBox significantly redesigns their API.

use crate::{
    Machine,
    collectors::network::NetworkInformation,
    configuration::parser::{CommonConfig, ConfigData, DeviceConfig, VmConfig},
};
use serde_json::{Value, json};
use std::{collections::HashMap, net::IpAddr};
use thanix_client::types::{
    WritableDeviceWithConfigContextRequest, WritableIPAddressRequest, WritableInterfaceRequest,
    WritableVMInterfaceRequest, WritableVirtualMachineWithConfigContextRequest,
};

/// Translates the machine information to a [`WritableDeviceWithConfigContextRequest`]
/// as required by NetBox's API.
///
/// # Note
///
/// Certain information provided in the config file will be overwritten if a different one is detected by the collector!
///
/// # Parameters
/// - `state: &ThanixClient` - API Client instance used for search and validation.
/// - `machine: &Machine` -  Collected information about the device.
/// - `config_data: ConfigData` -  Additional information about the device provided by config file or CLI.
///
/// # Returns
///
/// - `WritableDeviceWithConfigContextRequest` - A device payload.
pub fn information_to_device(
    machine: &Machine,
    common: &CommonConfig,
    device: &DeviceConfig,
) -> WritableDeviceWithConfigContextRequest {
    println!("Creating Device object...");

    let mut payload = WritableDeviceWithConfigContextRequest::default();

    payload.name = Some(common.name.clone());
    payload.device_type = Value::from(device.device_type);
    payload.role = Value::from(device.role);
    payload.serial = machine.dmi_information.system_information.serial.clone();
    payload.asset_tag = Some(machine.dmi_information.chassis_information.asset.clone());
    payload.site = Value::from(device.site);
    payload.status = common.status.clone();
    payload.comments = common.comments.clone();
    payload.custom_fields = machine.custom_information.clone();
    payload.description = common.description.clone();

    payload
}

/// Translate gathered information about the virtual machine into a usable Payload.
/// Returns a payload for the VM POST or UPDATE request.
///
/// - `state`: The client instance to be used for communication.
/// - `machine`: The collected information about the virtual machine.
/// - `config_data`: Data parsed from the `nazar-config.toml`.
#[allow(unused)]
pub fn information_to_vm(
    machine: &Machine,
    common: &CommonConfig,
    vm: &VmConfig,
) -> WritableVirtualMachineWithConfigContextRequest {
    let mut payload = WritableVirtualMachineWithConfigContextRequest::default();

    payload.name = common.name.clone();
    payload.serial = machine.dmi_information.system_information.serial.clone();
    payload.status = common.status.clone();
    payload.comments = common.comments.clone();
    payload.custom_fields = machine.custom_information.clone();
    payload.description = common.description.clone();
    payload.cluster = Some(Value::from(vm.cluster));
    payload.vcpus = machine
        .dmi_information
        .cpu_information
        .core_count
        .parse()
        .ok();

    payload
}

/// Translates gathered information into a Interface payload.
///
/// # Parameters
/// - `interface: &NetworkInformation` - The interface to be translated into a payload.
/// - `config_data: &ConfigData` - The configuration data.
/// - `device_id: &i64` - The ID of the device that this interface belongs to.
///
/// # Returns
/// - `WritableInterfaceRequest` - The payload to use for Interface operations.
pub fn information_to_interface(
    config_data: &CommonConfig,
    interface: &NetworkInformation,
    device_id: &i64,
) -> WritableInterfaceRequest {
    println!(
        "Creating Network Interface payload for '{}'...",
        &interface.name
    );

    let mut payload = WritableInterfaceRequest::default();

    payload.device = Value::from(device_id.to_owned());
    payload.name = interface.name.clone();

    if let Some(x) = &interface.mac_addr {
        payload.primary_mac_address = Some(json!({"mac_address": x}));
    }
    payload.speed = Some(interface.interface_speed.unwrap_or_default());
    payload.description = config_data.comments.clone();
    payload.mark_connected = interface.is_connected;
    payload.enabled = true;
    payload.custom_fields = Some(HashMap::new());

    payload
}

pub fn information_to_vm_interface(
    config_data: &CommonConfig,
    interface: &NetworkInformation,
    device_id: &i64,
) -> WritableVMInterfaceRequest {
    println!(
        "Creating Network Interface payload for '{}'...",
        &interface.name
    );

    let mut payload = WritableVMInterfaceRequest::default();

    payload.virtual_machine = Value::from(device_id.to_owned());
    payload.name = interface.name.clone();

    if let Some(x) = &interface.mac_addr {
        payload.primary_mac_address = Some(json!({"mac_address": x}));
    }
    payload.description = config_data.comments.clone();
    payload.enabled = true;
    payload.custom_fields = Some(HashMap::new());

    payload
}

/// Returns the payload necessary to create a new IP address.
///
/// # Parameters
/// - `interface_address: IpAddr` - The IpAddress of the interface to register.
/// - `interface_id: i64` - ID of the network interface this IP belongs to.
pub fn information_to_ip(
    interface_address: IpAddr,
    interface_id: i64,
    is_vm: bool,
) -> WritableIPAddressRequest {
    println!("Creating IP Address payload...");

    // payload.vrf = todo!();
    // payload.tenant = todo!();
    // payload.role = todo!();
    // payload.nat_inside = todo!();
    // payload.dns_name = todo!();
    // payload.tags = todo!();
    WritableIPAddressRequest {
        address: format!("{interface_address}"),
        status: String::from("active"),
        assigned_object_type: Some(String::from(if is_vm {
            "virtualization.vminterface"
        } else {
            "dcim.interface"
        })),
        assigned_object_id: Some(interface_id as u64),
        description: String::from("This Address was automatically created by Nazara."),
        comments: String::from("Automatically created by Nazara."),
        custom_fields: Some(HashMap::new()),
        ..Default::default()
    }
}
