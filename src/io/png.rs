extern crate image;

use image::{ColorType, ImageFormat};

pub fn write_png(path: &str, width: u32, height: u32, data: &[u8]) {
     match image::save_buffer_with_format(path, data, width, height, ColorType::Rgb8, ImageFormat::Png) {
        Ok(()) => println!("Image saved to {:?}", path),
        Err(error) => panic!("Image could not be saved: {:?}", error)
     }
}