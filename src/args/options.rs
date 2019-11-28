use super::names;
use regex::Regex;
use std::convert::TryFrom;

#[derive(Default)]
pub struct CliOptions(pub(super) Vec<CliOption>);

impl CliOptions {
    pub fn has(&self, option: &CliOption) -> bool {
        self.0.iter().any(|opt| opt == option)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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
        let re = Regex::new(r#"^\s*(?P<dashes>--?)(?P<name>\S+)\s*$"#).unwrap();
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

#[derive(PartialEq)]
pub enum CliOption {
    Help,
}
