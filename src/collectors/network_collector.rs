use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use std::net::IpAddr;

pub struct NetworkInformation {
    name: String,
    v4ip: IpAddr,
    v4broadcast: Option<IpAddr>,
    v4netmask: Option<IpAddr>,
    v6ip: IpAddr,
    v6broadcast: Option<IpAddr>,
    v6netmask: Option<IpAddr>,
    mac_addr: Option<String>,
    index: u32,
}

pub fn collect_network_information() -> Vec<NetworkInterface> {
    /*
     * Collect information about all available network interfaces.
     *
     * Will return a Vector containing a JSON-like data format.
     */
    let network_interfaces: Vec<NetworkInterface> = NetworkInterface::show().unwrap();

    return network_interfaces;
}

pub fn construct_network_information() -> Vec<NetworkInformation> {
    /*
     * Deconstruct NetworkInterface vector and construct NetworkInformation objects from it.
     */

    let raw_information: Vec<NetworkInterface> = collect_network_information();
    let interfaces: Vec<NetworkInformation> = Vec::new();

    for network_interface in raw_information {
        match Some(&network_interface.addr) {
            // TODO: Check if reference is correct here
            Some(v) => {
                let network_information: NetworkInformation = NetworkInformation {
                    name: network_interface.name,
                    v4ip: network_interface.addr[0].ip(), // if reference in match not correct, this will throw errros.
                    v4broadcast: network_interface.addr[0].broadcast(),
                    v4netmask: network_interface.addr[0].netmask(),
                    v6ip: network_interface.addr[1].ip(),
                    v6broadcast: network_interface.addr[1].broadcast(),
                    v6netmask: network_interface.addr[1].netmask(),
                    mac_addr: network_interface.mac_addr,
                    index: network_interface.index,
                };
            }
            None => {}
        }
    }

    return interfaces;
}
