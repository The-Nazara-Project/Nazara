//! ## Network Collector Module
//!
//! This module provides logic to collect and process Information about all network interfaces a device has.
//!
use network_interface::Addr::{V4, V6};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::net::IpAddr;

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
#[derive(Serialize, Debug)]
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
                    message: "\x1b[31m[error]\x1b[0m No network interfaces found!".to_string(),
                });
            } else {
                return Ok(network_interfaces);
            }
        }
        Err(_) => {
            let exc: collector_exceptions::UnableToCollectDataError =
                collector_exceptions::UnableToCollectDataError {
                    message: "\x1b[31m[FATAL]\x1b[0m Unable to collect information about network interfaces!"
                        .to_string(),
                };
            exc.abort(20);
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
    println!("Collecting Network Information...");

    let raw_information: Vec<NetworkInterface> = match collect_network_information() {
        Ok(information) => information,
        Err(_) => {
            return Err(collector_exceptions::NoNetworkInterfacesException {
                message: "\x1b[31m[error]\x1b[0m No network interfaces to process".to_string(),
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
                        interface_speed: validate_network_speed(&network_interface.name),
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
                                interface_speed: validate_network_speed(&network_interface.name),
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
                                interface_speed: validate_network_speed(&network_interface.name),
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
                        interface_speed: validate_network_speed(&network_interface.name),
                    };
                }
            }
            None => {
                // If a Network interface is completely missing an address block, it is assumed that it is invalid.
                // This will raise a custom exception and cause the program to panic.
                let exc = collector_exceptions::InvalidNetworkInterfaceError {
                    message: "\x1b[31m[FATAL]\x1b[0m A Network interface cannot be recognized!".to_string(),
                };
                exc.abort(25);
            }
        }
        if !check_for_physical_nw(&network_information.name) {
            network_information.is_physical = false;
        }
        interfaces.push(network_information)
    }
    println!("\x1b[32m[success]\x1b[0m Network Interface collection completed.");
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

/// Validates if the speed of a network can be actually read and will print the error messages if it cannot.
///
/// ## Arguments
///
/// * `interface_name: &str` - The name of the interface to check.
///
/// ## Returns
///
/// - `u32` - Returns `u32` if the speed finding process succeeds.
/// - `None` - Returns `None` when the collection process fails.
fn validate_network_speed(interface_name: &str) -> Option<u32> {
    match build_interface_file_from_name(interface_name) {
        Ok(file) => match collect_interface_speed(interface_name, file) {
            Ok(speed) => Some(speed),
            Err(err) => {
                println!("{}", err);
                None
            }
        },
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

/// Builds the path to the interface's `speed` file using its name and reads the file if possible.
///
/// ## Arguments
///
/// * `interface_name: &str` - The name of the interface to be investigated.
///
/// ## Returns
///
/// * `Ok(std::fs::File)` - Returns the file object if it can be opened.
/// * `Err(String)` - If the file cannot be opened. This mostly happens with the loopback device or wireless devices and is a sysfs problem.
fn build_interface_file_from_name(interface_name: &str) -> Result<std::fs::File, String> {
    let interface_path: String = format!("/sys/class/net/{}/speed", interface_name);

    match std::fs::File::open(interface_path) {
        Ok(file) => {
            return Ok(file);
        }
        Err(_) => {
            return Err(format!(
                "\x1b[33m[warning]\x1b[0m Speed file for interface '{}' does not exist.",
                interface_name
            ))
        }
    }
}

/// Will collect the speed of a given interface by reading the entry of `/sys/class/net/<interface_name>/speed`.
///
/// If the content of the file is `-1`, the device is disabled and the interface speed is later set to None.
///
/// ## Arguments
///
/// * `interface_name: &str` - The name of the interface to investigate.
/// * `mut input: impl Read` - Any Argument which implements the `Read` trait. In this case it is a `fs::File` object.
///
/// ## Returns
///
/// * `Ok(u32)` - If the entry for interface speed, in *`Mbps`*, can be read.
/// * `Err(String)` - If the file cannot be read, indicating loopback or wireless devices, or the content is `-1`, if the interface is disabled, an Err is returned.
fn collect_interface_speed(interface_name: &str, mut input: impl Read) -> Result<u32, String> {
    let mut network_speed: String = String::new();

    match input.read_to_string(&mut network_speed) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("\x1b[33m[warning]\x1b[0m Unable to open speed file for interface '{}'. This may happen for the loopback or wireless devices. ({}))", interface_name, err));
        }
    };
    network_speed = network_speed.trim().replace("\n", "");

    if network_speed == "-1" {
        return Err(format!(
            "\x1b[36m[info]\x1b[0m No interface speed known for '{}'. The interface might be disabled.",
            interface_name
        ));
    }

    let interface_speed: u32 =
        network_speed
            .parse::<u32>()
            .map_err(|e: std::num::ParseIntError| {
                format!(
                    "\x1b[31m[error]\x1b[0m Failed to parse speed as u32 for interface '{}': {}!",
                    interface_name, e
                )
            })?;

    Ok(interface_speed)
}

