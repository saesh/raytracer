use crate::structures::hittable::{Hittable, HitRecord};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::materials::material::Material;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32,  radius: f32, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {center0: center0, center1: center1, time0: time0, time1: time1, radius: radius, material: material}
    }

    pub fn material(&self) -> Box<dyn Material> {
        return dyn_clone::clone_box(&*self.material);
    }

    pub fn center(&self, time: f32) -> Vec3 {
        return self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0);
    }
}

impl Hittable for MovingSphere {
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
                let hit_record = HitRecord::new(hit_point, temp_t, ray, &outward_normal, self.material());

                return Some(hit_record);
            }

            let temp_t = (-half_b + root) / a;
            if temp_t < t_max && temp_t > t_min {
                let hit_point = ray.at(temp_t);
                let outward_normal = (hit_point - self.center(ray.time)) / self.radius;
                let hit_record = HitRecord::new(hit_point, temp_t, ray, &outward_normal, self.material());

                return Some(hit_record);
            } 
        }

        return None;
    }

    fn size(&self) -> usize {
        1
    }
}