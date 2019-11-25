extern crate dirs;
extern crate htmlescape;
extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate toml;

mod action;
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

    match action() {
        Action::PrintStatus => cmus_status::print_cmus_status(),
        Action::PrintAbout => {
            print_about();
            Ok(())
        }
    }
}

fn print_about() {
    unimplemented!()
}
