//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
//     let imgx = 800;
//     let imgy = 800;
// //    let mut imgbuf = image::ImageBuffer::from_raw(imgx, imgy, buffer).unwrap();

//     // Create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

//     // Iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r = (0.3 * x as f32) as u8;
//         let b = (0.3 * y as f32) as u8;
//         *pixel = image::Rgb([r, 0, b]);
//     }

//     let raw_buffer = imgbuf.into_raw();
//     let dyn_img = image::load_from_memory_with_format(&raw_buffer[..], image::ImageFormat::PNG).unwrap();
//     let mut result = vec![];
//     dyn_img.write_to(&mut result, image::ImageOutputFormat::JPEG(99));
//     dyn_img.save("test.png");

let mut f = File::open("/Users/augusto.dias/check.jpeg").unwrap();

let mut buffer = vec![];
f.read_to_end(&mut buffer);

    match image::load_from_memory_with_format(&buffer[..], image::ImageFormat::JPEG) {
        Err(why) => {
            panic!("{:?}", why);
        }
        Ok(dyn_img) => {
            let mut result = vec![];
            match dyn_img.write_to(&mut result, image::ImageOutputFormat::JPEG(99)) {
                Err(why) => {
                    panic!("{:?}", why);
                }
                Ok(_) => dyn_img.save("test.png").unwrap(),
            }
        }
    }
}