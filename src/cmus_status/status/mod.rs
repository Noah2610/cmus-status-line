mod builder;
mod format;
mod settings;

use super::data::prelude::*;
use crate::error::prelude::*;
use std::fmt;

use builder::CmusStatusBuilder;
use format::{Format, FormatPart};
use settings::Settings;

const OVERFLOW_STR: &str = "...";

pub struct CmusStatus {
    data:     CmusData,
    format:   Format,
    settings: Settings,
}

impl CmusStatus {
    pub fn builder() -> CmusStatusBuilder {
        CmusStatusBuilder::default()
    }

    fn get_format_text(&self, part: &FormatPart) -> Option<String> {
        match part {
            FormatPart::Text(text) => Some(text.to_string()),
            FormatPart::Title => self.data.get_title(),
            FormatPart::StatusStr => Some(self.data.get_status().to_string()),
            FormatPart::MatchStatus(status, text) => {
                if *status == *self.data.get_status() {
                    Some(text.to_string())
                } else {
                    None
                }
            }
            FormatPart::MaxLen(max, part_inner) => {
                let max = *max;
                self.get_format_text(part_inner.as_ref()).map(|text| {
                    let mut text = text.to_string();
                    if text.len() > max {
                        let overflow_str_len = OVERFLOW_STR.len();
                        if max >= overflow_str_len * 2 {
                            text.truncate(max - overflow_str_len);
                            text.push_str(OVERFLOW_STR);
                        } else {
                            text.truncate(max);
                        }
                    }
                    text
                })
            }
            FormatPart::ProgressBar(bar_config) => {
                if let Some(time) = self.data.get_time() {
                    let width = bar_config.inner_width();
                    let percent_complete = time.completion_percentage();
                    let characters =
                        (width as f32 * percent_complete).round() as usize;
                    Some(bar_config.text_with_filled(characters))
                } else {
                    None
                }
            }
        }
    }

    fn maybe_escape_html(&self, text: &str) -> String {
        if self.settings.escape_html {
            htmlescape::encode_minimal(text)
        } else {
            text.into()
        }
    }
}

impl fmt::Display for CmusStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.maybe_escape_html(
                self.format
                    .iter()
                    .filter_map(|part| self.get_format_text(part))
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str()
            )
        )
    }
}
