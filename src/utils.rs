#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { 
        min 
    } else if x > max {
        max
    } else {
        x
    }
}