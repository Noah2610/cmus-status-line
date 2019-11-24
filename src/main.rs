extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;

mod cmus_status;
mod error;

fn main() {
    use std::io::{self, Write};
    use std::thread::sleep;
    use std::time::Duration;

    const SLEEP_MS: u64 = 100;
    let sleep_duration = Duration::from_millis(SLEEP_MS);
    let mut stdout = io::stdout();

    loop {
        match cmus_status::print_cmus_status() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        stdout.flush().unwrap();
        sleep(sleep_duration);
    }
}
