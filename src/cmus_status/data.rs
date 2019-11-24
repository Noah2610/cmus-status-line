pub mod prelude {
    pub use super::CmusData;
    pub use super::CmusPlaybackStatus;
    pub use super::CmusTime;
    pub use super::Seconds;
}

use crate::error::prelude::*;
use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;

pub type Seconds = u32;

#[derive(Debug)]
pub struct CmusData {
    status: CmusPlaybackStatus,
    // file /home/noah/Music/Soundtracks/Celeste/23_Official_Celeste_B-Sides_-_02_-_Ben_Prunty_-_Old_Site_Black_Moonrise_Mix.mp3
    file:     PathBuf,
    time:     CmusTime,
    settings: CmusSettings,
}

impl CmusData {
    pub fn get_title(&self) -> String {
        self.file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("_", " ")
    }

    pub fn get_status(&self) -> &CmusPlaybackStatus {
        &self.status
    }
}

impl TryFrom<String> for CmusData {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // status playing
        // file /home/noah/Music/Soundtracks/LittleRunmo/Little_Runmo_Level_One.mp3
        // duration 106
        // position 72
        // set aaa_mode artist
        // set continue true
        // set play_library false
        // set play_sorted true
        // set replaygain disabled
        // set replaygain_limit true
        // set replaygain_preamp 0.000000
        // set repeat true
        // set repeat_current false
        // set shuffle true
        // set softvol false
        // set vol_left 90
        // set vol_right 90

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
            file:   file.ok_or(Error::CmusMissingData(FILE_NAME.into()))?,
            time:   CmusTime {
                duration: time_duration
                    .ok_or(Error::CmusMissingData(TIME_DURATION_NAME.into()))?,
                position: time_position
                    .ok_or(Error::CmusMissingData(TIME_POSITION_NAME.into()))?,
            },
            // TODO
            settings: CmusSettings {},
        })
    }
}

// status playing
#[derive(Debug, PartialEq, Deserialize)]
pub enum CmusPlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

impl TryFrom<&str> for CmusPlaybackStatus {
    type Error = Error;

    fn try_from(status_str: &str) -> Result<Self, Self::Error> {
        const STATUS_PLAYING: &str = "playing";
        const STATUS_PAUSED: &str = "paused";
        const STATUS_STOPPED: &str = "stopped";

        match status_str.to_lowercase().as_str() {
            STATUS_PLAYING => Ok(CmusPlaybackStatus::Playing),
            STATUS_PAUSED => Ok(CmusPlaybackStatus::Paused),
            STATUS_STOPPED => Ok(CmusPlaybackStatus::Stopped),
            s => Err(Error::CmusUnknownStatus(s.into())),
        }
    }
}

impl fmt::Display for CmusPlaybackStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CmusPlaybackStatus::Playing => "Playing",
                CmusPlaybackStatus::Paused => "Paused",
                CmusPlaybackStatus::Stopped => "Stopped",
            }
        )
    }
}

// duration 203
// position 43
#[derive(Debug)]
pub struct CmusTime {
    pub duration: Seconds,
    pub position: Seconds,
}

// set aaa_mode artist
// set continue true
// set play_library false
// set play_sorted true
// set replaygain disabled
// set replaygain_limit true
// set replaygain_preamp 0.000000
// set repeat true
// set repeat_current false
// set shuffle true
// set softvol false
// set vol_left 90
// set vol_right 90
#[derive(Debug)]
pub struct CmusSettings {}
