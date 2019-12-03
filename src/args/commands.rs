use super::names;
use regex::Regex;
use std::convert::TryFrom;

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

pub enum CliCommand {
    Status,
    Help,
    DumpConfig,
}

impl CliCommand {
    pub fn name(&self) -> &str {
        match self {
            CliCommand::Status => names::CMD_STATUS,
            CliCommand::Help => names::CMD_HELP,
            CliCommand::DumpConfig => names::CMD_DUMP_CONFIG,
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
                names::CMD_DUMP_CONFIG => Ok(CliCommand::DumpConfig),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
