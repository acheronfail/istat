use std::error::Error;
use std::time::Duration;

use async_trait::async_trait;
use bytesize::ByteSize;
use hex_color::HexColor;
use serde_derive::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use sysinfo::SystemExt;

use crate::context::{BarEvent, BarItem, Context};
use crate::format::{float, FloatFormat};
use crate::i3::{I3Button, I3Item, I3Markup};
use crate::theme::Theme;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum MemDisplay {
    #[default]
    Bytes,
    Percentage,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Mem {
    #[serde(with = "crate::human_time")]
    interval: Duration,
    #[serde(flatten)]
    float_fmt: FloatFormat,
    #[serde(default)]
    display: MemDisplay,
}

impl Mem {
    fn get_color(theme: &Theme, used_pct: f64) -> Option<HexColor> {
        match used_pct as u64 {
            80..=100 => Some(theme.red),
            60..=79 => Some(theme.orange),
            40..=59 => Some(theme.yellow),
            _ => None,
        }
    }
}

#[async_trait(?Send)]
impl BarItem for Mem {
    async fn start(self: Box<Self>, mut ctx: Context) -> Result<(), Box<dyn Error>> {
        let mut total = None;
        let mut display_iter = MemDisplay::iter().cycle();
        // SAFETY: this is an endless iterator through the enum, so it should never be empty
        let display = &mut display_iter.next().unwrap();
        while *display != self.display {
            *display = display_iter.next().unwrap();
        }

        loop {
            let (available, total) = {
                let state = ctx.state.get_mut();
                state.sys.refresh_memory();
                (
                    state.sys.available_memory(),
                    *total.get_or_insert_with(|| state.sys.total_memory()),
                )
            };

            let used_pct = ((total - available) as f64 / total as f64) * 100.0;
            let s = match *display {
                MemDisplay::Bytes => ByteSize(available).to_string_as(false),
                MemDisplay::Percentage => format!("{}%", float(used_pct, &self.float_fmt)),
            };

            let mut item = I3Item::new(format!(" {}", s)).markup(I3Markup::Pango);
            if let Some(fg) = Self::get_color(&ctx.theme(), used_pct) {
                item = item.color(fg);
            }

            ctx.update_item(item).await?;
            ctx.delay_with_event_handler(self.interval, |ev| {
                if let BarEvent::Click(c) = ev {
                    if let I3Button::Left = c.button {
                        // SAFETY: this is an endless iterator through the enum, so it should never be empty
                        *display = display_iter.next().unwrap();
                    }
                }

                async {}
            })
            .await;
        }
    }
}
