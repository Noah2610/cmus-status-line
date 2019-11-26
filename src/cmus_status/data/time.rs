use crate::error::prelude::*;
use std::convert::TryFrom;
use std::fmt;

pub type Seconds = u32;

#[derive(Debug, PartialEq, Deserialize)]
pub enum CmusPlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

impl TryFrom<&str> for CmusPlaybackStatus {
    type Error = Error;

    fn try_from(status_str: &str) -> MyResult<Self> {
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

#[derive(Debug)]
pub struct CmusTime {
    pub duration: Seconds,
    pub position: Seconds,
}

impl CmusTime {
    pub fn completion_percentage(&self) -> f32 {
        self.position as f32 / self.duration as f32
    }
}
