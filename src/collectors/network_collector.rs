use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;

pub struct network_information {
    name: String,
    addr: String,
    netmask: String,

}


fn collect_network_information() -> Vec<NetworkInterface> {
    /*
     * Collect information about all available network interfaces.
     *
     * Will return a Vector containing a JSON-like data format.
     */
    let network_interfaces: Vec<NetworkInterface> = NetworkInterface::show().unwrap();

    return network_interfaces;
}
