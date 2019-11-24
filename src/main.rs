extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_plain;

mod cmus_status;
mod error;

fn main() {
    match cmus_status::print_cmus_status() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
