mod filter;

use std::error::Error;
use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use hex_color::HexColor;
use iwlib::WirelessInfo;
use serde_derive::{Deserialize, Serialize};

use self::filter::InterfaceFilter;
use crate::context::{BarItem, Context};
use crate::dbus::dbus_connection;
use crate::dbus::network_manager::NetworkManagerProxy;
use crate::format::fraction;
use crate::i3::{I3Item, I3Markup};
use crate::net::Interface;
use crate::theme::Theme;

impl Interface {
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
        let (addr, fg) = match self.get_wireless_info() {
            Some(info) => self.format_wireless(info, theme),
            None => self.format_normal(theme),
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
    /// This type is in the format of `interface[:type]`, where `interface` is the interface name, and
    /// `type` is an optional part which is either `ipv4` or `ipv6`.
    ///
    /// If `interface` is an empty string, then all interfaces are matched, for example:
    /// - `vpn0:ipv4` will match ip4 addresses for the `vpn` interface
    /// - `:ipv6`     will match all interfaces which have an ip6 address
    #[serde(default)]
    filter: Vec<InterfaceFilter>,
}

#[async_trait(?Send)]
impl BarItem for Nic {
    async fn start(self: Box<Self>, mut ctx: Context) -> Result<(), Box<dyn Error>> {
        let connection = dbus_connection(crate::dbus::BusType::System).await?;
        let nm = NetworkManagerProxy::new(&connection).await?;
        let mut nm_state_change = nm.receive_state_changed().await?;

        let mut idx = 0;
        loop {
            let mut interfaces = Interface::get_interfaces()?
                .into_iter()
                .filter(|i| {
                    if self.filter.is_empty() {
                        true
                    } else {
                        self.filter.iter().any(|f| f.matches(i))
                    }
                })
                .collect::<Vec<_>>();

            // no networks active
            if interfaces.is_empty() {
                ctx.update_item(I3Item::new("inactive").color(ctx.config.theme.dim))
                    .await?;

                idx = 0;
                tokio::select! {
                    Some(_) = ctx.wait_for_event(self.interval) => continue,
                    Some(_) = nm_state_change.next() => continue,
                }
            }

            let len = interfaces.len();
            idx = idx % len;

            let theme = &ctx.config.theme;
            let (full, short) = interfaces[idx].format(theme);
            let full = format!(r#"{}{}"#, full, fraction(theme, idx + 1, len));

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
