mod playback_status;
mod time;

pub mod prelude {
    pub use super::CmusData;
    pub use super::CmusPlaybackStatus;
    pub use super::{CmusTime, Seconds};
}

pub use playback_status::CmusPlaybackStatus;
pub use time::{CmusTime, Seconds};

use crate::error::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CmusData {
    status:   CmusPlaybackStatus,
    file:     Option<PathBuf>,
    time:     Option<CmusTime>,
    tags:     HashMap<String, String>,
    settings: CmusSettings,
}

impl CmusData {
    pub fn get_title(&self) -> Option<String> {
        self.file.as_ref().map(|file| {
            file.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("_", " ")
        })
    }

    pub fn get_status(&self) -> &CmusPlaybackStatus {
        &self.status
    }

    pub fn get_tag(&self, tag_name: &str) -> Option<String> {
        self.tags.get(tag_name).cloned()
    }

    pub fn is_status(&self, other_status: &CmusPlaybackStatus) -> bool {
        &self.status == other_status
    }

    pub fn get_time(&self) -> &Option<CmusTime> {
        &self.time
    }
}

impl TryFrom<String> for CmusData {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        const STATUS_NAME: &str = "status";
        const FILE_NAME: &str = "file";
        const TIME_DURATION_NAME: &str = "duration";
        const TIME_POSITION_NAME: &str = "position";
        const TAG_NAME: &str = "tag";
        const SETTINGS_NAME: &str = "set";

        let mut status = None;
        let mut file = None;
        let mut time_duration = None;
        let mut time_position = None;
        let mut tags = HashMap::new();

        for line in string.trim().split("\n") {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let data_name =
                words.first().ok_or(Error::CmusParseError(format!(
                    "Couldn't get data identifier name from `cmus-remote -Q` \
                     (first word per line)\nOutput:\n{}",
                    string
                )))?;
            let data_words = words
                .iter()
                .skip(1)
                .map(Deref::deref)
                .collect::<Vec<&str>>();
            let data_line = data_words.join(" ");

            match *data_name {
                STATUS_NAME => {
                    status =
                        Some(CmusPlaybackStatus::try_from(data_line.as_str())?);
                }
                FILE_NAME => {
                    file = Some(PathBuf::from(data_line.as_str()));
                }
                TIME_DURATION_NAME => {
                    time_duration = Some(data_line.parse::<Seconds>().or(
                        Err(Error::CouldntParseTimeToNumber(line.into())),
                    )?);
                }
                TIME_POSITION_NAME => {
                    time_position = Some(data_line.parse::<Seconds>().or(
                        Err(Error::CouldntParseTimeToNumber(line.into())),
                    )?);
                }
                TAG_NAME => {
                    let tag_name = data_words
                        .get(0)
                        .ok_or(Error::CmusExpectDataArguments(1, line.into()))?
                        .to_string();
                    let tag_value = data_words
                        .iter()
                        .skip(1)
                        .map(Deref::deref)
                        .collect::<Vec<&str>>()
                        .join(" ");
                    tags.insert(dbg!(tag_name), dbg!(tag_value));
                }
                SETTINGS_NAME => {
                    // TODO
                }
                _ => return Err(Error::CmusUnknownData(line.into())),
            }
        }

        Ok(Self {
            status: status.ok_or(Error::CmusMissingData(STATUS_NAME.into()))?,
            file: file,
            time: time_duration
                .and_then(|duration| {
                    time_position
                        .and_then(|position| Some((duration, position)))
                })
                .map(|(duration, position)| CmusTime {
                    duration: duration,
                    position: position,
                }),
            tags,
            // TODO
            settings: CmusSettings {},
        })
    }
}

// TODO
#[derive(Debug)]
pub struct CmusSettings {}
