//! ## Network Collector Module
//!
//! This module provides logic to collect and process Information about all network interfaces a device has.
//!
use super::errors::CollectorError;
use futures::TryStreamExt;
use rtnetlink::new_connection;
use rtnetlink::packet_route::address::AddressAttribute;
use rtnetlink::packet_route::link::LinkAttribute;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// ## Network Information
///
/// This object contains information about one specific network interface.
///
/// ### Members
///
/// * name - `String` containing the name of a network interface.
/// * v4ip - `IpAddr` the IPv4 address of this interface.
/// * v4broadcast - `IpAddr` the IPv4 broadcast address of this interface.
/// * v4netmask - `IpAddr` the IPv4 netmask of this interface.
/// * v6ip - `IpAddr` the IPv6 address of this interface.
/// * v6broadcast - `IpAddr` the IPv6 broadcast address of this interface.
/// * v6netmask - `IpAddr` the IPv6 netmask of this interface.
/// * mac_addr - `String` the mac address of this interface.
/// * index - `u32` the index of this interface.
/// * is_physical - `bool` whether this device is a physical or virtual interface.
/// * is_connected - `bool` whether the interface is connected. Determined by if it has an address or not.
/// * interface_speed - `u32` the speed of the interface.
#[derive(Serialize, Debug, Default)]
pub struct NetworkInformation {
    pub name: String,
    pub interface_speed: Option<u32>,
    pub v4ip: Option<IpAddr>,
    pub v4broadcast: Option<IpAddr>,
    pub v4netmask: Option<IpAddr>,
    pub v6ip: Option<IpAddr>,
    pub v6broadcast: Option<IpAddr>,
    pub v6netmask: Option<IpAddr>,
    pub mac_addr: Option<String>,
    pub index: Option<u32>,
    pub is_physical: bool,
    pub is_connected: bool,
}

/// Constructs instances of the NetworkInformation struct.
///
/// Loopback device collected by `collect_network_information` is skipped.
///
/// ### Returns
///
/// A list of NetworkInformation objects including information about whether a network is virtual or physical.
pub fn construct_network_information() -> Result<Vec<NetworkInformation>, CollectorError> {
    /*
     * Deconstruct NetworkInterface vector and construct NetworkInformation objects from it.
     */
    println!("Collecting Network Information...");

    let mut result = Vec::new();

    let my_fut = async {
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);
        let mut links = handle.link().get().execute();
        while let Some(msg) = links.try_next().await.unwrap() {
            let mut net_int = NetworkInformation::default();
            net_int.index = Some(msg.header.index);

            let mut has_link_to_other = false;

            for nla in msg.attributes.into_iter() {
                match nla {
                    LinkAttribute::Carrier(x) => {
                        if x == 1 {
                            net_int.is_connected = true;
                        }
                    }
                    LinkAttribute::Link(x) => {
                        if x != 0 {
                            has_link_to_other = true;
                        }
                    }
                    LinkAttribute::IfName(name) => {
                        net_int.name = name;
                    }
                    LinkAttribute::Address(addr) => {
                        net_int.mac_addr = Some(format!(
                            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                            addr[0], addr[1], addr[2], addr[3], addr[4], addr[5]
                        ));
                    }
                    _ => (),
                }
            }

            // A physical interface has a MAC address, no link and no reserved name.
            if !has_link_to_other
                && net_int.mac_addr.is_some()
                && !["veth", "br", "lo", "docker"]
                    .iter()
                    .any(|x| net_int.name.starts_with(x))
            {
                net_int.is_physical = true;
            }

            result.push(net_int);
        }

        let mut foo = handle.address().get().execute();
        while let Some(msg) = foo
            .try_next()
            .await
            .map_err(|x| CollectorError::InvalidNetworkInterface(x.to_string()))?
        {
            let target_intf = result
                .iter_mut()
                .find(|x| x.index.unwrap() == msg.header.index)
                .unwrap();
            for nia in msg.attributes.into_iter() {
                match nia {
                    AddressAttribute::Address(addr) => {
                        if addr.is_ipv4() {
                            target_intf.v4ip = Some(addr);

                            let bits = !(1u32 << (u32::BITS - msg.header.prefix_len as u32))
                                .wrapping_sub(1);
                            target_intf.v4netmask = Some(IpAddr::from(Ipv4Addr::from_bits(bits)));
                        }
                        if addr.is_ipv6() {
                            target_intf.v6ip = Some(addr);

                            let bits = !(1u128 << (u128::BITS - msg.header.prefix_len as u32))
                                .wrapping_sub(1);
                            target_intf.v6netmask = Some(IpAddr::from(Ipv6Addr::from_bits(bits)));
                        }
                    }
                    AddressAttribute::Broadcast(addr) => {
                        target_intf.v4broadcast = Some(IpAddr::from(addr));
                    }
                    AddressAttribute::Multicast(addr) => {
                        target_intf.v6broadcast = Some(IpAddr::from(addr));
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    };

    // The rtlink crate is async only. Because we don't want to introduce async everywhere,
    // just run the future in this thread.
    let t: Result<(), CollectorError> = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(my_fut);
    t?;

    // We always ignore the "lo" interface.
    result = result
        .into_iter()
        .filter(|x| x.name != "lo")
        .collect::<Vec<_>>();

    println!("\x1b[32m[success]\x1b[0m Network Interface collection completed.");
    Ok(result)
}
