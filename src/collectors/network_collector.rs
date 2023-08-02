//! ## Network Collector Module
//!
//! This module provides logic to collect and process Information about all network interfaces a device has.
//!


/*
TODO:
- Distinguish between actual and virtual PCI Network Interfaces
- Implement Error Checking
*/

use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use std::net::IpAddr;
use std::fs;

/// ## Network Information
///
/// This object contains information about one specific network interface.
///
/// ### Members
///
/// * name - `String` containing the name of a network interface.
/// * v4ip - `IpAddr` the IPv4 address of this interface.
/// * v4broadcast - `IpAddr` the IPv4 broadcast address of this interface.
/// * v4netmask - `IpAddr` the IPv4 netmask of this interface.
/// * v6ip - `IpAddr` the IPv6 address of this interface.
/// * v6broadcast - `IpAddr` the IPv6 broadcast address of this interface.
/// * v6netmask - `IpAddr` the IPv6 netmask of this interface.
/// * mac_addr - `String` the mac address of this interface.
/// * index - `u32` the index of this interface.
/// * is_virutal - `bool` whether this device is a physical or virtual interface.
pub struct NetworkInformation {
    name: String,
    v4ip: IpAddr,
    v4broadcast: Option<IpAddr>,
    v4netmask: Option<IpAddr>,
    v6ip: IpAddr,
    v6broadcast: Option<IpAddr>,
    v6netmask: Option<IpAddr>,
    mac_addr: Option<String>,
    index: u32,
    is_physical: bool,
}


/// Collect information about all network interfaces.
///
/// ### Returns
///
/// Vec<NetworkInterface> - A list of all NetworkInterfaces that the crate was able to collect.
pub fn collect_network_information() -> Vec<NetworkInterface> {
    /*
     * Collect information about all available network interfaces.
     *
     * Will return a Vector containing a JSON-like data format.
     */
    let network_interfaces: Vec<NetworkInterface> = NetworkInterface::show().unwrap();

    return network_interfaces;
}


/// Constructs instances of the NetworkInformation struct.
///
/// ### Returns
///
/// A list of NetworkInformation objects including information about whether a network is virtual or physical.
pub fn construct_network_information() -> Vec<NetworkInformation> {
    /*
     * Deconstruct NetworkInterface vector and construct NetworkInformation objects from it.
     */

    let raw_information: Vec<NetworkInterface> = collect_network_information();
    let mut interfaces: Vec<NetworkInformation> = Vec::new();
    let mut network_information: NetworkInformation;

    for network_interface in raw_information {
        match Some(&network_interface.addr) {
            // TODO: Check if reference is correct here
            Some(v) => {
                network_information = NetworkInformation {
                    name: network_interface.name,
                    v4ip: network_interface.addr[0].ip(), // if reference in match not correct, this will throw errors.
                    v4broadcast: network_interface.addr[0].broadcast(),
                    v4netmask: network_interface.addr[0].netmask(),
                    v6ip: network_interface.addr[1].ip(),
                    v6broadcast: network_interface.addr[1].broadcast(),
                    v6netmask: network_interface.addr[1].netmask(),
                    mac_addr: network_interface.mac_addr,
                    index: network_interface.index,
                    is_physical: true,
                };
            }
            None => {}
        }

        if !check_for_physical_nw(&network_information.name) {
            network_information.is_physical = false;
        }

        interfaces.push(network_information)
    }

    return interfaces;
}


/// NetBox needs to differentiate between a physical and virtual network device.
///
/// Checks if the interface is a Physical Function (PF)
///
/// ### Arguments
///
/// * `interface` - A string slice that holds the name of the network interface.
///
/// ### Returns
///
/// * `bool` - Returns `true` if the network interface is a PF, `false` otherwise
///
/// ### Example
///
/// let is_pf = check_for_physical_nw("eth0");
///
/// println!("Is eth0 a Physical Function? {}", is_pf);
///
fn check_for_physical_nw(interface_name: &str) -> bool {
    /*
    Check if network interface is physical or not.

    This function constructs a path to the interface's virtfn0 directory.
    This directory only exists for phyiscal network devices, not virtual ones or non-SR-IOV devices.

    Returns true if the path exists, meaning the interface is a physical network device.
    Returns false if the path does not exist, meaning the interface is a virtual device.
    */
    let path: String = format!("/sys/class/net/{}/device/virtfn0", interface);

    return fs::metadata(&path).is_ok();
}
