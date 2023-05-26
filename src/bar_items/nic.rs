use std::cmp::Ordering;
use std::error::Error;
use std::net::{SocketAddrV4, SocketAddrV6};
use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use hex_color::HexColor;
use iwlib::{get_wireless_info, WirelessInfo};
use nix::ifaddrs::getifaddrs;
use nix::net::if_::InterfaceFlags;
use serde_derive::{Deserialize, Serialize};

use crate::context::{BarItem, Context};
use crate::dbus::dbus_connection;
use crate::dbus::network_manager::NetworkManagerProxy;
use crate::format::fraction;
use crate::i3::{I3Item, I3Markup};
use crate::theme::Theme;

#[derive(Debug, PartialEq, Eq)]
struct Interface {
    name: String,
    addr: String,
    is_wireless: Option<bool>,
}

impl PartialOrd for Interface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.addr.partial_cmp(&other.addr) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.is_wireless.partial_cmp(&other.is_wireless)
    }
}

impl Ord for Interface {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Interface {
    fn new(name: impl AsRef<str>, addr: impl AsRef<str>) -> Interface {
        Interface {
            name: name.as_ref().into(),
            addr: addr.as_ref().into(),
            is_wireless: None,
        }
    }

    fn format_wireless(&self, i: WirelessInfo, theme: &Theme) -> (String, Option<HexColor>) {
        let fg = match i.wi_quality {
            100..=u8::MAX => theme.green,
            80..=99 => theme.green,
            60..=79 => theme.yellow,
            40..=59 => theme.orange,
            _ => theme.red,
        };

        (
            format!("({}) {}% at {}", self.addr, i.wi_quality, i.wi_essid),
            Some(fg),
        )
    }

    fn format_normal(&self, theme: &Theme) -> (String, Option<HexColor>) {
        (format!("({})", self.addr), Some(theme.green))
    }

    fn format(&mut self, theme: &Theme) -> (String, String) {
        // TODO: contribute AsRef upstream to https://github.com/psibi/iwlib-rs
        // See: https://github.com/psibi/iwlib-rs/pull/2
        let name = self.name.clone();

        // check if this is a wireless network
        let (addr, fg) = match self.is_wireless {
            // not a wireless interface, just return defaults
            Some(false) => self.format_normal(theme),
            // SAFETY: we've previously checked if this is a wireless network
            Some(true) => self.format_wireless(get_wireless_info(name).unwrap(), theme),
            // check if we're a wireless network and remember for next time
            None => match get_wireless_info(name) {
                Some(i) => {
                    self.is_wireless = Some(true);
                    self.format_wireless(i, theme)
                }
                None => {
                    self.is_wireless = Some(false);
                    self.format_normal(theme)
                }
            },
        };

        let fg = fg
            .map(|c| format!(r#" foreground="{}""#, c))
            .unwrap_or("".into());
        (
            format!(r#"<span{}>{}{}</span>"#, fg, self.name, addr),
            format!(r#"<span{}>{}</span>"#, fg, self.name),
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Nic {
    #[serde(default, with = "crate::human_time::option")]
    interval: Option<Duration>,
}

impl Nic {
    fn get_interfaces() -> Result<Vec<Interface>, Box<dyn Error>> {
        let if_addrs = match getifaddrs() {
            Ok(if_addrs) => if_addrs,
            Err(e) => return Err(format!("call to `getifaddrs` failed: {}", e).into()),
        };

        let mut interfaces = vec![];
        for if_addr in if_addrs.into_iter() {
            // skip any interfaces that aren't active
            if !if_addr.flags.contains(InterfaceFlags::IFF_UP) {
                continue;
            }

            // skip the local loopback interface
            if if_addr.flags.contains(InterfaceFlags::IFF_LOOPBACK) {
                continue;
            }

            // skip any unsupported entry (see nix's `getifaddrs` documentation)
            let addr = match if_addr.address {
                Some(addr) => addr,
                None => continue,
            };

            // extract ip address
            let addr = match (addr.as_sockaddr_in(), addr.as_sockaddr_in6()) {
                (Some(ipv4), _) => format!("{}", SocketAddrV4::from(*ipv4).ip()),
                (_, Some(ipv6)) => format!("{}", SocketAddrV6::from(*ipv6).ip()),
                (None, None) => continue,
            };

            interfaces.push(Interface::new(if_addr.interface_name, addr));
        }

        interfaces.sort();

        Ok(interfaces)
    }
}

#[async_trait(?Send)]
impl BarItem for Nic {
    async fn start(self: Box<Self>, mut ctx: Context) -> Result<(), Box<dyn Error>> {
        let connection = dbus_connection(crate::dbus::BusType::System).await?;
        let nm = NetworkManagerProxy::new(&connection).await?;
        let mut nm_state_change = nm.receive_state_changed().await?;

        let mut idx = 0;
        loop {
            let mut interfaces = Nic::get_interfaces()?;

            // no networks active
            if interfaces.is_empty() {
                ctx.update_item(I3Item::new("disconnected").color(ctx.theme().red))
                    .await?;

                idx = 0;
                tokio::select! {
                    Some(_) = ctx.wait_for_event(self.interval) => continue,
                    Some(_) = nm_state_change.next() => continue,
                }
            }

            let len = interfaces.len();
            idx = idx % len;

            let theme = ctx.theme();
            let (full, short) = interfaces[idx].format(&theme);
            let full = format!(r#"{}{}"#, full, fraction(&theme, idx + 1, len));

            let item = I3Item::new(full).short_text(short).markup(I3Markup::Pango);
            ctx.update_item(item).await?;

            // cycle through networks on click
            let wait_for_click = async {
                match self.interval {
                    Some(duration) => {
                        ctx.delay_with_event_handler(duration, |event| {
                            Context::paginate(&event, len, &mut idx);
                            async {}
                        })
                        .await
                    }
                    None => {
                        if let Some(event) = ctx.wait_for_event(self.interval).await {
                            Context::paginate(&event, len, &mut idx);
                        }
                    }
                }
            };

            tokio::select! {
                () = wait_for_click => continue,
                Some(_) = nm_state_change.next() => continue,
            }
        }
    }
}
