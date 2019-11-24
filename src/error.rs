use std::fmt;
use std::process::ExitStatus;

pub mod prelude {
    pub use super::Error;
    pub use super::MyResult;
}

pub type MyResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CmusNotInstalled,
    CmusError { status: ExitStatus, stderr: String },
    CmusParseError(String),
    CmusUnknownData(String),
    CmusExpectDataArguments(usize, String),
    CmusUnknownStatus(String),
    CouldntParseTimeToNumber(String),
    CmusMissingData(String),
}

impl Error {
    fn message(&self) -> String {
        match self {
            Error::CmusNotInstalled => "cmus is not installed.".to_string(),
            Error::CmusError { status, stderr } => format!(
                "cmus exited with status code {}\nstderr: {}",
                status, stderr,
            ),
            Error::CmusParseError(msg) => {
                format!("cmus-remote parsing error: {}", msg)
            }
            Error::CmusUnknownData(data_line) => {
                format!("cmus-remote returned unknown data: {}", data_line)
            }
            Error::CmusExpectDataArguments(expected_args, data_line) => {
                format!(
                    "expected {} arguments for data line from cmus-remote: {}",
                    expected_args, data_line
                )
            }
            Error::CmusUnknownStatus(status) => {
                format!("cmus-remote returned unknown status: {}", status)
            }
            Error::CouldntParseTimeToNumber(time_string) => format!(
                "couldn't parse string to number, expected to be string of \
                 seconds: {}",
                time_string
            ),
            Error::CmusMissingData(data_name) => {
                format!("missing required data from cmus-remote: {}", data_name)
            }
        }
    }
}

impl std::error::Error for Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR:\n{}\nExiting.", self.message())
    }
}
