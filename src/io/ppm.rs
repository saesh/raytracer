use crate::structures::color::Color;

pub fn write_header(image_width: i32, image_height: i32) {
    println!("P3\n{} {}\n255", image_width, image_height);
}

pub fn write_pixel(color: Color) {
    let r = (255.999 * color.r) as i32;
    let g = (255.999 * color.g) as i32;
    let b = (255.999 * color.b) as i32;

    println!("{} {} {}", r, g, b);
}
