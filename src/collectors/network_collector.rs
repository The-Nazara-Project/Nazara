//! ## Network Collector Module
//!
//! This module provides logic to collect and process Information about all network interfaces a device has.
//!

/*
TODO:
- Distinguish between actual and virtual PCI Network Interfaces
- Implement Error Checking
*/

use network_interface::Addr::{V4, V6};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::fs;
use std::net::IpAddr;

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
/// * is_physical - `bool` whether this device is a physical or virtual interface.
/// * is_connected - `bool` whether the interface is connected. Determined by if it has an address or not.
#[derive(Debug)]
pub struct NetworkInformation {
    name: String,
    v4ip: Option<IpAddr>,
    v4broadcast: Option<IpAddr>,
    v4netmask: Option<IpAddr>,
    v6ip: Option<IpAddr>,
    v6broadcast: Option<IpAddr>,
    v6netmask: Option<IpAddr>,
    mac_addr: Option<String>,
    index: u32,
    is_physical: bool,
    is_connected: bool,
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
            Some(_v) => {
                // Cases where only one set of addresses exist.
                if network_interface.addr.len() == 0 {
                    network_information = NetworkInformation {
                        name: network_interface.name,
                        v4ip: None,
                        v4broadcast: None,
                        v4netmask: None,
                        v6ip: None,
                        v6broadcast: None,
                        v6netmask: None,
                        mac_addr: network_interface.mac_addr,
                        index: network_interface.index,
                        is_physical: true,
                        is_connected: false,
                    };
                } else if network_interface.addr.len() == 1 {
                    // Match if the only set of ip addresses is ipv4 or ipv6
                    match network_interface.addr[0] {
                        V4(_ip) => {
                            network_information = NetworkInformation {
                                name: network_interface.name,
                                v4ip: Some(network_interface.addr[0].ip()),
                                v4broadcast: network_interface.addr[0].broadcast(),
                                v4netmask: network_interface.addr[0].netmask(),
                                v6ip: None,
                                v6broadcast: None,
                                v6netmask: None,
                                mac_addr: network_interface.mac_addr,
                                index: network_interface.index,
                                is_physical: true,
                                is_connected: true,
                            }
                        }
                        V6(_ip) => {
                            network_information = NetworkInformation {
                                name: network_interface.name,
                                v4ip: None,
                                v4broadcast: None,
                                v4netmask: None,
                                v6ip: Some(network_interface.addr[0].ip()),
                                v6broadcast: network_interface.addr[0].broadcast(),
                                v6netmask: network_interface.addr[0].netmask(),
                                mac_addr: network_interface.mac_addr,
                                index: network_interface.index,
                                is_physical: true,
                                is_connected: true,
                            }
                        }
                    }
                } else {
                    network_information = NetworkInformation {
                        name: network_interface.name,
                        v4ip: Some(network_interface.addr[0].ip()),
                        v4broadcast: network_interface.addr[0].broadcast(),
                        v4netmask: network_interface.addr[0].netmask(),
                        v6ip: Some(network_interface.addr[1].ip()),
                        v6broadcast: network_interface.addr[1].broadcast(),
                        v6netmask: network_interface.addr[1].netmask(),
                        mac_addr: network_interface.mac_addr,
                        index: network_interface.index,
                        is_physical: true,
                        is_connected: true,
                    };
                }
            }
            None => {
                // Raise exception here
                network_information = NetworkInformation {
                    name: network_interface.name,
                    v4ip: None,
                    v4broadcast: None,
                    v4netmask: None,
                    v6ip: None,
                    v6broadcast: None,
                    v6netmask: None,
                    mac_addr: network_interface.mac_addr,
                    index: network_interface.index,
                    is_physical: true,
                    is_connected: true,
                };
            }
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
/// This function checks if the interface is a Physical Function (PF)
///
/// Check if network interface is physical or not.
///
/// This function constructs a path to the interface's virtfn0 directory.<br>
/// This directory only exists for physical network devices, not virtual ones or non-SR-IOV devices.
///
/// ### Arguments
///
/// * `interface` - A string slice that holds the name of the network interface.
///
/// ### Returns
///
/// * `bool` - Returns `true` if the network interface is a PF, `false` otherwise
fn check_for_physical_nw(interface_name: &str) -> bool {
    /*
    Check if network interface is physical or not.

    This function constructs a path to the interface's virtfn0 directory.
    This directory only exists for physical network devices, not virtual ones or non-SR-IOV devices.

    Returns true if the path exists, meaning the interface is a physical network device.
    Returns false if the path does not exist, meaning the interface is a virtual device.
    */
    let path: String = format!("/sys/class/net/{}/device/virtfn0", interface_name);

    return fs::metadata(&path).is_ok();
}
