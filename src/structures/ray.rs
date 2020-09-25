use crate::structures::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}