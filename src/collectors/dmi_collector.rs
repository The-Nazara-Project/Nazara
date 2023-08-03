//! ## Dmi Collector Module
//!
//! This module provides logic to collect and process system information by using dmidecode.
//!

/*
 * TODO:
 * 1. Implement Error checking.
 * 2. Check netbox api documentation for correct information type.
 * 3. REMOVE DEBUG PRINT STATEMENTS.
 * */

use super::util::{find_table, split_output};
use std::{
    process::{Command, Output},
    str::Split,
};

/// ## DmiInformation
///
/// ### Members
///
/// * system_information: [`SystemInformation`](struct.SystemInformation) - Basic system information.
/// * chassis_information: [`ChassisInformation`](struct.ChassisInformation) - The type of asset.
/// * cpu_information: [`CpuInformation`](struct.CpuInformation) - Information about the processor(s).
#[derive(Debug)]
pub struct DmiInformation {
    system_information: SystemInformation,
    chassis_information: ChassisInformation,
    cpu_information: CpuInformation,
}

/// ## SystemInformation
///
/// Basic information of the machine extracted from dmidecode table 1.
///
/// ### Members
///
/// * vendor: `String` - The name of the machine's manufacturer (e.g. LENOVO).
/// * model: `String` - The model number of the machine.
/// * uuid: `String` - The UUID of the device.
/// * serial: `String` - The serial number of the machine.
/// * is_virtual: `bool` - Whether the machine is a virtual machine or not.
#[derive(Debug)]
pub struct SystemInformation {
    vendor: String,
    model: String,
    uuid: String,
    serial: String,
    is_virtual: bool,
}

/// ## Chassis Information
///
/// The type of asset provided by dmidecode table 3.
///
/// ### Members
///
/// * asset: `String`- Type of asset.
#[derive(Debug)]
pub struct ChassisInformation {
    asset: String,
}

/// ## CpuInformation
///
/// Information about the CPU(s) of the system.
///
/// ### Members
///
/// * versions: `String` - The type of CPU(s) used.
/// * core_count: `String` - The number of cores.
/// * cores_enabled: `String` - The number of enabled cores.
/// * thread_count: `String` - The number of threads.
/// * max_speed: `String`- The maximum speed of the CPU.
/// * voltage: `String` - The voltage the CPU runs at.
/// * status: `String` - Shows if the socket is enabled/disabled and populated/empty.
#[derive(Debug)]
pub struct CpuInformation {
    version: String,
    core_count: String,
    cores_enabled: String,
    thread_count: String,
    max_speed: String,
    voltage: String,
    status: String,
}

/// Executes `dmidecode` with a given table number.
///
/// ## Arguments
///
/// * dmidecor_table: i32 - The index of the table to return.
///
/// ## Returns
///
/// * String - The content of the dmi table as a string.
fn execute_dmidecode(dmidecode_table: i32) -> String {
    /*
     * Collect DMI information from System.
     *
     * This function executes the dmidecode command for the table type provided.
     */
    let output: Output = Command::new("sudo")
        .arg("dmidecode")
        .arg("-t")
        .arg(dmidecode_table.to_string())
        .output()
        .expect("Failed to execute command");

    // Read the output of the command
    return String::from_utf8_lossy(&output.stdout).to_string();
}

/// Construct [DmiInformation](struct.DmiInformation) out of the collected information.
///
/// # Returns
///
/// An instance of the DmiInformation struct containing the collected system, chassis and cpu information.
pub fn dmidecode() -> DmiInformation {
    /*
     * Return a new instance of DmiInfomration joining all collected information.
     *
     * */
    let dmi_information: DmiInformation = DmiInformation {
        system_information: dmidecode_system(),
        chassis_information: dmidecode_chassis(),
        cpu_information: dmidecode_cpu(),
    };
    return dmi_information;
}

