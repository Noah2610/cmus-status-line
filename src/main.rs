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
    match run() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}

fn run() -> error::MyResult<()> {
    use action::prelude::*;

    match action()? {
        Action::Status => cmus_status::print_cmus_status(),
        Action::About => {
            print_about();
            Ok(())
        }
    }
}

fn print_about() {
    unimplemented!()
}
