use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let d = args.last().unwrap();

    let path = Path::new(d);

    if !path.exists() {
        panic!("File Does not exist");
    }
}
