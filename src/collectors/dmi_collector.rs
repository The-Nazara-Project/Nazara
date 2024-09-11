//! ## Dmi Collector Module
//!
//! This module provides logic to collect and process system information by using dmidecode.
//!

/*
 * TODO:
 * 1. Implement Error checking.
 * 2. Check netbox api documentation for correct information type.
 * */

use super::collector_exceptions::CollectorError;
use super::util::{find_table, split_output};
use serde::{Deserialize, Serialize};
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
    pub arch: Option<String>,
}

/// List of possible system parameters to collect dmi information from.
///
/// ## Members
///
/// * `system-manufacturer`
/// * `system-product-name`
/// * `system-uuid`
/// * `system-serial-number`
const POSSIBLE_SYSTEM_PARAMETERS: [&str; 4] = [
    "system-manufacturer",
    "system-product-name",
    "system-uuid",
    "system-serial-number",
];

/// List of possible chassis parameters to collect dmi information from.
///
/// ## Members
///
/// * `chassis-type`
/// * `chassis-asset-tag`
/// * `chassis-serial-number`
const POSSIBLE_CHASSIS_PARAMETERS: [&str; 3] =
    ["chassis-type", "chassis-asset-tag", "chassis-serial-number"];

/// Construct [DmiInformation](struct.DmiInformation) out of the collected information.
///
/// # Returns
///
/// An instance of the DmiInformation struct containing the collected system, chassis and cpu information.
pub fn construct_dmi_information() -> DmiInformation {
    /*
     * Return a new instance of DmiInformation joining all collected information.
     *
     * */
    let dmi_information: DmiInformation = DmiInformation {
        system_information: dmidecode_system(DefaultDmiDecodeInformation {}),
        chassis_information: dmidecode_chassis(DefaultDmiDecodeInformation {}),
        cpu_information: dmidecode_cpu(DefaultDmiDecodeTable {}),
    };
    return dmi_information;
}

/// Represents the `get_dmidecode_table` function which, when called, returns the contents of the requested dmi table.
trait DmiDecodeTable {
    fn get_dmidecode_table(dmidecode_table: i32) -> String;
}

/// Default implementation of the `get_dmidecode_table` functionality.
struct DefaultDmiDecodeTable;

/// Default implementation of the `get_dmidecode_table` function.
impl DmiDecodeTable for DefaultDmiDecodeTable {
    /// Executes `dmidecode` with a given table number.
    ///
    /// ## Arguments
    ///
    /// * dmidecode_table: i32 - The index of the table to return.
    ///
    /// ## Returns
    ///
    /// * String - The content of the dmi table as a string.
    ///
    /// ## Panics
    ///
    /// If `dmidecode -t <dmidecode_table>` fails, the function panics.
    fn get_dmidecode_table(dmidecode_table: i32) -> String {
        /*
         * Collect DMI information from System.
         *
         * This function executes the dmidecode command for the table type provided.
         */
        let output: Output = match Command::new("sudo")
            .arg("dmidecode")
            .arg("-t")
            .arg(dmidecode_table.to_string())
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                let err = CollectorError::DmiError(e.to_string());
                err.abort(None);
            }
        };

        // Read the output of the command
        return String::from_utf8_lossy(&output.stdout).to_string();
    }
}

/// Implements a trait representing the `get_dmidecode_information` function.
///
/// This is needed mainly for testing `dmidecode_system` and `dmidecode_chassis` so we can implement two versions of this
/// function. One with the real implementation and one returning the expected test values.
trait DmiDecodeInformation {
    fn get_dmidecode_information(parameter: &str) -> String;
}

/// Empty struct which implements the `DmiDecodeInformation` trait.
struct DefaultDmiDecodeInformation;

/// Implement the `DmiDecodeInformation` trait for the `DefaultDmiDecodeInformation` struct.
///
/// This represents the default implementation of the `get_dmidecode_information function.
impl DmiDecodeInformation for DefaultDmiDecodeInformation {
    /// Execute `dmidecode -s <PARAMETER>` where `<PARAMETER>` is the system property to look for.
    ///
    /// This method of obtaining system information is quicker than the other approach with crawling through the dmi tables.
    /// It is only suitable for basic system information such as BIOS, platform and chassis information.
    ///
    /// ## Arguments
    ///
    /// * `parameter: &str` - The system property to look for.
    ///
    /// ## Returns
    ///
    /// * `String` - The system property
    ///
    /// ## Panics
    ///
    /// If the `dmidecode` execution fails, a `UnableToCollectDataError` is raised and the function panics.
    fn get_dmidecode_information(parameter: &str) -> String {
        let output: Output = match Command::new("sudo")
            .arg("dmidecode")
            .arg("-s")
            .arg(parameter)
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                let err = CollectorError::DmiError(e.to_string());
                err.abort(None);
            }
        };
        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
}

