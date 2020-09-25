use crate::structures::hittable::{Hittable, HitRecord};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {center: center, radius: radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        // optimized version of the quadratic formular components
        // see: https://raytracing.github.io/books/RayTracingInOneWeekend.html#surfacenormalsandmultipleobjects/simplifyingtheray-sphereintersectioncode
        let a = &ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
    
        // discriminant = 0 -> no intersection
        // discriminant = 1 -> one intersection
        // discriminant = 2 -> two intersections
        if discriminant < 0.0 {
            None
        } else {
            Some(HitRecord {
                p: Vec3::ZERO,
                normal: Vec3::ZERO,
                t: (-half_b - discriminant.sqrt()) / a
            })
        }
    }
}