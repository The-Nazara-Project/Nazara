/*
 * TODO:
 * 1. Implement Error checking.
 * 2. Check netbox api documentation for correct information type.
 * 3. REMOVE DEBUG PRINT STATEMENTS.
 * */

use std::{str::Split, process::{Command, Output}};
use super::util::{find_table, split_output};

// Fully collected DMI information
#[derive(Debug)]
pub struct DmiInformation {
   system_information: SystemInformation,
   chassis_information: ChassisInformation,
   cpu_information: CpuInformation,
}

// General system information
#[derive(Debug)]
pub struct SystemInformation {
    vendor: String,
    model: String,
    uuid: String,
    serial: String,
}

#[derive(Debug)]
pub struct ChassisInformation {
    asset: String,
}

// CPU Information
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

fn dmidecode_system() -> SystemInformation {
    /*
     * Collect general system information.
     *
     * This function calls the execute_dmidecode function and reads the output the command
     * provides.
     * The output is processed and the required information is extracted from the string and saved into a
     * system_information instance.
     * */
    let output: String = execute_dmidecode(1);
    let output_split: Split<'_, &str> = output.split("\n");
    let mut split: Vec<&str> = Vec::new();

    let mut system_information: SystemInformation = SystemInformation {
        vendor: String::new(),
        model: String::new(),
        uuid: String::new(),
        serial: String::new(),
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
            None => println!("Info: Key not found at this location...")
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("Info: Value not found at this location...")
        }
        match key.as_str() {
            "Manufacturer" => {
                system_information.vendor = value.trim().to_string();
            },
            "Product Name" => {
                system_information.model = value.trim().to_string();
            },
            "Serial Number" => {
                system_information.serial = value.trim().to_string();
            },
            "UUID" => {
                system_information.uuid = value.trim().to_string();
            },
            _=> {
                continue;
            },
        }
    }
    return system_information
}


fn dmidecode_chassis() -> ChassisInformation {
    /*
     * Collect Chassis information.
     *
     * Works like dmidecode_system with only one key to be checked.
     * */
    let output: String = execute_dmidecode(3);
    let output_split: Split<'_, &str> = output.split("\n");
    let mut split: Vec<&str> = Vec::new();

    let mut chassis_information: ChassisInformation = ChassisInformation {asset: String::new()};

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
            None => println!("Info: Key not found at this location...")
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("Info: Value not found at this location...")
        }
        match key.as_str() {
            "Asset Tag" => {
                chassis_information.asset = value.trim().to_string();
            },
            _=> {
                continue;
            },
        }
    }

    return chassis_information;
}


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
            None => println!("Info: Key not found at this location...")
        }
        match split.get(1) {
            Some(x) => {
                value = x.to_string();
            }
            None => println!("Info: Value not found at this location...")
        }
        match key.as_str() {
            "Version" => {
                cpu_information.version = value.trim().to_string();
            },
            "Core Count" => {
                cpu_information.core_count = value.trim().to_string();
            },
            "Core Enabled" => {
                cpu_information.cores_enabled = value.trim().to_string();
            },
            "Thread Count" => {
                cpu_information.thread_count = value.trim().to_string();
            },
            "Max Speed" => {
                cpu_information.max_speed = value.trim().to_string();
            },
            "Voltage" => {
                cpu_information.voltage = value.trim().to_string();
            },
            "Status" => {
                cpu_information.status = value.trim().to_string();
            },
            _=> {
                continue;
            },
        }
    }
    return cpu_information
}