/// Collect general system information and construct a new [SystemInformation](struct.SystemInformation) object from it.
///
/// This function call the `get_dmidecode_information` function for each parameter required for system information.
///
/// If the system-manufacturer returns `QEMU` it is assumed that the machine is a virtual machine and the `is_virtual`
/// field of [SystemInformation](struct.SystemInformation) is updated accordingly.
///
/// This is important as Virtual Machines and Physical Machines are treated differently by NetBox and are registered at
/// different URLs.
///
/// Note: Fields *can* be empty strings if a parameter, that is being searched for, is not recognized in the match
/// statement.
///
/// # Arguments
///
/// * `_param: T` - Receives an Object which implements the `DmiDecodeInformation` trait. Can be either the default
/// implementation or a test implementation which returns expected values.
///
/// # Returns
///
/// * `system_information: SystemInformation`- A SystemInformation object.
fn dmidecode_system<T: DmiDecodeInformation>(_param: T) -> SystemInformation {
    println!("Collecting system information...");
    let mut system_information: SystemInformation = SystemInformation {
        vendor: String::new(),
        model: String::new(),
        uuid: String::new(),
        serial: String::new(),
        is_virtual: false,
    };

    for parameter in POSSIBLE_SYSTEM_PARAMETERS.iter() {
        match *parameter {
            "system-manufacturer" => {
                system_information.vendor = T::get_dmidecode_information(*parameter);

                if system_information.vendor == "QEMU" {
                    system_information.is_virtual = true;
                }
            }
            "system-product-name" => {
                system_information.model = T::get_dmidecode_information(*parameter)
            }
            "system-uuid" => system_information.uuid = T::get_dmidecode_information(*parameter),
            "system-serial-number" => {
                system_information.serial = T::get_dmidecode_information(*parameter)
            }
            _ => {
                println!(
                    "\x1b[36m[info]\x1b[0m Parameter {} not supported therefore not collected.",
                    parameter
                );
            }
        }
    }
    println!("\x1b[32m[success]\x1b[0m System information collection completed.");
    return system_information;
}

/// Construct a ChassisInformation object by parsing the content of dmi chassis table.
///
/// # Arguments
///
/// * `_param: T` - Receives an Object which implements the `DmiDecodeInformation` trait. Can be either the default
/// implementation or a test implementation which returns expected values.
///
/// # Returns
///
/// A ChassisInformation object.
fn dmidecode_chassis<T: DmiDecodeInformation>(_param: T) -> ChassisInformation {
    println!("Collecting chassis information...");
    let mut chassis_information: ChassisInformation = ChassisInformation {
        chassis_type: String::new(),
        asset: String::new(),
        chassis_serial: String::new(),
    };

    for parameter in POSSIBLE_CHASSIS_PARAMETERS.iter() {
        match *parameter {
            "chassis-type" => {
                chassis_information.chassis_type = T::get_dmidecode_information(*parameter)
            }
            "chassis-asset-tag" => {
                chassis_information.asset = T::get_dmidecode_information(*parameter)
            }
            "chassis-serial-number" => {
                chassis_information.chassis_serial = T::get_dmidecode_information(*parameter)
            }
            _ => {
                println!(
                    "\x1b[36m[info]\x1b[0m Parameter {} not supported. Therefore will not be collected.",
                    parameter
                );
            }
        }
    }
    println!("\x1b[32m[success]\x1b[0m Chassis information collection completed.");
    return chassis_information;
}

