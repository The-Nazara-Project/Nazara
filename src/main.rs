mod collectors;

use collectors::{dmi_collector, network_collector};

fn main() {
    let output: dmi_collector::DmiInformation = dmi_collector::dmidecode();
    println!("{:#?}", output);

    let output2 = network_collector::construct_network_information();

    println!("{:#?}", output2);
}
