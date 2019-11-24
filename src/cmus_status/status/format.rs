use crate::error::prelude::*;
use regex::Regex;
use std::convert::TryFrom;

const DEFAULT_FORMAT: &str = "DEFAULT-%Title-MORE-%Artist";

pub struct Format {
    parts: Vec<FormatPart>,
}

#[derive(Deserialize)]
pub enum FormatPart {
    Text(String),
    Title,
    Artist,
}

impl TryFrom<&str> for Format {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?P<keyword>%\w+)|(?P<Text>.+?)").unwrap();

        let mut parts = Vec::new();

        for caps in re.captures_iter(string) {
            if let Some(keyword) = caps.name("keyword") {
                let keyword = keyword.as_str();
                let part = serde_plain::from_str::<FormatPart>(keyword)
                    .or(Err(Error::InvalidFormatKeyword(keyword.into())))?;
                parts.push(part);
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
