mod collectors;
pub mod configuration;

use collectors::{dmi_collector, network_collector};
use configuration::config_parser::set_up_configuration;

fn main() {
    let output: dmi_collector::DmiInformation = dmi_collector::construct_dmi_information();
    println!("{:#?}", output);

    let output2 = network_collector::construct_network_information().unwrap();

    println!("{:#?}", output2);

    let config = match set_up_configuration() {
        Ok(conf) => conf,
        Err(err) => {
            panic!("{}", err)
        }
    };

    println!("{:#?}", config);
}
