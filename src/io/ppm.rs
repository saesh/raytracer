use crate::structures::color::Color;

#[inline]
pub fn write_header(image_width: i32, image_height: i32) {
    println!("P3\n{} {}\n255", image_width, image_height);
}

pub fn write_pixel(color: Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f32;

    let r = color.r * scale;
    let g = color.g * scale;
    let b = color.b * scale;

    println!("{} {} {}", 
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32, 
        (256.0 * clamp(b, 0.0, 0.999)) as i32);
}

#[inline]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    return if x < min { 
        min 
    } else if x > max {
        max
    } else {
        x
    };
}