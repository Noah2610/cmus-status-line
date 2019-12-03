use crate::args::prelude::*;
use crate::error::prelude::*;

pub mod prelude {
    pub use super::action;
    pub use super::Action;
}

pub enum Action {
    Status,
    Help,
    Version,
    DumpConfig,
}

impl Default for Action {
    fn default() -> Self {
        Action::Status
    }
}

pub fn action() -> MyResult<Action> {
    let args = Args::new()?;

    let mut action_opt = args
        .commands
        .iter()
        .try_fold((None, 0), |(_, cmd_index), cmd| {
            let act_or_err: MyResult<Action> = match cmd {
                CliCommand::Status => {
                    if cmd_index == 0 {
                        if args.options.is_empty() {
                            Ok(Action::Status)
                        } else {
                            Err(Error::CommandTakesNoOptions(
                                cmd.name().to_string(),
                            ))
                        }
                    } else {
                        Err(Error::InvalidCommandLen(args.commands.to_string()))
                    }
                }
                CliCommand::Help => Ok(Action::Help),
                CliCommand::DumpConfig => Ok(Action::DumpConfig),
            };
            match act_or_err {
                Ok(act) => Ok((Some(act), cmd_index + 1)),
                Err(e) => Err(e),
            }
        })?
        .0;

    if action_opt.is_none() {
        action_opt = args.options.iter().find_map(|opt| match opt {
            CliOption::Help => Some(Action::Help),
            CliOption::Version => Some(Action::Version),
        });
    }

    Ok(action_opt.unwrap_or_else(Action::default))
}
