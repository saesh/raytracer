use std::ops::{Add, Mul};

use crate::io::random;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {r: r, g: g, b: b}
    }

    pub fn random() -> Color {
        Color {
            r: random::random_double(),
            g: random::random_double(),
            b: random::random_double(),
        }
    }

    pub fn random_bounded(min: f32, max: f32) -> Color {
        Color {
            r: random::random_double_bounded(min, max),
            g: random::random_double_bounded(min, max),
            b: random::random_double_bounded(min, max),
        }
    }
}

pub const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0};
pub const WHITE: Color = Color {r: 1.0, g: 1.0, b: 1.0};

pub fn linear_blend(t: f32, start: Color, end: Color) -> Color {
    return (1.0 - t) * start + t * end;
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {
            r: self * _rhs.r,
            g: self * _rhs.g,
            b: self * _rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, _rhs: f32) -> Color {
        Color {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs,
        }
    }
}