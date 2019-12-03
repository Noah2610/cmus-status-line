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
    pub(super) const CMD_DUMP_CONFIG: &str = "dump-config";
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

pub fn print_help() {
    let opt_help = {
        let opt = CliOption::Help;
        format!("-{}, --{}", opt.name_single(), opt.name_double())
    };
    let opt_vers = {
        let opt = CliOption::Version;
        format!("-{}, --{}", opt.name_single(), opt.name_double())
    };
    let cmd_status = CliCommand::Status.name();
    let cmd_help = CliCommand::Help.name();
    let cmd_dump_config = CliCommand::DumpConfig.name();

    println!(
        r#"{description}

USAGE:
    {name} [OPTIONS] [COMMAND]

OPTIONS:
    {opt_help:<opt_width$} Print this help message and exit.
    {opt_vers:<opt_width$} Print version information and exit.

COMMANDS:
    {cmd_status}
        Print the current cmus playback status
        with the format configured in the config.toml file.
        This is the default command, so you may omit this argument.
    {cmd_dump_config}
        Print the default config as TOML to stdout.
        To write the default config to the proper config file, run something like:
            mkdir -p ~/.config/{name}
            {name} {cmd_dump_config} > ~/.config/{name}/config.toml
    {cmd_help}
        Print this help message and exit."#,
        description = crate::meta::DESCRIPTION,
        name =  crate::meta::NAME,
        opt_width = 16,
        opt_help = opt_help,
        opt_vers = opt_vers,
        cmd_status = cmd_status,
        cmd_help = cmd_help,
        cmd_dump_config = cmd_dump_config,
    );
}

pub fn print_version() {
    println!("{} v{}", crate::meta::NAME, crate::meta::VERSION)
}

pub fn dump_config() {
    print!(
        r#"# DEFAULT CONFIG FOR {name}
# To write this config to the proper config file, run something like:
#     mkdir -p ~/.config/{name}
#     {name} {cmd_dump_config} > ~/.config/{name}/config.toml

{config}"#,
        name = crate::meta::NAME,
        cmd_dump_config = CliCommand::DumpConfig.name(),
        config = crate::config::DEFAULT_CONFIG
    );
}
