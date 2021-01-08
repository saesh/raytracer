use std::sync::Arc;

use crate::hitable::Hitable;
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::materials::{Material, HitRecord};
use crate::aabb::{AABB, surrounding_box};
use crate::objects::sphere::Sphere;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32,  radius: f32, material: Arc<dyn Material>) -> Self {
        MovingSphere {center0, center1, time0, time1, radius, material}
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
                let outward_normal = (hit_point - self.center(ray.time)) / self.radius;
                let (u, v) = Sphere::get_sphere_uv((hit_point - self.center(ray.time)) / self.radius);
                let hit_record = HitRecord::new(hit_point, temp_t, ray, &outward_normal, u, v, &*self.material);

                return Some(hit_record);
            }

            let temp_t = (-half_b + root) / a;
            if temp_t < t_max && temp_t > t_min {
                let hit_point = ray.at(temp_t);
                let outward_normal = (hit_point - self.center(ray.time)) / self.radius;
                let (u, v) = Sphere::get_sphere_uv((hit_point - self.center(ray.time)) / self.radius);
                let hit_record = HitRecord::new(hit_point, temp_t, ray, &outward_normal, u, v, &*self.material);

                return Some(hit_record);
            } 
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB {
            min: self.center(t0) - Vec3 { x: self.radius, y: self.radius, z: self.radius },
            max: self.center(t0) + Vec3 { x: self.radius, y: self.radius, z: self.radius },
        };

        let box1 = AABB {
            min: self.center(t1) - Vec3 { x: self.radius, y: self.radius, z: self.radius },
            max: self.center(t1) + Vec3 { x: self.radius, y: self.radius, z: self.radius },
        };

        Some(surrounding_box(&box0, &box1))
    }
}