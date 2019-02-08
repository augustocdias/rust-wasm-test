extern crate cfg_if;
extern crate image;
extern crate wasm_bindgen;
extern crate web_sys;

mod image_reader;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use image_reader::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-wasm-test!");
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn blur(buffer: &[u8], format: String, value: f32) -> Vec<u8> {
    let img_format = get_format(&format);
    let img = read_image(buffer, img_format.input);
    return blur_image(&img, value, img_format.output);
}

#[wasm_bindgen]
pub fn brighten(buffer: &[u8], format: String, value: i32) -> Vec<u8> {
    let img_format = get_format(&format);
    let img = read_image(buffer, img_format.input);
    return brighten_image(&img, value, img_format.output);
}

#[wasm_bindgen]
pub fn flip(buffer: &[u8], format: String) -> Vec<u8> {
    let img_format = get_format(&format);
    let img = read_image(buffer, img_format.input);
    return flip_image(&img, img_format.output);
}

#[wasm_bindgen]
pub fn gray(buffer: &[u8], format: String) -> Vec<u8> {
    let img_format = get_format(&format);
    let img = read_image(buffer, img_format.input);
    return gray_image(&img, img_format.output);
}

#[wasm_bindgen]
pub fn invert(buffer: &[u8], format: String) -> Vec<u8> {
    let img_format = get_format(&format);
    let mut img = read_image(buffer, img_format.input);
    return invert_image(&mut img, img_format.output);
}

#[wasm_bindgen]
pub fn generate_fractal() -> Vec<u8> {
    return get_output(&fractal(), image::ImageOutputFormat::PNG);
}

#[wasm_bindgen]
pub fn show_image(buffer: &[u8], format: String) -> Vec<u8> {
    let img_format = get_format(&format);
    let img = read_image(buffer, img_format.input);
    return get_output(&img, img_format.output);
}

pub fn get_format(format: &String) -> InternalFormat {
    return match format.to_uppercase().as_ref() {
        "PNG" => InternalFormat{ input: image::ImageFormat::PNG, output: image::ImageOutputFormat::PNG },
        "BMP" => InternalFormat{ input: image::ImageFormat::BMP, output: image::ImageOutputFormat::BMP },
        "GIF" => InternalFormat{ input: image::ImageFormat::GIF, output: image::ImageOutputFormat::GIF },
        _ => {
            alert("Format not supported");
            panic!("format not supported");
        }
    };
}