use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self { AABB { min, max } }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {

            let inv_d = 1.0 / ray.direction.get(a);
            let t0 = (self.min.get(a) - ray.origin.get(a)) * inv_d;
            let t1 = (self.max.get(a) - ray.origin.get(a)) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            
            if t_max <= t_min {
                return false;
            }
        }
        
        return true;
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3 {
        x: box0.min.x.min(box1.min.x),
        y: box0.min.y.min(box1.min.y),
        z: box0.min.z.min(box1.min.z),
    };

    let big = Vec3 {
        x: box0.max.x.max(box1.max.x),
        y: box0.max.y.max(box1.max.y),
        z: box0.max.z.max(box1.max.z),
    };

    AABB {
        min: small,
        max: big,
    }
}