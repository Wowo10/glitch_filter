extern crate image;

use crate::mymath::Sobelable;
use crate::image::GenericImageView;
use std::env;
use std::path::Path;

mod mymath;

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
    
    std::fs::create_dir_all("./output").unwrap();

    //1. Apply grayscale
    let grayscaled = img.grayscale();
    grayscaled.save("output/1grayscale.png").unwrap();
    println!("grayscale - done");
    
    //2. apply Gaussian smoothering    
    let blurred = grayscaled.blur(1.0).to_luma();
    blurred.save("output/2blurred.png").unwrap();
    println!("blur - done");

    //3. sobel
    let sobel = blurred.sobel();
    sobel.save("output/3sobel.png").unwrap();
    println!("sobel - done");

    //transpoze + color
    //transpoze + color


    //combine pictures
}
