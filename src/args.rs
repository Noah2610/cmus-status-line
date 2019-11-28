pub mod prelude {
    pub use super::Args;
    pub use super::CliCommand;
    pub use super::CliCommands;
    pub use super::CliOption;
    pub use super::CliOptions;
}

mod names {
    pub(super) const CMD_DEFAULT: &str = CMD_STATUS;
    pub(super) const CMD_HELP: &str = "help";
    pub(super) const CMD_STATUS: &str = "status";
    pub(super) const OPT_DOUBLE_HELP: &str = "help";
    pub(super) const OPT_SINGLE_HELP: char = 'h';
}

pub use argument_types::{CliCommand, CliCommands, CliOption, CliOptions};

use crate::error::prelude::*;
use std::convert::TryFrom;
use std::env;

pub struct Args {
    pub commands: CliCommands,
    pub options:  CliOptions,
}

impl Args {
    pub fn new() -> MyResult<Self> {
        let (commands, options) = env::args().skip(1).try_fold(
            (CliCommands::default(), CliOptions::default()),
            |(mut commands, mut options), arg| {
                if let Ok(opts) = CliOptions::try_from(arg.as_str()) {
                    options.0.append(&mut opts.into());
                    Ok((commands, options))
                } else {
                    if let Ok(cmd) = CliCommand::try_from(arg.as_str()) {
                        commands.0.push(cmd);
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

    impl CliCommand {
        fn name(&self) -> &str {
            match self {
                CliCommand::Status => names::CMD_STATUS,
                CliCommand::Help => names::CMD_HELP,
            }
        }
    }

    impl TryFrom<&str> for CliCommand {
        type Error = ();
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            let re = Regex::new(r#"^\s*(?P<name>\w+\S*)\s*$"#).unwrap();
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
        }
    }

    #[derive(Default)]
    pub struct CliCommands(pub(super) Vec<CliCommand>);

    impl CliCommands {
        pub fn iter(&self) -> std::slice::Iter<CliCommand> {
            self.0.iter()
        }
    }

    impl ToString for CliCommands {
        fn to_string(&self) -> String {
            self.0
                .iter()
                .map(CliCommand::name)
                .collect::<Vec<&str>>()
                .join(" ")
        }
    }

    pub enum CliOption {
        Help,
    }

    #[derive(Default)]
    pub struct CliOptions(pub(super) Vec<CliOption>);

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
                Regex::new(r#"^\s*(?P<dashes>--?)(?P<name>\S+)\s*$"#).unwrap();
            // let re_double = Regex::new(r#"^--(?P<name>\S+)"#).unwrap();
            if let Some(caps) = re.captures(s) {
                if let Some(name) = caps.name("name").map(|m| m.as_str()) {
                    let dashes = caps.name("dashes").ok_or(())?.as_str().len();

                    match dashes {
                        // DOUBLE
                        2 => match name {
                            names::OPT_DOUBLE_HELP => {
                                Ok(vec![CliOption::Help].into())
                            }
                            _ => Err(()),
                        },
                        // SINGLE
                        1 => Ok(name
                            .chars()
                            .try_fold(Vec::new(), |mut opts, c| match c {
                                names::OPT_SINGLE_HELP => {
                                    opts.push(CliOption::Help);
                                    Ok(opts)
                                }
                                _ => Err(()),
                            })?
                            .into()),
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
}
