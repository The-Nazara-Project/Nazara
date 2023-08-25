mod collectors;
pub mod configuration;

use collectors::{dmi_collector, network_collector};
use configuration::config_parser::*;

fn main() {
    let output: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();
    println!("{:#?}", output);

    let output2 = network_collector::construct_network_information();

    println!("{:#?}", output2);

    let result = ConfigData::initialize_config_file().is_ok();

    if result {
        println!("Config file created!")
    }
}
