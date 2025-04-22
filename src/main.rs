mod cli;
use cli::{Address, IP, Link};

use clap::Parser;
use indexmap::IndexMap;
use nix::{
    ifaddrs::{self, InterfaceAddress},
    net,
    sys::socket::{AddressFamily, SockaddrLike},
};

#[cfg(linux_android)]
use nix::libc::{ARPHRD_ETHER, ARPHRD_LOOPBACK};

type InterfaceMap = IndexMap<String, Vec<InterfaceAddress>>;

/// Creates a Map of interface names and their InterfaceAddress configurations
/// where each interface may have more than one InterfaceAdress configuration
fn collect_interfaces() -> Result<InterfaceMap, nix::Error> {
    let mut ifaddr_map: InterfaceMap = InterfaceMap::new();
    ifaddrs::getifaddrs()?.for_each(|ifaddr| {
        ifaddr_map
            .entry(ifaddr.interface_name.clone())
            .or_default()
            .push(ifaddr.clone());
    });
    Ok(ifaddr_map)
}

// InterfaceFlags formatted in the style of iproute2
fn flag_names(ifaddr: &InterfaceAddress) -> String {
    ifaddr
        .flags
        .iter_names()
        .map(|(name, _)| name.strip_prefix("IFF_").unwrap_or(name))
        .collect::<Vec<_>>()
        .join(",")
}

// TODO: accept args
fn link_show() -> Result<(), nix::Error> {
    let ifaddrs = collect_interfaces()?;
    for (ifname, ifaddrs) in ifaddrs.iter() {
        let ifidx = net::if_::if_nametoindex(ifname.as_str())?;
        let flag_names = flag_names(ifaddrs.first().unwrap());
        println!("{}: {}: <{}>", ifidx, ifname, flag_names);
        for ifaddr in ifaddrs {
            let address = ifaddr.address.unwrap();
            match address.family() {
                #[cfg(any(bsd, solarish))]
                Some(AddressFamily::Link) => {
                    if ifaddr.flags.contains(InterfaceFlags::IFF_LOOPBACK) {
                        println!("    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00");
                        continue;
                    }
                    let addr = address.as_link_addr().unwrap();
                    if addr.is_empty() {
                        println!("    link/none");
                        continue;
                    }
                    println!("    link/ether {addr} brd ff:ff:ff:ff:ff:ff");
                }
                #[cfg(linux_android)]
                Some(AddressFamily::Packet) => {
                    let addr = address.as_link_addr().unwrap();
                    // https://github.com/torvalds/linux/blob/master/include/uapi/linux/if_arp.h
                    match addr.hatype() {
                        ARPHRD_LOOPBACK => {
                            println!("    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00");
                        }
                        ARPHRD_ETHER => {
                            println!("    link/ether {addr} brd ff:ff:ff:ff:ff:ff");
                        },
                        _ => {
                            println!("    link/none");
                        }
                    }
                }
                _ => {}
            }
        }
        println!();
    }
    Ok(())
}

// TODO: accept args
fn addr_show() -> Result<(), nix::Error> {
    let ifaddrs = collect_interfaces()?;
    for (ifname, ifaddrs) in ifaddrs.iter() {
        let ifidx = net::if_::if_nametoindex(ifname.as_str())?;
        let flag_names = flag_names(ifaddrs.first().unwrap());
        println!("{}: {}: <{}>", ifidx, ifname, flag_names);
        for ifaddr in ifaddrs {
            let address = ifaddr.address.unwrap();
            match address.family() {
                #[cfg(any(bsd, solarish))]
                Some(AddressFamily::Link) => {
                    if ifaddr.flags.contains(InterfaceFlags::IFF_LOOPBACK) {
                        println!("    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00");
                        continue;
                    }
                    let addr = address.as_link_addr().unwrap();
                    if addr.is_empty() {
                        println!("    link/none");
                        continue;
                    }
                    println!("    link/ether {addr} brd ff:ff:ff:ff:ff:ff");
                }
                #[cfg(linux_android)]
                Some(AddressFamily::Packet) => {
                    let addr = address.as_link_addr().unwrap();
                    // https://github.com/torvalds/linux/blob/master/include/uapi/linux/if_arp.h
                    match addr.hatype() {
                        ARPHRD_LOOPBACK => {
                            println!("    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00");
                        }
                        ARPHRD_ETHER => {
                            println!("    link/ether {addr} brd ff:ff:ff:ff:ff:ff");
                        },
                        _ => {
                            println!("    link/none");
                        }
                    }
                }
                Some(AddressFamily::Inet) => {
                    let addr = address.as_sockaddr_in().unwrap().ip();
                    let mask = ifaddr.netmask.unwrap().as_sockaddr_in().unwrap().ip();
                    let prefix = u32::from(mask).leading_ones();
                    match (ifaddr.broadcast, ifaddr.destination) {
                        // eth
                        (Some(bcast), None) => {
                            let bcast = bcast.as_sockaddr_in().unwrap().ip();
                            println!("    inet {addr}/{prefix} brd {bcast}");
                        }
                        // p2p
                        (None, Some(peer)) => {
                            let peer = peer.as_sockaddr_in().unwrap().ip();
                            println!("    inet {addr} peer {peer}/{prefix}");
                        }
                        // loopback,etc
                        _ => {
                            println!("    inet {addr}/{prefix}");
                        }
                    };
                }
                Some(AddressFamily::Inet6) => {
                    let addr = address.as_sockaddr_in6().unwrap().ip();
                    let netmask = ifaddr.netmask.unwrap().as_sockaddr_in6().unwrap().ip();
                    let prefix = u128::from(netmask).leading_ones();
                    match (ifaddr.broadcast, ifaddr.destination) {
                        // eth
                        (Some(bcast), None) => {
                            let bcast = bcast.as_sockaddr_in6().unwrap().ip();
                            println!("    inet6 {addr}/{prefix} brd {bcast}");
                        }
                        // p2p
                        (None, Some(peer)) => {
                            let peer = peer.as_sockaddr_in6().unwrap().ip();
                            println!("    inet6 {addr} peer {peer}/{prefix}");
                        }
                        // loopback,etc
                        _ => {
                            println!("    inet6 {addr}/{prefix}");
                        }
                    };
                }
                _ => {}
            };
        }
        println!();
    }
    Ok(())
}

fn main() -> Result<(), nix::Error> {
    match IP::parse() {
        IP::Address(Address::Show) => addr_show(),
        IP::Link(Link::Show) => link_show(),
        _ => unimplemented!(),
    }
}
