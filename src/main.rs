mod cmus_status;
mod error;

fn main() {
    match cmus_status::get_cmus_status() {
        Ok(cmus_status) => unimplemented!(),
        Err(e) => panic!("{}", e),
    }
}
