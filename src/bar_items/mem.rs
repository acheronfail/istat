use std::error::Error;
use std::time::Duration;

use async_trait::async_trait;
use bytesize::ByteSize;
use hex_color::HexColor;
use serde_derive::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use sysinfo::SystemExt;

use crate::context::{BarItem, Context};
use crate::format::{float, FloatFormat};
use crate::i3::{I3Button, I3Item};
use crate::theme::Theme;
use crate::BarEvent;

#[derive(Debug, PartialEq, EnumIter)]
pub enum MemDisplay {
    Bytes,
    Percentage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mem {
    #[serde(with = "humantime_serde")]
    interval: Duration,
    #[serde(flatten)]
    float_fmt: FloatFormat,
}

impl Mem {
    fn get_color(theme: &Theme, used_pct: f64) -> Option<HexColor> {
        match used_pct as u64 {
            80..=100 => Some(theme.error),
            60..80 => Some(theme.danger),
            40..60 => Some(theme.warning),
            _ => None,
        }
    }
}

#[async_trait(?Send)]
impl BarItem for Mem {
    async fn start(self: Box<Self>, mut ctx: Context) -> Result<(), Box<dyn Error>> {
        let mut total = None;
        let mut display_iter = MemDisplay::iter().cycle();
        let display = &mut display_iter.next().unwrap();
        loop {
            let (available, total) = {
                let mut state = ctx.state.borrow_mut();
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

            let mut item = I3Item::new(format!(" {}", s)).name("mem");
            if let Some(fg) = Self::get_color(&ctx.theme, used_pct) {
                item = item.color(fg);
            }

            ctx.update_item(item).await?;
            ctx.delay_with_event_handler(self.interval, |ev| {
                if let BarEvent::Click(c) = ev {
                    if let I3Button::Left = c.button {
                        *display = display_iter.next().unwrap();
                    }
                }

                async {}
            })
            .await;
        }
    }
}
