use super::*;
use crate::error::prelude::*;
use regex::Regex;
use std::convert::TryFrom;

const DEFAULT_FORMAT: &str = r#"
%{MatchStatus(Playing, "")}
%{MatchStatus(Paused, "")}
%{MatchStatus(Stopped, "")} 
%{MaxLen(60, Title)}  
%{ProgressBar("<####---->")}
"#;

#[derive(Deserialize)]
#[serde(default, try_from = "String")]
pub struct Format {
    parts: Vec<FormatPart>,
}

impl Format {
    pub fn iter(&self) -> std::slice::Iter<FormatPart> {
        self.parts.iter()
    }

    fn try_from_string<S>(string: S) -> MyResult<Self>
    where
        S: ToString,
    {
        let string = string.to_string();
        let re = Regex::new(r"(%\{(?P<keyword>.+?)\})|(?P<text>.+?)").unwrap();
        let mut parts = Vec::new();

        for caps in re.captures_iter(string.as_str()) {
            if let Some(keyword) = caps.name("keyword") {
                let keyword = keyword.as_str();
                let part = ron::de::from_str::<FormatPart>(keyword)
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

impl TryFrom<&str> for Format {
    type Error = Error;

    fn try_from(string: &str) -> MyResult<Self> {
        Self::try_from_string(string)
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

#[derive(Deserialize)]
pub enum FormatPart {
    Text(String),
    Title,
    StatusStr,
    MatchStatus(CmusPlaybackStatus, String),
    MaxLen(usize, Box<FormatPart>), // Inclusive
    ProgressBar(ProgressBarConfig),
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

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct ProgressBarConfig {
    pub start:   Option<char>,
    pub end:     Option<char>,
    pub full:    char,
    pub empty:   char,
    total_width: usize,
}

impl ProgressBarConfig {
    pub fn inner_width(&self) -> usize {
        self.total_width
            - if self.start.is_some() { 1 } else { 0 }
            - if self.end.is_some() { 1 } else { 0 }
    }

    pub fn text_with_filled(&self, filled_characters: usize) -> String {
        assert!(self.total_width > filled_characters);

        let mut s = String::new();
        if let Some(start) = self.start {
            s.push(start);
        }
        s.push_str(self.full.to_string().repeat(filled_characters).as_str());
        s.push_str(
            self.empty
                .to_string()
                .repeat(self.inner_width() - filled_characters)
                .as_str(),
        );
        if let Some(end) = self.end {
            s.push(end);
        }
        s
    }
}

impl TryFrom<String> for ProgressBarConfig {
    type Error = Error;

    fn try_from(s: String) -> MyResult<Self> {
        let len = s.len();
        if len < 2 {
            Err(Error::ProgressBarConfigMinLen(2, s))
        } else if len == 2 {
            let chars = s.chars().collect::<Vec<char>>();
            Ok(ProgressBarConfig {
                start:       None,
                end:         None,
                full:        *chars.get(0).unwrap(),
                empty:       *chars.get(1).unwrap(),
                total_width: len,
            })
        } else if len == 3 {
            let chars = s.chars().collect::<Vec<char>>();
            Ok(ProgressBarConfig {
                start:       Some(*chars.get(0).unwrap()),
                end:         None,
                full:        *chars.get(1).unwrap(),
                empty:       *chars.get(2).unwrap(),
                total_width: len,
            })
        } else {
            let chars = s.chars().collect::<Vec<char>>();
            Ok(ProgressBarConfig {
                start:       Some(*chars.get(0).unwrap()),
                end:         Some(*chars.get(len - 1).unwrap()),
                full:        *chars.get(1).unwrap(),
                empty:       *chars.get(len - 2).unwrap(),
                total_width: len,
            })
        }
    }
}
