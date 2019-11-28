extern crate dirs;
extern crate htmlescape;
extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate toml;

mod action;
mod args;
mod cmus_status;
mod config;
mod error;
mod meta;

fn main() {
    use std::process;

    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn run() -> error::MyResult<()> {
    use action::prelude::*;

    match action()? {
        Action::Status => cmus_status::print_cmus_status(),
        Action::Help => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("{} USAGE\nTODO", meta::NAME);
}
