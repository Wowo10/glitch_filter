extern crate image;

use crate::image::GenericImageView;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let d = args.last().unwrap();

    let path = Path::new(d);

    if !path.exists() {
        panic!("File Does not exist");
    }

    let img = image::open(path).unwrap();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    //img.save("data/test.png").unwrap();
}
