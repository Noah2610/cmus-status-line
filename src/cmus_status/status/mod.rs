mod builder;
mod format;

use super::data::prelude::*;
use crate::error::prelude::*;
use std::fmt;

use builder::CmusStatusBuilder;
use format::{Format, FormatPart};

pub struct CmusStatus {
    data:   CmusData,
    format: Format,
}

impl CmusStatus {
    pub fn builder() -> CmusStatusBuilder {
        CmusStatusBuilder::default()
    }
}

impl fmt::Display for CmusStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.format
                .iter()
                .filter_map(|part| {
                    match part {
                        FormatPart::Text(text) => Some(text.to_string()),
                        FormatPart::Title => self.data.get_title(),
                        FormatPart::StatusStr => {
                            Some(self.data.get_status().to_string())
                        }
                        FormatPart::MatchStatus(status, text) => {
                            if *status == *self.data.get_status() {
                                Some(text.to_string())
                            } else {
                                None
                            }
                        }
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
