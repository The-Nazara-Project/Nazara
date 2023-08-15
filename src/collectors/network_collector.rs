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
                    message: "Error: No network interfaces found!".to_string(),
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
                            Err(err) => {
                                println!("{}", err);
                                None
                            }
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
                                    Err(err) => {
                                        println!("{}", err);
                                        None
                                    }
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
                                    Err(err) => {
                                        println!("{}", err);
                                        None
                                    }
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
                            Err(err) => {
                                println!("{}", err);
                                None
                            }
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

/// Will collect the speed of a given interface by reading the entry of `/sys/class/net/<interface_name>/speed`.
///
/// If the content of the file is `-1`, the device is disabled and the interface speed is later set to None.
///
/// ## Arguments
///
/// * `interface_name: &str` - The name of the interface to investigate.
///
/// ## Returns
///
/// * `Ok(u32)` - If the entry for interface speed, in *`Mbps`*, returned by ethtool is not "Unknown".
/// * `Err` - If the file cannot be read, does not exist or the content is `-1` an Err is returned.
fn collect_interface_speed(interface_name: &str) -> Result<u32, String> {
    let interface_path: String = format!("/sys/class/net/{}/speed", interface_name);

    let output: Output =
        Command::new("cat")
            .arg(interface_path)
            .output()
            .map_err(|e: std::io::Error| {
                format!(
                    "ERROR: Failed to open /sys/class/net/{}/speed: {}.",
                    interface_name, e
                )
            })?;

    if !output.status.success() {
        return Err(format!(
            "WARNING: Collecting network speed for interface '{}' failed. This might happen with the loopback or wireless devices.",
            interface_name,
        ));
    }

    let output_str: String = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
        .replace("\n", "");

    if output_str == "-1" {
        return Err(format!(
            "INFO: No interface speed known for '{}'. The interface might be disabled.",
            interface_name
        ));
    }

    let interface_speed: u32 =
        output_str
            .parse::<u32>()
            .map_err(|e: std::num::ParseIntError| {
                format!(
                    "ERROR: Failed to parse speed as u32 for interface '{}': {}",
                    interface_name, e
                )
            })?;

    Ok(interface_speed)
}

#[cfg(test)]
mod network_collector_tests {
    use super::*;
    use mockall::{automock, predicate::*};

    /// Test for the `collect_network_interface`function.
    ///
    /// This test checks if the function `collect_network_information`returns a non-empty vector or throws a
    /// `NoNetworkInterfacesException` with the expected message.
    #[test]
    fn test_collect_network_information() {
        match collect_network_information() {
            Ok(network_interfaces) => {
                assert!(
                    !network_interfaces.is_empty(),
                    "No network interfaces found"
                );
            }
            Err(e) => match e {
                collector_exceptions::NoNetworkInterfacesException { message } => {
                    assert_eq!(message, "Error: No network interfaces found!".to_string());
                }
            },
        }
    }

    /// # Test the `construct_network_information` method.
    ///
    /// Test if the NetworkInformation objects are constructed correctly or if an `NoNetworkInterfacesException` is raised.
    #[test]
    fn test_construct_network_information() {
        match construct_network_information() {
            Ok(network_information) => {
                assert!(
                    !network_information.is_empty(),
                    "No network information found"
                );

                for network_info in network_information {
                    // Validate the structure and content of the NetworkInformation object
                    assert!(
                        !network_info.name.is_empty(),
                        "Network name cannot not be empty!"
                    );
                    assert!(
                        network_info.mac_addr.is_some(),
                        "MAC address must be present!\n\nFaulty Interface:'{}'\n",
                        network_info.name
                    );
                    assert!(
                        network_info.index.is_some(),
                        "Interface index must be present!\n\nFaulty Interface:'{}'\n",
                        network_info.name
                    );

                    // Validate the IP addresses
                    if let Some(v4ip) = network_info.v4ip {
                        assert!(
                            v4ip.is_ipv4(),
                            "IPv4 address must be valid!\n\nFaulty Interface:'{}'\n",
                            network_info.name
                        );
                    }
                    if let Some(v6ip) = network_info.v6ip {
                        assert!(
                            v6ip.is_ipv6(),
                            "IPv6 address must be valid!\n\nFaulty Interface:'{}'\n",
                            network_info.name
                        );
                    }

                    // Validate the interface speed
                    assert!(
                        network_info.interface_speed.is_some()
                            || network_info.interface_speed.is_none(),
                        "Interface speed must be either Some(speed) or None!"
                    );
                }
            }
            Err(e) => match e {
                collector_exceptions::NoNetworkInterfacesException { message } => {
                    assert_eq!(
                        message,
                        "Error: No network interfaces to process".to_string()
                    );
                }
            },
        }
    }

    #[automock]
    pub trait MockFs {
        fn read_to_string(&self, path: &str) -> std::io::Result<String>;
    }

    impl MockFs for std::fs::File {
        fn read_to_string(&self, path: &str) -> std::io::Result<String> {
            std::fs::read_to_string(path)
        }
    }

    #[test]
    fn test_collect_network_speed() {
        // Mock behaviour of the file system
        let mut mock_fs = MockFs::new();
        let interface_name = "eth0";
        let speed_file_path = format!("/sys/class/net/{}/speed", interface_name);

        // Test 1: Interface speed is known
        mock_fs
            .expect_read_to_string()
            .with(eq(speed_file_path.clone()))
            .returning(|_| Ok(String::from("1000")));

        let result = collect_interface_speed(interface_name);
        assert_eq!(result, Ok(1000));
    }
}