/// Construct a SystemInformation object by parsing the content of dmi system table.
///
/// # Returns
///
/// A SystemInformation object.
fn dmidecode_system() -> SystemInformation {
    /*
     * Collect general system information.
     *
     * This function calls the execute_dmidecode function and reads the output the command
     * provides.
     * The output is processed and the required information is extracted from the string and saved into a
     * system_information instance.
     *
     * If the vendor found is QEMU it is assumed that the machine is a virtual machine.
     * This is important as Virtual Machines and Physical Machines are treated differently by NetBox and registered at
     * different URLs.
     * */
    let output: String = execute_dmidecode(1);
    let output_split: Split<'_, &str> = output.split("\n");
    let mut split: Vec<&str> = Vec::new();

    let mut system_information: SystemInformation = SystemInformation {
        vendor: String::new(),
        model: String::new(),
        uuid: String::new(),
        serial: String::new(),
        is_virtual: false,
    };

    let mut table_found: bool = false;

    for part in output_split {
        if !table_found {
            table_found = find_table("System Information", part);
        }

        let split_output: Result<Vec<&str>, &str> = split_output(part);

        match split_output {
            Ok(_) => split = split_output.unwrap(),
            Err(_) => continue,
        }

        let mut key: String = String::new();
        let mut value: String = String::new();

        match split.get(0) {
            Some(x) => {
                key = x.to_string();
            }
            None => println!("Info: Key not found at this location..."),
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("Info: Value not found at this location..."),
        }
        match key.as_str() {
            "Manufacturer" => {
                system_information.vendor = value.trim().to_string();

                if system_information.vendor == "QEMU" {
                    system_information.is_virtual = true;
                }
            }
            "Product Name" => {
                system_information.model = value.trim().to_string();
            }
            "Serial Number" => {
                system_information.serial = value.trim().to_string();
            }
            "UUID" => {
                system_information.uuid = value.trim().to_string();
            }
            _ => {
                continue;
            }
        }
    }
    return system_information;
}

/// Construct a ChassisInformation object by parsing the content of dmi chassis table.
///
/// # Returns
///
/// A ChassisInformation object.
fn dmidecode_chassis() -> ChassisInformation {
    /*
     * Collect Chassis information.
     *
     * Works like dmidecode_system with only one key to be checked.
     * */
    let output: String = execute_dmidecode(3);
    let output_split: Split<'_, &str> = output.split("\n");
    let mut split: Vec<&str> = Vec::new();

    let mut chassis_information: ChassisInformation = ChassisInformation {
        asset: String::new(),
    };

    let mut table_found: bool = false;

    for part in output_split {
        if !table_found {
            table_found = find_table("System Information", part);
        }

        let split_output: Result<Vec<&str>, &str> = split_output(part);

        match split_output {
            Ok(_) => split = split_output.unwrap(),
            Err(_) => continue,
        }

        let mut key: String = String::new();
        let mut value: String = String::new();

        match split.get(0) {
            Some(x) => {
                key = x.to_string();
            }
            None => println!("Info: Key not found at this location..."),
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("Info: Value not found at this location..."),
        }
        match key.as_str() {
            "Asset Tag" => {
                chassis_information.asset = value.trim().to_string();
            }
            _ => {
                continue;
            }
        }
    }

    return chassis_information;
}

/// Construct a CpuInformation object by parsing the content of dmi cpu table.
///
/// # Returns
///
/// A CpuInformation object.
fn dmidecode_cpu() -> CpuInformation {
    /*
     * Collect CPU information.
     *
     * Works exactly like above by calling execute_dmidecode and processing its output.
     * */
    let output: String = execute_dmidecode(4);
    let output_split: Split<'_, &str> = output.split("\n");
    let mut split: Vec<&str> = Vec::new();

    let mut cpu_information: CpuInformation = CpuInformation {
        version: String::new(),
        core_count: String::new(),
        cores_enabled: String::new(),
        thread_count: String::new(),
        max_speed: String::new(),
        voltage: String::new(),
        status: String::new(),
    };

    let mut table_found: bool = false;

    for part in output_split {
        if !table_found {
            table_found = find_table("System Information", part);
        }

        let split_output: Result<Vec<&str>, &str> = split_output(part);

        match split_output {
            Ok(_) => split = split_output.unwrap(),
            Err(_) => continue,
        }

        let mut key: String = String::new();
        let mut value: String = String::new();

        match split.get(0) {
            Some(x) => {
                key = x.to_string();
            }
            None => println!("Info: Key not found at this location..."),
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("Info: Value not found at this location..."),
        }
        match key.as_str() {
            "Version" => {
                cpu_information.version = value.trim().to_string();
            }
            "Core Count" => {
                cpu_information.core_count = value.trim().to_string();
            }
            "Core Enabled" => {
                cpu_information.cores_enabled = value.trim().to_string();
            }
            "Thread Count" => {
                cpu_information.thread_count = value.trim().to_string();
            }
            "Max Speed" => {
                cpu_information.max_speed = value.trim().to_string();
            }
            "Voltage" => {
                cpu_information.voltage = value.trim().to_string();
            }
            "Status" => {
                cpu_information.status = value.trim().to_string();
            }
            _ => {
                continue;
            }
        }
    }
    return cpu_information;
}
