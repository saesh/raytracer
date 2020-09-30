use std::sync::Arc;

use crate::hitable::Hitable;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::materials::{Material, HitRecord};
use crate::aabb::AABB;

const EPSILON: f32 = 0.0000001;

pub struct Triangle {
    vertex0: Vec3,
    vertex1: Vec3,
    vertex2: Vec3,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(vertex0: Vec3, vertex1: Vec3, vertex2: Vec3, material: Arc<dyn Material>) -> Self {
        Triangle {vertex0: vertex0, vertex1: vertex1, vertex2: vertex2, material: material}
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
         
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;

        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.vertex0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        if t < t_max && t > t_min {
            let hit_point = ray.at(t);
            let normal = edge1.cross(&edge2).normalize();

            return Some(
                HitRecord::new(hit_point, t, 0.0, 0.0, ray, &normal, &*self.material)
            );
        }

        return None;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(
              self.vertex0.x.min(self.vertex1.x.min(self.vertex2.x)),
              self.vertex0.y.min(self.vertex1.y.min(self.vertex2.y)),
              self.vertex0.z.min(self.vertex1.z.min(self.vertex2.z))),
              
            max: Vec3::new(
              self.vertex0.x.max(self.vertex1.x.max(self.vertex2.x)),
              self.vertex0.y.max(self.vertex1.y.max(self.vertex2.y)),
              self.vertex0.z.max(self.vertex1.z.max(self.vertex2.z))),
          })
    }
}