#[cfg(test)]
mod network_collector_tests {
    use super::*;
    use mockall::predicate::*;

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
                    assert_eq!(message, "\x1b[31m[error]\x1b[0m No network interfaces found!".to_string());
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
                        "\x1b[31m[error]\x1b[0m No network interfaces to process".to_string()
                    );
                }
            },
        }
    }

    /// Test if the `collect_network_speed` function returns an Ok(speed) when the file contains a valid String.
    #[test]
    fn test_collect_network_speed_with_known_speed() {
        let mock_speed: String = String::from("1000");
        let interface_name: &str = "eth0";

        // Test 1: Interface speed is known
        let result: Result<u32, String> =
            collect_interface_speed(interface_name, mock_speed.as_bytes());
        assert_eq!(
            result,
            Ok(1000),
            "Test Scenario Failed (1): collect_interface_speed did not return Ok() despite supplying a correct speed value (1000)!");
    }

    /// Test if the `collect_network_speed` function returns an Err(String) with an expected message, when the interface
    /// is deactivated (speed = -1).
    #[test]
    fn test_collect_network_speed_with_deactivated_interface() {
        // Test 2: Interface deactivated (Speed = -1)
        let mock_speed = String::from("-1");
        let interface_name: &str = "eth0";

        let result: Result<u32, String> =
            collect_interface_speed(interface_name, mock_speed.as_bytes());
        assert_eq!(
            result,
            Err(format!(
                "\x1b[36m[info]\x1b[0m No interface speed known for '{}'. The interface might be disabled.",
                interface_name
            )),
            "Test Scenario Failed (2): No error was raised by collect_interface_speed when passing speed = -1. The function did not identify the interface as disabled!"
        );
    }

    /// Test if the `collect_network_speed` function returns an Err(String) with an expected message, if the contents
    /// of the speed file cannot be parsed into a `u32`.
    #[test]
    fn test_collect_network_speed_parsing_error() {
        // Test 3: Parsing error (invalid contents of speed file / empty speed file)
        let mock_speed: String = String::new();
        let interface_name: &str = "eth0";
        let result: Result<u32, String> =
            collect_interface_speed(interface_name, mock_speed.as_bytes());
        assert_eq!(
            result,
            Err(format!(
                    "\x1b[31m[error]\x1b[0m Failed to parse speed as u32 for interface '{}': cannot parse integer from empty string!",
                    interface_name
                )),
            "Test Scenario Failed (3): No error was raised by collect_interface_speed when trying to parse an empty string into u32!"
        );
    }

    /// Test whether the `build_interface_file_from_name` function returns an `Err` when given a nonexistent interface.
    #[test]
    fn test_build_interface_file_from_name_no_file() {
        // Test Scenario 1: Return Error when speed file does not exist.
        let result: Result<fs::File, String> = build_interface_file_from_name("noneexistent");
        assert!(
            result.is_err(),
            "Test Scenario Failed (1): build_interface_file_from_name does not return an error on nonexisting interface!"
        );
    }

    /// Test whether the `build_interface_file_from_name` function returns an `Ok` when given an existing interface.
    #[test]
    fn test_build_interface_file_from_name_ok() {
        // Test Scenario 2: Return Ok for file found.
        let result: Result<fs::File, String> = build_interface_file_from_name("lo");
        assert!(
            result.is_ok(),
            "Test Scenario Failed (2): build_interface_file_from_name does not return Ok(file) despite the speed file existing!"
        );
    }
}
