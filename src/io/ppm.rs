#[inline]
pub fn write_header(image_width: i32, image_height: i32) {
    println!("P3\n{} {}\n255", image_width, image_height);
}

pub fn write_pixel(r: i32, g: i32, b: i32) {
    
    println!("{} {} {}", r, g, b);
}