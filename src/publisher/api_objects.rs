//! API Objects module.
//!
//! This module lists all objects found in the NetBox API.

mod api_objects;

use std::net::IpAddr;

/// Represent any device on NetBox.
///
/// # Members
///
/// - `name: Option<String>` - Optional Name of a device (optional).
/// - `role: Role` - The functional Role of this device.
/// - `device_type: DeviceType` - The hardware device type which defines the device's make & model.
/// - `airflow: Option<String>` - The direction in which air circulates through the device chassis.
/// - `serial_number: String` - The unique physical serial number assigned to this device by the manufacturer.
/// - `asset_tag: String` - A unique, locally-administered label used to identify hardware resources.
/// - `site: Site` - The Site in which this device is located.
/// - `location: Option<Location>` - The specific location where this device is located within the site (optional).
/// - `rack: Option<Rack>` - The Rack within which this device is installed (optional).
/// - `rack_face: Option<RackFace>` - If installed in a rack, this field denotes the primary face on which
/// the device is mounted.
/// - `position: Option<Position>` - If installed in a rack, this field indicates the base rack unit in which
/// the device is mounted.
/// - `coordinates: Option<Coordinates>` - Latitude & Longitude coordinates for geolocation.
/// - `status: Option<String>` - Status of the device.
/// - `platform: Option<Platform>` - The device may be associated to a particular Platform to indicate its OS.
/// - `configuration_template: Option<ConfTemplate>` - The Configuration Template from which the configuration for this
/// device can be rendered.
/// - `primary_ipv4: Option<IPAddr>` - Primary IPv4 Address. (NetBox *will* prefer IPv6 if applicable.)
/// - `primary_ipv6: Option<IpAddr>` - Primary IPv6 Address.
/// - `out_of_band_ip: Option` - Each device may have a out-of-band IP Address to access network infrastructure from a
/// physically separate management network.
/// - `cluster: Option<Cluster>` - Device may serve as host for a virtualization cluster.
/// - `virtual_chassis: Option<VirtualChassis>` - The virtual chassis which this device may be a member of.
/// - `vc_position: Option<VCPosition>` - If assigned to a VC, this field indicates the device's member position.
/// - `vc_priority: Option<VCPriority>` - If assigned to a VC, this field indicates the device's member position.
/// - `local_config_context: Option<ContextData>` - Any unique context data to be associated with the device.
pub struct Device {
    pub name: Option<String>,
    pub role: Role,
    pub device_type: DeviceType,
    pub airflow: Option<String>,
    pub serial_number: String,
    pub asset_tag: String,
    pub site: Site,
    pub location: Option<Location>,
    pub rack: Option<Rack>,
    pub rack_face: Option<RackFace>,
    pub position: Option<RackPosition>,
    pub coordinates: Option<Coordinates>,
    pub status: Option<String>,
    pub platform: Option<Platform>,
    pub primary_ipv4: Option<IPAddr>,
    pub primary_ipv6: Option<IPAddr>,
    pub out_of_band_addr: Option<IPAddr>,
    pub cluster: Option<Cluster>,
    pub virtual_chassis: Option<VirtualChassis>,
    pub vc_position: Option<VCPosition>,
    pub vc_priority: Option<VCPriority>,
    pub local_config_context: Option<ContextData>
}

/// Represent a Virtual Machine
///
/// # Members
///
/// - `name: String` - The virtual machine's configured name. ust be unique to the assigned cluster and tenant.
/// - `role: Role` - The functional role of the VM.
/// - `status: Option<String>` - The operational status of the VM.
/// - `site: Option<Site>` - The site this VM is assigned to.
/// - `cluster: Option<Cluster>` - The cluster that the VM is assigned to.
/// - `host_device: Option<Device>` - The host device within the assigned site/cluster on which VM resides.
/// - `platform: Option<Platform>` - The platform this VM operates on.
/// - `primary_ipv4: Option<IPAddr>` - Primary IPv4 Address. (NetBox *will* prefer IPv6 if applicable.)
/// - `primary_ipv6: Option<IpAddr>` - Primary IPv6 Address.
pub struct VirtualMachine {
// TOD: Finish implementing these structs
}

/// Represents the Device's role.
///
/// # Members
///
/// - `name: String` - A unique human-readable name.
/// - `slug: String` - A unique URL-identifier. (Can also be used for filtering.)
/// - `color: String` - The color used when displaying the role in the UI.
/// - `vm_role: bool` - This role may be assigned to virtual machines.
/// - `config_template: ConfigTemplate`
pub struct Role {
    pub name: String,
    pub slug: String,
    pub color: String,
    pub vm_role: bool,
    pub config_template: ConfigTemplate
}