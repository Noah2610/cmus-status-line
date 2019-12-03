pub mod prelude {
    pub use super::Args;
    pub use super::CliCommand;
    pub use super::CliCommands;
    pub use super::CliOption;
    pub use super::CliOptions;
}

mod names {
    pub(super) const CMD_HELP: &str = "help";
    pub(super) const CMD_STATUS: &str = "status";
    pub(super) const OPT_DOUBLE_HELP: &str = "help";
    pub(super) const OPT_DOUBLE_VERSION: &str = "version";
    pub(super) const OPT_SINGLE_HELP: char = 'h';
    pub(super) const OPT_SINGLE_VERSION: char = 'v';
}

mod commands;
mod options;

pub use commands::{CliCommand, CliCommands};
pub use options::{CliOption, CliOptions};

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
