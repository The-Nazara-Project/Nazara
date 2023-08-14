//! ## Network Collector Module
//!
//! This module provides logic to collect and process Information about all network interfaces a device has.
//!
use network_interface::Addr::{V4, V6};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::fs;
use std::net::IpAddr;
use std::process::{Command, Output};

use super::collector_exceptions;

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
/// * interface_speed - `u32` the speed of the interface.
#[derive(Debug)]
pub struct NetworkInformation {
    name: String,
    interface_speed: Option<u32>,
    v4ip: Option<IpAddr>,
    v4broadcast: Option<IpAddr>,
    v4netmask: Option<IpAddr>,
    v6ip: Option<IpAddr>,
    v6broadcast: Option<IpAddr>,
    v6netmask: Option<IpAddr>,
    mac_addr: Option<String>,
    index: Option<u32>,
    is_physical: bool,
    is_connected: bool,
}

/// Collect information about all network interfaces.
///
/// ### Returns
///
/// Vec<NetworkInterface> - A list of all NetworkInterfaces that the crate was able to collect.
///
/// ### Panics
///
/// This function will panic if NetworkInterface::show() returns an Error leading to no interfaces being collected.
pub fn collect_network_information(
) -> Result<Vec<NetworkInterface>, collector_exceptions::NoNetworkInterfacesException> {
    /*
     * Collect information about all available network interfaces.
     *
     * Will return a Vector containing a JSON-like data format.
     */
    let network_interfaces_result: Result<Vec<NetworkInterface>, network_interface::Error> =
        NetworkInterface::show();

    match network_interfaces_result {
        Ok(network_interfaces) => {
            if network_interfaces.is_empty() {
                return Err(collector_exceptions::NoNetworkInterfacesException {
                    message: "FATAL: No network interfaces found!".to_string(),
                });
            } else {
                return Ok(network_interfaces);
            }
        }
        Err(_) => {
            let exc: collector_exceptions::UnableToCollectDataError =
                collector_exceptions::UnableToCollectDataError {
                    message: "FATAL: Unable to collect information about network interfaces!"
                        .to_string(),
                };
            exc.panic();
        }
    }
}

