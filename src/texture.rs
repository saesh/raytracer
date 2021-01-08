use crate::color::Color;
use crate::structures::vec3::Vec3;
use crate::utils;

pub trait Texture: Sync + Send {
    fn color(&self, u: f32, v: f32, p: &Vec3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor { color_value: color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _p: &Vec3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture<T: Texture> {
    odd: T,
    even: T,
}

impl<T: Texture> CheckerTexture<T> {
    pub fn new(odd: T, even: T) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture> Texture for CheckerTexture<T> {
    fn color(&self, u: f32, v: f32, p: &Vec3) -> Color {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();

        if sines < 0.0 {
            self.odd.color(u, v, p)
        } else {
            self.even.color(u, v, p)
        }
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl ImageTexture {
    pub fn new(path: &str) -> Self { 
        let image = image::open(path).expect("Texture image could not be opened").to_rgb();
        let (width, height) = image.dimensions();
        let data = image.into_raw();
        
        ImageTexture { data, width: width as usize, height: height as usize } 
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f32, v: f32, _p: &Vec3) -> Color {
        let u = utils::clamp(u, 0., 1.);
        let v = 1. - utils::clamp(v, 0., 1.);

        let mut i = (u * self.width as f32) as usize;
        let mut j = (v * self.height as f32) as usize;

        if i >= self.width  { i = self.width - 1 };
        if j >= self.height { j = self.height - 1 };

        let color_scale = 1. / 255.;
        let bytes_per_pixel = 3;
        let bytes_per_scanline = 3;

        let pixel_idx = j * bytes_per_scanline * self.width + i * bytes_per_pixel;
        
        let r = self.data[pixel_idx]     as f32 * color_scale;
        let g = self.data[pixel_idx + 1] as f32 * color_scale;
        let b = self.data[pixel_idx + 2] as f32 * color_scale;

        Color::new(r, g, b)
    }
}