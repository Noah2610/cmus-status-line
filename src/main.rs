extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;

mod cmus_status;
mod error;

fn main() {
    match cmus_status::print_cmus_status() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
