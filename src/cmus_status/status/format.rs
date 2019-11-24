use crate::error::prelude::*;
use regex::Regex;
use std::convert::TryFrom;

const DEFAULT_FORMAT: &str = "TITLE: %Title - STATUS_STR: %StatusStr";

pub struct Format {
    parts: Vec<FormatPart>,
}

impl Format {
    pub fn iter(&self) -> std::slice::Iter<FormatPart> {
        self.parts.iter()
    }
}

#[derive(Deserialize)]
pub enum FormatPart {
    Text(String),
    Title,
    StatusStr,
}

impl FormatPart {
    fn is_text(&self) -> bool {
        if let FormatPart::Text(_) = self {
            true
        } else {
            false
        }
    }

    fn is_keyword(&self) -> bool {
        !self.is_text()
    }
}

impl TryFrom<&str> for Format {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(%(?P<keyword>\w+))|(?P<text>.+?)").unwrap();

        let mut parts = Vec::new();

        for caps in re.captures_iter(string) {
            if let Some(keyword) = caps.name("keyword") {
                let keyword = keyword.as_str();
                let part = serde_plain::from_str::<FormatPart>(keyword)
                    .or(Err(Error::InvalidFormatKeyword(keyword.into())))?;
                parts.push(part);
            }
            if let Some(text) = caps.name("text") {
                if let Some(prev_text) = parts.last_mut().and_then(|last| {
                    if let FormatPart::Text(prev_text) = last {
                        Some(prev_text)
                    } else {
                        None
                    }
                }) {
                    prev_text.push_str(text.as_str());
                } else {
                    parts.push(FormatPart::Text(text.as_str().into()));
                }
            }
        }

        Ok(Self { parts })
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::try_from(DEFAULT_FORMAT).unwrap()
    }
}
