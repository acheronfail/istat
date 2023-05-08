use std::error::Error;
use std::net::{SocketAddrV4, SocketAddrV6};
use std::time::Duration;

use async_trait::async_trait;
use nix::ifaddrs::getifaddrs;
use tokio::time::sleep;

use crate::context::{BarItem, Context};
use crate::i3::I3Item;

#[derive(Debug)]
struct Interface {
    name: String,
    addr: String,
    // TODO: network name + strength for wifi
}

pub struct Nic {
    interval: Duration,
}

impl Default for Nic {
    fn default() -> Self {
        Nic {
            interval: Duration::from_secs(60),
        }
    }
}

impl Nic {
    fn get_interfaces() -> Vec<Interface> {
        let if_addrs = match getifaddrs() {
            Ok(if_addrs) => if_addrs,
            Err(_) => todo!(),
        };

        let mut interfaces = vec![];
        for if_addr in if_addrs.into_iter() {
            if if_addr.interface_name == "lo" {
                continue;
            }

            let addr = match if_addr.address {
                Some(addr) => addr,
                None => continue,
            };

            let addr = match (addr.as_sockaddr_in(), addr.as_sockaddr_in6()) {
                (Some(ipv4), _) => format!("{}", SocketAddrV4::from(*ipv4).ip()),
                (_, Some(ipv6)) => format!("{}", SocketAddrV6::from(*ipv6).ip()),
                (None, None) => continue,
            };

            interfaces.push(Interface {
                name: if_addr.interface_name,
                addr,
            });
        }

        interfaces
    }
}

#[async_trait(?Send)]
impl BarItem for Nic {
    async fn start(self: Box<Self>, ctx: Context) -> Result<(), Box<dyn Error>> {
        loop {
            let interfaces = Nic::get_interfaces();
            let full_text = interfaces
                .iter()
                .map(|i| format!("{}: {}", i.name, i.addr))
                .collect::<Vec<_>>()
                .join(", ");
            let short_text = interfaces
                .iter()
                .map(|i| i.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            // TODO: network name for short text
            // TODO: color for network strength
            ctx.update_item(I3Item::new(full_text).short_text(short_text).name("nic"))
                .await?;

            // TODO: is there an agnostic/kernel way to detect network changes and _only then_ check for ips?
            // if not, then: dbus? networkmanager?
            sleep(self.interval).await;
        }
    }
}