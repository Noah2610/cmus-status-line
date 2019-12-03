mod format_expression;
mod format_part;

pub mod prelude {
    pub use super::format_expression::FormatExpression;
    pub use super::format_part::FormatPart;
    pub use super::Format;
}

pub use prelude::*;

use crate::error::prelude::*;
use regex::Regex;
use std::convert::TryFrom;

const DEFAULT_FORMAT: &str = r#"NO FORMAT"#;

#[derive(Deserialize)]
#[serde(default, try_from = "String")]
pub struct Format {
    parts: Vec<FormatPart>,
}

impl Format {
    pub fn iter(&self) -> std::slice::Iter<FormatPart> {
        self.parts.iter()
    }

    fn try_from_string(string: String) -> MyResult<Self> {
        let re =
            Regex::new(r"(%\{\s*(?P<keyword>(.|\s)+?)\s*\})|(?P<text>.+?)")
                .unwrap();
        let mut parts = Vec::new();

        for caps in re.captures_iter(string.as_str()) {
            if let Some(keyword) = caps.name("keyword") {
                let keyword = keyword.as_str();
                let part =
                    ron::de::from_str::<FormatPart>(keyword).or_else(|e| {
                        Err(Error::FailedParsingConfig(None, format!("{}", e)))
                    })?;
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

impl TryFrom<&str> for Format {
    type Error = Error;
    fn try_from(string: &str) -> MyResult<Self> {
        Self::try_from_string(string.to_string())
    }
}

impl TryFrom<String> for Format {
    type Error = Error;
    fn try_from(string: String) -> MyResult<Self> {
        Self::try_from_string(string)
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::try_from(DEFAULT_FORMAT).unwrap()
    }
}
