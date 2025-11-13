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
    configuration::parser::{CommonConfig, DeviceConfig, VmConfig},
};
use serde_json::{Value, json};
use std::{collections::HashMap, net::IpAddr};
use thanix_client::types::{
    PatchedWritableDeviceWithConfigContextRequest,
    PatchedWritableVirtualMachineWithConfigContextRequest, WritableDeviceWithConfigContextRequest,
    WritableIPAddressRequest, WritableInterfaceRequest, WritableVMInterfaceRequest,
    WritableVirtualMachineWithConfigContextRequest,
};

/// Translates the machine information to a [`WritableDeviceWithConfigContextRequest`]
/// as required by NetBox's API.
///
/// # Note
///
/// Certain information provided in the config file will be overwritten if a different one is detected by the collector!
///
/// # Parameters
/// - `state`: API Client instance used for search and validation.
/// - `machine`: Collected information about the device.
/// - `config_data`: Additional information about the device provided by config file or CLI.
///
/// # Returns
/// A device payload.
pub fn information_to_device(
    machine: &Machine,
    common: &CommonConfig,
    device: &DeviceConfig,
) -> WritableDeviceWithConfigContextRequest {
    WritableDeviceWithConfigContextRequest {
        name: Some(compute_effective_name(
            &common.name,
            &machine.dmi_information.system_information.hostname,
        )),
        device_type: Value::from(device.device_type),
        role: Value::from(device.role),
        serial: machine.dmi_information.system_information.serial.clone(),
        site: Value::from(device.site),
        status: common.status.clone(),
        comments: common.comments.clone(),
        custom_fields: machine.custom_information.clone(),
        description: common.description.clone(),
        ..Default::default()
    }
}

pub fn information_to_existing_device(
    machine: &Machine,
    common: &CommonConfig,
    device: &DeviceConfig,
) -> PatchedWritableDeviceWithConfigContextRequest {
    PatchedWritableDeviceWithConfigContextRequest {
        name: Some(Some(compute_effective_name(
            &common.name,
            &machine.dmi_information.system_information.hostname,
        ))),
        device_type: Some(Value::from(device.device_type)),
        role: Some(Value::from(device.role)),
        serial: Some(machine.dmi_information.system_information.serial.clone()),
        site: Some(Value::from(device.site)),
        status: Some(common.status.clone()),
        description: Some(common.description.clone()),
        comments: Some(common.comments.clone()),
        custom_fields: Some(machine.custom_information.clone()),
        ..Default::default()
    }
}

/// Translate gathered information about the virtual machine into a usable Payload.
///
/// # Parameters
/// - `state`: The client instance to be used for communication.
/// - `machine`: The collected information about the virtual machine.
/// - `config_data`: Data parsed from the `nazar-config.toml`.
///
/// # Returns
/// A payload for the VM POST or UPDATE request.
pub fn information_to_vm(
    machine: &Machine,
    common: &CommonConfig,
    vm: &VmConfig,
) -> WritableVirtualMachineWithConfigContextRequest {
    WritableVirtualMachineWithConfigContextRequest {
        name: compute_effective_name(
            &common.name,
            &machine.dmi_information.system_information.hostname,
        ),
        serial: machine.dmi_information.system_information.serial.clone(),
        status: common.status.clone(),
        comments: common.comments.clone(),
        custom_fields: machine.custom_information.clone(),
        description: common.description.clone(),
        cluster: Some(Value::from(vm.cluster)),
        vcpus: machine
            .dmi_information
            .cpu_information
            .core_count
            .parse()
            .ok(),
        ..Default::default()
    }
}

pub fn information_to_existing_vm(
    machine: &Machine,
    common: &CommonConfig,
    vm: &VmConfig,
) -> PatchedWritableVirtualMachineWithConfigContextRequest {
    PatchedWritableVirtualMachineWithConfigContextRequest {
        name: Some(compute_effective_name(
            &common.name,
            &machine.dmi_information.system_information.hostname,
        )),
        serial: Some(machine.dmi_information.system_information.serial.clone()),
        status: Some(common.status.clone()),
        comments: Some(common.comments.clone()),
        custom_fields: Some(machine.custom_information.clone()),
        description: Some(common.description.clone()),
        cluster: Some(Some(Value::from(vm.cluster))),
        vcpus: Some(
            machine
                .dmi_information
                .cpu_information
                .core_count
                .parse()
                .ok(),
        ),
        ..Default::default()
    }
}

/// Translates gathered information into a Interface payload.
///
/// # Parameters
/// - `interface`: The interface to be translated into a payload.
/// - `config_data`: The configuration data.
/// - `device_id`: The ID of the device that this interface belongs to.
///
/// # Returns
/// The payload to use for Interface operations.
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
    payload.r#type = String::from("other");

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
/// - `interface_address`: The IpAddress of the interface to register.
/// - `interface_id`: ID of the network interface this IP belongs to.
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

/// Will compose the name of the device/VM entry.
///
/// If the configured name ends with '@' this name will be concatenated
/// with the detected hostname to create something like machine@host.name
///
/// # Parameters
/// * `config_name: &Option<String>` - The name read from the config file.
/// * `hostname: &str` - The hostname read from the system.
///
/// # Returns
/// The concatenated name as `String`.
pub fn compute_effective_name(config_name: &Option<String>, hostname: &str) -> String {
    match config_name {
        Some(name) => {
            if name.ends_with('@') {
                format!("{}{}", name, hostname)
            } else {
                name.clone()
            }
        }
        None => hostname.to_string(),
    }
}
