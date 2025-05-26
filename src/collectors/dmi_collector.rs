//! ## Dmi Collector Module
//!
//! This module provides logic to collect and process system information by using dmidecode.

use super::collector_exceptions::CollectorError;
use dmidecode::{Structure, processor::ProcessorType};
use serde::Serialize;
use std::{error::Error, fs};

/// ## DmiInformation
///
/// ### Members
///
/// * system_information: [`SystemInformation`](struct.SystemInformation) - Basic system information.
/// * chassis_information: [`ChassisInformation`](struct.ChassisInformation) - The type of asset.
/// * cpu_information: [`CpuInformation`](struct.CpuInformation) - Information about the processor(s).
#[derive(Serialize, Debug)]
pub struct DmiInformation {
    pub system_information: SystemInformation,
    pub chassis_information: ChassisInformation,
    pub cpu_information: CpuInformation,
}

/// ## SystemInformation
///
/// Basic information of the machine extracted from dmidecode.
///
/// ### Members
///
/// * vendor: `String` - The name of the machine's manufacturer (e.g. LENOVO).
/// * model: `String` - The model number of the machine.
/// * uuid: `String` - The UUID of the device.
/// * serial: `String` - The serial number of the machine.
/// * is_virtual: `bool` - Whether the machine is a virtual machine or not.
#[derive(Serialize, Debug)]
pub struct SystemInformation {
    pub vendor: String,
    pub model: String,
    pub uuid: String,
    pub serial: String,
    pub is_virtual: bool,
}

/// ## Chassis Information
///
/// The type of asset provided by dmidecode table 3.
///
/// ### Members
///
/// * chassis_type: `String` - Type of the chassis.
/// * asset: `String`- Type of asset.
/// * chassis_serial: `Serial` - Serial number of the chassis.
#[derive(Serialize, Debug)]
pub struct ChassisInformation {
    pub chassis_type: String,
    pub asset: String,
    pub chassis_serial: String,
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
/// * arch: `String` - The architecture of the CPU (x86_64, etc).
#[derive(Serialize, Debug)]
pub struct CpuInformation {
    pub version: String,
    pub core_count: String,
    pub cores_enabled: String,
    pub thread_count: String,
    pub max_speed: String,
    pub voltage: String,
    pub status: String,
}

/// Construct [DmiInformation](struct.DmiInformation) instance by parsing SMBIOS and DMI tables
/// from the system.
///
/// This function reads raw data from SMBIOS entry points and DMI tables and parses it into these
/// structs:
///
/// - [SystemInformation](struct.SystemInformation) (vendor, model, sertial number, UUID, etc.)
/// - [ChassisInformation](struct.ChassisInformation) (chassis type, asset tag, chassis serial no,
/// etc.)
/// - [CpuInformation](struct.CpuInformation) (core count, threads, speed, voltage and status)
///
/// Only the first available structure of each required type is used. For CPUs, only the structure
/// marked as `CentralProcessor` is considered.
///
/// # Returns
///
/// An instance of the DmiInformation struct containing the collected system, chassis and cpu information.
///
/// # Errors
///
/// Returns a `Box<dyn Error>` if:
/// - The SMBIOS header or DMI table cannot be read from filesystem.
/// - The DMI entry point search fails.
/// - Any of the required structures (system, chassis, CPU) are missing or malformed.
pub fn construct_dmi_information() -> Result<DmiInformation, Box<dyn Error>> {
    println!("Collecting DMI Information...");

    // Get the SMBIOS header and DMI table from sysfs.
    let buf = fs::read("/sys/firmware/dmi/tables/smbios_entry_point")?;
    let dmi = fs::read("/sys/firmware/dmi/tables/DMI")?;
    let entry = dmidecode::EntryPoint::search(&buf)?;

    let mut system_information = None;
    let mut chassis_information = None;
    let mut cpu_information = None;

    // Iterate over the DMI tables.
    for table in entry.structures(&dmi) {
        match table? {
            Structure::System(x) => {
                system_information = Some(SystemInformation {
                    vendor: x.manufacturer.to_owned(),
                    model: x.product.to_owned(),
                    // If we have a UUID, construct one from the buffer, otherwise an empty string.
                    uuid: x
                        .uuid
                        .map_or_else(|| String::new(), |f| uuid::Uuid::from_bytes(f).to_string()),
                    serial: x.serial.to_owned(),
                    // TODO
                    is_virtual: false,
                })
            }
            Structure::Enclosure(x) => {
                chassis_information = Some(ChassisInformation {
                    chassis_type: x.enclosure_type.to_string(),
                    asset: x.asset_tag_number.to_owned(),
                    chassis_serial: x.serial_number.to_owned(),
                })
            }
            Structure::Processor(x) => {
                // There may be multiple processor tables. We only care about the CPU.
                if x.processor_type != ProcessorType::CentralProcessor {
                    continue;
                }

                cpu_information = Some(CpuInformation {
                    version: String::new(),
                    core_count: x.core_count.unwrap_or_default().to_string(),
                    cores_enabled: x.core_enabled.unwrap_or_default().to_string(),
                    thread_count: x.thread_count.unwrap_or_default().to_string(),
                    max_speed: x.max_speed.to_string(),
                    voltage: x.voltage.to_string(),
                    // Fancy formatting for bitflags.
                    status: format!("{:?}", x.status),
                });
            }
            _ => continue,
        }
    }

    Ok(DmiInformation {
        system_information: system_information.ok_or(CollectorError::UnableToCollectDataError(
            "Couldn't collect system information".to_owned(),
        ))?,
        chassis_information: chassis_information.ok_or(
            CollectorError::UnableToCollectDataError(
                "Couldn't collect chassis information".to_owned(),
            ),
        )?,
        cpu_information: cpu_information.ok_or(CollectorError::UnableToCollectDataError(
            "Couldn't collect CPU information".to_owned(),
        ))?,
    })
}

#[cfg(test)]
pub mod dmi_collector_tests {
    // TODO
}
