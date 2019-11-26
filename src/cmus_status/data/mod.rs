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
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CmusData {
    status:   CmusPlaybackStatus,
    file:     Option<PathBuf>,
    time:     Option<CmusTime>,
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
        const SETTINGS_NAME: &str = "set";

        let mut status = None;
        let mut file = None;
        let mut time_duration = None;
        let mut time_position = None;
        // let mut settings = Vec::new();

        for line in string.trim().split("\n") {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let data_name =
                words.first().ok_or(Error::CmusParseError(format!(
                    "Couldn't get data identifier name from `cmus-remote -Q` \
                     (first word per line)\nOutput:\n{}",
                    string
                )))?;

            match *data_name {
                STATUS_NAME => {
                    status = Some(CmusPlaybackStatus::try_from(
                        *words.get(1).ok_or(Error::CmusExpectDataArguments(
                            1,
                            line.into(),
                        ))?,
                    )?);
                }
                FILE_NAME => {
                    file = Some(PathBuf::from(*words.get(1).ok_or(
                        Error::CmusExpectDataArguments(1, line.into()),
                    )?));
                }
                TIME_DURATION_NAME => {
                    time_duration = Some(
                        (*words.get(1).ok_or(
                            Error::CmusExpectDataArguments(1, line.into()),
                        )?)
                        .parse::<Seconds>()
                        .or(Err(
                            Error::CouldntParseTimeToNumber(line.into()),
                        ))?,
                    );
                }
                TIME_POSITION_NAME => {
                    time_position = Some(
                        (*words.get(1).ok_or(
                            Error::CmusExpectDataArguments(1, line.into()),
                        )?)
                        .parse::<Seconds>()
                        .or(Err(
                            Error::CouldntParseTimeToNumber(line.into()),
                        ))?,
                    );
                }
                SETTINGS_NAME => {
                    // TODO
                }
                _ => return Err(Error::CmusUnknownData(line.into())),
            }
        }

        Ok(Self {
            status: status.ok_or(Error::CmusMissingData(STATUS_NAME.into()))?,
            file:   file,
            time:   time_duration
                .and_then(|duration| {
                    time_position
                        .and_then(|position| Some((duration, position)))
                })
                .map(|(duration, position)| CmusTime {
                    duration: duration,
                    position: position,
                }),
            // TODO
            settings: CmusSettings {},
        })
    }
}

// TODO
#[derive(Debug)]
pub struct CmusSettings {}
