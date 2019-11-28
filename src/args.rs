pub mod prelude {
    pub use super::Args;
    pub use super::CliCommand;
    pub use super::CliOption;
}

mod names {
    pub(super) const CMD_DEFAULT: &str = CMD_STATUS;
    pub(super) const CMD_HELP: &str = "help";
    pub(super) const CMD_STATUS: &str = "status";
    pub(super) const OPT_DOUBLE_HELP: &str = "help";
    pub(super) const OPT_SINGLE_HELP: char = 'h';
}

pub use argument_types::{CliCommand, CliOption};

use crate::error::prelude::*;
use argument_types::CliOptions;
use std::convert::TryFrom;
use std::env;

pub struct Args {
    commands: Vec<CliCommand>,
    options:  Vec<CliOption>,
}

impl Args {
    pub fn new() -> MyResult<Self> {
        let (commands, options) = env::args().skip(1).try_fold(
            (Vec::<CliCommand>::new(), Vec::<CliOption>::new()),
            |(mut commands, mut options), arg| {
                if let Ok(mut opts) = CliOptions::try_from(arg.as_str()) {
                    options.append(&mut opts.into());
                    Ok((commands, options))
                } else {
                    if let Ok(cmd) = CliCommand::try_from(arg.as_str()) {
                        commands.push(cmd);
                        Ok((commands, options))
                    } else {
                        Err(Error::InvalidArgument(arg))
                    }
                }
            },
        )?;

        Ok(Self { commands, options })
    }
}

mod argument_types {
    use super::names;
    use regex::Regex;
    use std::convert::TryFrom;

    pub enum CliCommand {
        Status,
        Help,
    }

    impl TryFrom<&str> for CliCommand {
        type Error = ();
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            let re = Regex::new(r#"^\s*(?P<name>\w+\S*)\s*$"#).unwrap();
            if re.captures_len() == 1 {
                if let Some(name) = re
                    .captures(s)
                    .and_then(|caps| caps.name("name"))
                    .map(|m| m.as_str())
                {
                    match name {
                        names::CMD_STATUS => Ok(CliCommand::Status),
                        names::CMD_HELP => Ok(CliCommand::Help),
                        _ => Err(()),
                    }
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }

    pub enum CliOption {
        Help,
    }

    pub(super) struct CliOptions(Vec<CliOption>);

    impl Into<Vec<CliOption>> for CliOptions {
        fn into(self) -> Vec<CliOption> {
            self.0
        }
    }

    impl From<Vec<CliOption>> for CliOptions {
        fn from(opts: Vec<CliOption>) -> Self {
            Self(opts)
        }
    }

    impl TryFrom<&str> for CliOptions {
        type Error = ();
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            let re =
                Regex::new(r#"^\s*(?P<name_with_dashes>--?\S+)\s*$"#).unwrap();
            let re_double = Regex::new(r#"^--(?P<name>\S+)"#).unwrap();
            if re.captures_len() == 1 {
                if let Some(name_with_dashes) = re
                    .captures(s)
                    .and_then(|caps| caps.name("name_with_dashes"))
                    .map(|m| m.as_str())
                {
                    // DOUBLE
                    if let Some(name) = re_double
                        .captures(name_with_dashes)
                        .and_then(|caps| caps.name("name"))
                        .map(|m| m.as_str())
                    {
                        match name {
                            names::OPT_DOUBLE_HELP => {
                                Ok(vec![CliOption::Help].into())
                            }
                            _ => Err(()),
                        }
                    } else {
                        // SINGLE
                        Ok(name_with_dashes
                            .chars()
                            .skip(1)
                            .try_fold(Vec::new(), |mut opts, c| match c {
                                names::OPT_SINGLE_HELP => {
                                    opts.push(CliOption::Help);
                                    Ok(opts)
                                }
                                _ => Err(()),
                            })?
                            .into())
                    }
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
}
