extern crate dirs;
extern crate htmlescape;
extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate toml;

mod cmus_status;
mod config;
mod error;

fn main() {
    match cmus_status::print_cmus_status() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
