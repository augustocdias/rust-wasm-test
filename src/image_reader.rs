extern crate image;
extern crate num_complex;

use std::error::Error;
use alert;

pub struct InternalFormat {
    pub input: image::ImageFormat,
    pub output: image::ImageOutputFormat
}

pub fn blur_image(img: &image::DynamicImage, sigma: f32, format: image::ImageOutputFormat) -> Vec<u8> {
    return get_output(&img.blur(sigma), format);
}


pub fn brighten_image(img: &image::DynamicImage, value: i32, format: image::ImageOutputFormat) -> Vec<u8> {
    return get_output(&img.brighten(value), format);
}

pub fn flip_image(img: &image::DynamicImage, format: image::ImageOutputFormat) -> Vec<u8> {
    return get_output(&img.fliph().flipv(), format);
}

pub fn gray_image(img: &image::DynamicImage, format: image::ImageOutputFormat) -> Vec<u8> {
    return get_output(&img.grayscale(), format);
}

pub fn invert_image(img: &mut image::DynamicImage, format: image::ImageOutputFormat) -> Vec<u8> {
    img.invert();
    return get_output(img, format);
}

pub fn fractal() -> image::DynamicImage {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).data;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    return image::DynamicImage::ImageRgb8(imgbuf);

}

pub fn read_image(buffer: &[u8], format: image::ImageFormat) -> image::DynamicImage {
    
    match image::load_from_memory_with_format(buffer, format) {
        Err(why) => {
            alert(why.description());
            panic!("Error generating input: {:?}", why);
        }
        Ok(dyn_img) => dyn_img
    }
}

pub fn get_output(dyn_img: &image::DynamicImage, format: image::ImageOutputFormat) -> Vec<u8> {
    let mut result = vec![];
    match dyn_img.write_to(&mut result, format) {
        Err(why) => {
            alert(why.description());
            panic!("Error generating output: {:?}", why);
        }
        Ok(_) => result,
    }
}