use std::sync::Arc;

use crate::objects::{Hitable, HitRecord};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::materials::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere{
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere {center: center, radius: radius, material: material}
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        // optimized version of the quadratic formular components
        // see: https://raytracing.github.io/books/RayTracingInOneWeekend.html#surfacenormalsandmultipleobjects/simplifyingtheray-sphereintersectioncode
        let a = &ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
    
        // discriminant = 0 -> no intersection
        // discriminant = 1 -> one intersection
        // discriminant = 2 -> two intersections
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp_t = (-half_b - root) / a;
            if temp_t < t_max && temp_t > t_min {
                let hit_point = ray.at(temp_t);
                let outward_normal = (hit_point - self.center) / self.radius;
                let hit_record = HitRecord::new(hit_point, temp_t, ray, &outward_normal, &*self.material);

                return Some(hit_record);
            }

            let temp_t = (-half_b + root) / a;
            if temp_t < t_max && temp_t > t_min {
                let hit_point = ray.at(temp_t);
                let outward_normal = (hit_point - self.center) / self.radius;
                let hit_record = HitRecord::new(hit_point, temp_t, ray, &outward_normal, &*self.material);

                return Some(hit_record);
            } 
        }

        return None;
    }
}