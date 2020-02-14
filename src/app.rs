extern crate cmus_status_line;

fn main() {
    use std::process;

    match cmus_status_line::run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
