//! This module provides logic to collect and process Information about all network interfaces a device has.

use futures::TryStreamExt;
use rtnetlink::new_connection;
use rtnetlink::packet_route::address::AddressAttribute;
use rtnetlink::packet_route::link::LinkAttribute;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{NazaraError, error::NazaraResult};

/// This object contains information about one specific network interface.
#[derive(Serialize, Debug, Default)]
pub struct NetworkInformation {
    /// The name of a network interface.
    pub name: String,
    /// The speed of the interface.
    pub interface_speed: Option<u32>,
    /// The IPv4 address of this interface.
    pub v4ip: Option<IpAddr>,
    /// The IPv4 broadcast address of this interface.
    pub v4broadcast: Option<IpAddr>,
    /// The IPv4 netmask of this interface.
    pub v4netmask: Option<IpAddr>,
    /// The IPv6 address of this interface.
    pub v6ip: Option<IpAddr>,
    /// The IPv6 broadcast address of this interface.
    pub v6broadcast: Option<IpAddr>,
    /// The IPv6 netmask of this interface.
    pub v6netmask: Option<IpAddr>,
    /// The mac address of this interface.
    pub mac_addr: Option<String>,
    /// The index of this interface.
    pub index: Option<u32>,
    /// Whether this device is a physical or virtual interface.
    pub is_physical: bool,
    /// Whether the interface is connected. Determined by if it has an address or not.
    pub is_connected: bool,
}

/// Returns a list of network interfaces.
/// Any collected loopback device is skipped.
///
/// # Returns
/// * `Ok(Vec<NetworkInformation>)` - A list of all collected network interfaces.
/// * `Err(CollectorError)` - A `CollectorError` instance containing information about the failure.
pub fn construct_network_information() -> NazaraResult<Vec<NetworkInformation>> {
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
            .map_err(|x| NazaraError::InvalidNetworkInterface(x.to_string()))?
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
    let t: NazaraResult<()> = tokio::runtime::Builder::new_current_thread()
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

    println!("Network Interface collection completed.");
    Ok(result)
}
