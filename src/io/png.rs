extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub fn write_png(path: &str, width: u32, height: u32, data: &Vec<u8>) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap();
}