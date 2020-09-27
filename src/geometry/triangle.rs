use crate::structures::hittable::{Hittable, HitRecord};
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::structures::material::Material;

const EPSILON: f32 = 0.0000001;

pub struct Triangle {
    vertex0: Vec3,
    vertex1: Vec3,
    vertex2: Vec3,
    material: Box<dyn Material>,
}

impl Triangle {
    pub fn new(vertex0: Vec3, vertex1: Vec3, vertex2: Vec3, material: Box<dyn Material>) -> Triangle {
        Triangle {vertex0: vertex0, vertex1: vertex1, vertex2: vertex2, material: material}
    }

    pub fn material(&self) -> Box<dyn Material> {
        return dyn_clone::clone_box(&*self.material);
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
         
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;

        let h = Vec3::cross(&ray.direction, &edge2);
        let a = Vec3::dot(&edge1, &h);

        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.vertex0;
        let u = f * Vec3::dot(&s, &h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = Vec3::cross(&s, &edge1);
        let v = f * Vec3::dot(&ray.direction, &q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * Vec3::dot(&edge2, &q);
        if t < t_max && t > t_min {
            let hit_point = ray.at(t);
            let normal = Vec3::cross(&edge1, &edge2).normalize();

            return Some(
                HitRecord::new(hit_point, t, ray, &normal, self.material())
            );
        }

        return None;
    }
}