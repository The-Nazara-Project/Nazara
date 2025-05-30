//! This module provides logic to collect and process system information by using SMBIOS and DMI tables.

use super::errors::CollectorError;
use dmidecode::{Structure, processor::ProcessorType};
use serde::Serialize;
use std::{error::Error, fs};

#[derive(Serialize, Debug)]
pub struct DmiInformation {
    /// Basic system information.
    pub system_information: SystemInformation,
    /// The type of asset.
    pub chassis_information: ChassisInformation,
    /// Information about the processor(s).
    pub cpu_information: CpuInformation,
}

/// Basic information of the machine extracted from dmidecode.
#[derive(Serialize, Debug)]
pub struct SystemInformation {
    /// The name of the machine's manufacturer (e.g. LENOVO).
    pub vendor: String,
    /// The model number of the machine.
    pub model: String,
    /// The UUID of the device.
    pub uuid: String,
    /// The serial number of the machine.
    pub serial: String,
    /// Whether the machine is a virtual machine or not.
    pub is_virtual: bool,
}

/// The type of asset provided by dmidecode table 3.
#[derive(Serialize, Debug)]
pub struct ChassisInformation {
    /// Type of the chassis.
    pub chassis_type: String,
    /// Type of asset.
    pub asset: String,
    /// Serial number of the chassis.
    pub chassis_serial: String,
}

/// Information about the CPU(s) of the system.
#[derive(Serialize, Debug)]
pub struct CpuInformation {
    /// The type of CPU(s) used.
    pub version: String,
    /// The number of cores.
    pub core_count: String,
    /// The number of enabled cores.
    pub cores_enabled: String,
    /// The number of threads.
    pub thread_count: String,
    /// The maximum speed of the CPU.
    pub max_speed: String,
    /// The voltage the CPU runs at.
    pub voltage: String,
    /// Shows if the socket is enabled/disabled and populated/empty.
    pub status: String,
}

/// Parses SMBIOS and DMI tables from the sysfs.
/// Returns an error if:
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
        let Ok(t) = table else {
            eprintln!("[warn] DMI tables contain malformed structure: {table:?}");
            continue;
        };

        match t {
            Structure::System(x) => {
                system_information = Some(SystemInformation {
                    vendor: x.manufacturer.to_owned(),
                    model: x.product.to_owned(),
                    // If we have a UUID, construct one from the buffer, otherwise an empty string.
                    uuid: x
                        .uuid
                        .map_or_else(String::new, |f| uuid::Uuid::from_bytes(f).to_string()),
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
        system_information: system_information.ok_or(CollectorError::UnableToCollectData(
            "Couldn't collect system information".to_owned(),
        ))?,
        chassis_information: chassis_information.ok_or(CollectorError::UnableToCollectData(
            "Couldn't collect chassis information".to_owned(),
        ))?,
        cpu_information: cpu_information.ok_or(CollectorError::UnableToCollectData(
            "Couldn't collect CPU information".to_owned(),
        ))?,
    })
}

// NOTE: The dmidecode crate already handles malformed DMI information. Explicit tests are not needed.