/// Constructs instances of the NetworkInformation struct.
///
/// ### Returns
///
/// A list of NetworkInformation objects including information about whether a network is virtual or physical.
///
/// ### Panics
///
/// This function will panic if a network interface, for some reason, lacks an address block.
pub fn construct_network_information(
) -> Result<Vec<NetworkInformation>, collector_exceptions::NoNetworkInterfacesException> {
    /*
     * Deconstruct NetworkInterface vector and construct NetworkInformation objects from it.
     */
    let raw_information: Vec<NetworkInterface> = match collect_network_information() {
        Ok(information) => information,
        Err(_) => {
            return Err(collector_exceptions::NoNetworkInterfacesException {
                message: "Error: No network interfaces to process".to_string(),
            });
        }
    };

    let mut interfaces: Vec<NetworkInformation> = Vec::new();
    let mut network_information: NetworkInformation;

    for network_interface in raw_information {
        match Some(&network_interface.addr) {
            Some(_v) => {
                // Cases where only one set of addresses exist.
                if network_interface.addr.is_empty() {
                    network_information = NetworkInformation {
                        name: network_interface.name.clone(),
                        v4ip: None,
                        v4broadcast: None,
                        v4netmask: None,
                        v6ip: None,
                        v6broadcast: None,
                        v6netmask: None,
                        mac_addr: network_interface.mac_addr,
                        index: Some(network_interface.index),
                        is_physical: true,
                        is_connected: false,
                        interface_speed: match collect_interface_speed(&network_interface.name) {
                            Ok(speed) => Some(speed),
                            Err(_) => None,
                        },
                    };
                } else if network_interface.addr.len() == 1 {
                    // Match if the only set of ip addresses is ipv4 or ipv6
                    match network_interface.addr[0] {
                        V4(_ip) => {
                            network_information = NetworkInformation {
                                name: network_interface.name.clone(),
                                v4ip: Some(network_interface.addr[0].ip()),
                                v4broadcast: network_interface.addr[0].broadcast(),
                                v4netmask: network_interface.addr[0].netmask(),
                                v6ip: None,
                                v6broadcast: None,
                                v6netmask: None,
                                mac_addr: network_interface.mac_addr,
                                index: Some(network_interface.index),
                                is_physical: true,
                                is_connected: true,
                                interface_speed: match collect_interface_speed(
                                    &network_interface.name,
                                ) {
                                    Ok(speed) => Some(speed),
                                    Err(_) => None,
                                },
                            }
                        }
                        V6(_ip) => {
                            network_information = NetworkInformation {
                                name: network_interface.name.clone(),
                                v4ip: None,
                                v4broadcast: None,
                                v4netmask: None,
                                v6ip: Some(network_interface.addr[0].ip()),
                                v6broadcast: network_interface.addr[0].broadcast(),
                                v6netmask: network_interface.addr[0].netmask(),
                                mac_addr: network_interface.mac_addr,
                                index: Some(network_interface.index),
                                is_physical: true,
                                is_connected: true,
                                interface_speed: match collect_interface_speed(
                                    &network_interface.name,
                                ) {
                                    Ok(speed) => Some(speed),
                                    Err(_) => None,
                                },
                            }
                        }
                    }
                } else {
                    network_information = NetworkInformation {
                        name: network_interface.name.clone(),
                        v4ip: Some(network_interface.addr[0].ip()),
                        v4broadcast: network_interface.addr[0].broadcast(),
                        v4netmask: network_interface.addr[0].netmask(),
                        v6ip: Some(network_interface.addr[1].ip()),
                        v6broadcast: network_interface.addr[1].broadcast(),
                        v6netmask: network_interface.addr[1].netmask(),
                        mac_addr: network_interface.mac_addr,
                        index: Some(network_interface.index),
                        is_physical: true,
                        is_connected: true,
                        interface_speed: match collect_interface_speed(&network_interface.name) {
                            Ok(speed) => Some(speed),
                            Err(_) => None,
                        },
                    };
                }
            }
            None => {
                // If a Network interface is completely missing an address block, it is assumed that it is invalid.
                // This will raise a custom exception and cause the program to panic.
                let exc = collector_exceptions::InvalidNetworkInterfaceError {
                    message: "FATAL: A Network interface cannot be recognized!".to_string(),
                };
                exc.panic();
            }
        }
        if !check_for_physical_nw(&network_information.name) {
            network_information.is_physical = false;
        }
        interfaces.push(network_information)
    }
    return Ok(interfaces);
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
/// * `interface_name` - A string slice that holds the name of the network interface.
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

/// Will attempt to collect the network interface's speed by running `ethtool <interface_name> | grep "Speed"` and
/// catching the output.
///
/// If "Speed" is to be reported as "Unknown!", None is returned.
///
/// ## Arguments
///
/// * `interface_name: &str` - The name of the interface to investigate.
///
/// ## Returns
///
/// * `Ok(u32)` - If the entry for interface speed returned by ethtool is not "Unknown".
/// * `Err` - If the entry for interface speed returned by ethtool is "Unknown".
fn collect_interface_speed(interface_name: &str) -> Result<u32, String> {
    let output: Output = Command::new("ethtool")
        .arg("-s")
        .arg(interface_name)
        .output()
        .map_err(|e: std::io::Error| format!("Failed to execute ethtool command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ethtool command failed with exit code: {}",
            output.status
        ));
    }

    let output_str: String = String::from_utf8_lossy(&output.stdout).to_string();

    let speed_str: &str = output_str
        .lines()
        .find(|line: &&str| line.starts_with("Speed:"))
        .and_then(|line: &str| line.split_whitespace().nth(1))
        .ok_or_else(|| "Failed to parse speed from ethtool output".to_string())?;

    if speed_str == "Unknown!" {
        return Err(format!(
            "INFO: No interface speed known for {}. The interface may be virtual or disabled.",
            interface_name
        ));
    }

    let interface_speed: u32 = speed_str
        .parse::<u32>()
        .map_err(|e: std::num::ParseIntError| format!("Failed to parse speed as u32: {}", e))?;

    Ok(interface_speed)
}
