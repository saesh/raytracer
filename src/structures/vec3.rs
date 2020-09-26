use std::ops::{Add, Sub, Mul, Div};
use std::f32::consts::PI;

use crate::io::random;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub const ZERO: Vec3 = Vec3 {
        x: 0.0, y: 0.0, z: 0.0
    };

    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn normalize(self) -> Vec3 {
        return self / self.length()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        return v - 2.0 * Vec3::dot(v, n) * n;
    }
}

pub fn random_bounded(min: f32, max: f32) -> Vec3 {
    Vec3 {
        x: random::random_double_bounded(min, max),
        y: random::random_double_bounded(min, max),
        z: random::random_double_bounded(min, max),
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_bounded(-1.0, 1.0);
        if p.length_squared() >= 1.0 { continue };
        return p;
    }
}

// Lambertian distribution
pub fn random_unit_vector() -> Vec3 {
    let a = random::random_double_bounded(0.0, 2.0 * PI);
    let z = random::random_double_bounded(-1.0, 1.0);
    let r = (1.0 - z*z).sqrt();

    Vec3 {
        x: r * a.cos(),
        y: r * a.sin(),
        z: z
    }
}

// hemispherical scattering
// pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
//     let in_unit_sphere = random_in_unit_sphere();
//     if Vec3::dot(&in_unit_sphere, &normal) > 0.0 {
//         return in_unit_sphere;
//     } else {
//         return -1.0 * in_unit_sphere;
//     }
// }

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::dot(&a, &b), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
    }

    #[test]
    fn length() {
        let a = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(a.length(), 3_f32.sqrt())
    }

    #[test]
    fn normalize() {
        let a = Vec3::new(1.0, 2.0, 3.0);

        assert!(a.normalize().length() - 1. < 0.0000001)
    }
}