extern crate dirs;
extern crate htmlescape;
extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate toml;

pub mod action;
pub mod args;
pub mod cmus_status;
pub mod config;
pub mod error;
pub mod meta;

pub fn run() -> error::MyResult<()> {
    use action::prelude::*;

    match action()? {
        Action::Status => cmus_status::print_cmus_status(),
        Action::Help => {
            args::print_help();
            Ok(())
        }
        Action::Version => {
            args::print_version();
            Ok(())
        }
        Action::DumpConfig => {
            args::dump_config();
            Ok(())
        }
    }
}