/// Construct a CpuInformation object by parsing the content of dmi cpu table.
///
/// Captures the output of `dmidecode -t 4` and processes the table to find the required values.
///
/// # Returns
///
/// A CpuInformation object.
fn dmidecode_cpu<T: DmiDecodeTable>(_param: T) -> CpuInformation {
    println!("Collecting CPU information...");
    let output: String = T::get_dmidecode_table(4);
    let output_split: Split<'_, &str> = output.split("\n");
    let mut split: Vec<&str>;

    let mut cpu_information: CpuInformation = CpuInformation {
        version: String::new(),
        core_count: String::new(),
        cores_enabled: String::new(),
        thread_count: String::new(),
        max_speed: String::new(),
        voltage: String::new(),
        status: String::new(),
        arch: None,
    };

    let mut table_found: bool = false;

    for part in output_split {
        if !table_found {
            table_found = find_table("Processor Information", part);
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
            None => println!("\x1b[36m[info]\x1b[0m Key not found at this location..."),
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("\x1b[36m[info]\x1b[0m Value not found at this location..."),
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
    cpu_information.arch = match get_architecture() {
        Ok(arch) => Some(arch),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };
    println!("\x1b[32m[success]\x1b[0m CPU information collection completed.");
    return cpu_information;
}

/// Gets the architecture name through `uname`.
///
/// *This will override any information entered in the config file's `platform` field!*
///
/// # Returns
/// - Ok(arch_name): `string` - Name of the CPU architecture.
/// - Err(UnableToCollectDataError)
fn get_architecture() -> Result<String, CollectorError> {
    println!("Running uname to collect CPU architecture...");
    let output = match Command::new("uname").arg("-p").output() {
        Ok(output) => output,
        Err(e) => {
            let err = CollectorError::UnableToCollectDataError (
                String::from(format!("\x1b[31m[error]\x1b[0m An error occured while attempting to execute `uname -p`! {}", e))
			);
            return Err(err);
        }
    };

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
pub mod dmi_collector_tests {
    use super::*;
    use std::any::Any;

    /// Check a given value's type for being String or not.
    ///
    /// ## Returns
    ///
    /// * `bool` - True/False depending on if the given value is a String type.
    fn is_string(value: &dyn Any) -> bool {
        value.is::<String>()
    }

    /// Check a given value's type for being SystemInformation or not.
    ///
    /// ## Returns
    ///
    /// * `bool` - True/False depending on if the given value is a String type.
    fn is_system_information(value: &dyn Any) -> bool {
        value.is::<SystemInformation>()
    }

    /// Tests whether the `get_dmidecode_information` function panics when it tries to execute `dmidecode -s` with an
    /// invalid parameter.
    #[test]
    #[should_panic]
    fn test_get_dmidecode_information_panics() {
        let result: Result<String, Box<dyn Any + Send>> = std::panic::catch_unwind(|| {
            DefaultDmiDecodeInformation::get_dmidecode_information("invalid")
        });

        assert!(
            result.is_err(),
            "Test failure: get_dmidecode_information did not panic when supplied with an invalid
        argument"
        );
    }

    /// Test that `get_dmidecode_information` does not return an empty String. Validating that the given parameter is
    /// valid.
    #[test]
    fn test_get_dmidecode_information_ok() {
        let result: String =
            DefaultDmiDecodeInformation::get_dmidecode_information("system-manufacturer");

        assert!(
            is_string(&result),
            "Test failure: get_dmidecode_information did not return a String type!"
        );
        assert!(
            !result.is_empty(),
            "Test failure: get_dmidecode_information did return an empty string despite supplying a valid parameter!"
        );
    }

    struct MockDmiDecodeInformation;

    impl DmiDecodeInformation for MockDmiDecodeInformation {
        fn get_dmidecode_information(parameter: &str) -> String {
            match parameter {
                "system-manufacturer" => "TEST".to_string(),
                "system-product-name" => "TestMachine".to_string(),
                "system-uuid" => "123456-123-1222".to_string(),
                "system-serial-number" => "123456789".to_string(),
                _ => String::new(),
            }
        }
    }

    #[test]
    fn test_dmidecode_system() {
        let expected_vendor: &str = "TEST";
        let expected_model: &str = "TestMachine";
        let expected_uuid: &str = "123456-123-1222";
        let expected_serial: &str = "123456789";

        // Mock existing get_dmidecode_information function return the expected parameters

        let system_information: SystemInformation = dmidecode_system(MockDmiDecodeInformation {});

        assert!(
            is_system_information(&system_information),
            "Test Failure: `dmidecode_system` did not return instance of `SystemInformation`!"
        );
        assert_eq!(system_information.vendor, expected_vendor);
        assert_eq!(system_information.model, expected_model);
        assert_eq!(system_information.uuid, expected_uuid);
        assert_eq!(system_information.serial, expected_serial);
    }

    struct MockDmiDecodeTable;

    impl DmiDecodeTable for MockDmiDecodeTable {
        fn get_dmidecode_table(_dmidecode_table: i32) -> String {
            let return_value: String = String::from("Processor Information\n\tSocket Designation: FP5\n\tType: Central Processor\n\tFamily: Zen\n\tManufacturer: Advanced Micro Devices, Inc\n\tVersion: AMD Ryzen 7 PRO 3700 w/ Radeon Vega Mobile Gfx\n\tVoltage: 1.2 V\n\tMax Speed: 4000 MHz\n\tCurrent Speed: 2300 MHz\n\tStatus: Populated, Enabled\n\tCore Count: 4\n\tCore Enabled: 4\n\tThread Count: 8");
            return return_value;
        }
    }

    #[test]
    fn test_dmidecode_cpu() {
        let expected: CpuInformation = CpuInformation {
            version: "AMD Ryzen 7 PRO 3700 w/ Radeon Vega Mobile Gfx".to_string(),
            core_count: "4".to_string(),
            cores_enabled: "4".to_string(),
            thread_count: "8".to_string(),
            max_speed: "4000 MHz".to_string(),
            voltage: "1.2 V".to_string(),
            status: "Populated, Enabled".to_string(),
            arch: Some("x86_64".to_string()),
        };

        let result = dmidecode_cpu(MockDmiDecodeTable {});

        assert_eq!(expected.version, result.version);
        assert_eq!(expected.core_count, result.core_count);
        assert_eq!(expected.cores_enabled, result.cores_enabled);
        assert_eq!(expected.thread_count, result.thread_count);
        assert_eq!(expected.max_speed, result.max_speed);
        assert_eq!(expected.voltage, result.voltage);
        assert_eq!(expected.status, result.status);
        assert_eq!(expected.arch, result.arch);
    }
}
