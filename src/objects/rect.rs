use std::sync::Arc;

use crate::materials::{Material, HitRecord};
use crate::hitable::Hitable;
use crate::structures::ray::Ray;
use crate::aabb::AABB;
use crate::structures::vec3::Vec3;

pub struct XyRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k:  f32,
    material: Arc<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        XyRect {
            x0, x1, y0, y1, k, material
        }
    }
}

impl Hitable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let hit_record = HitRecord::new(
            ray.at(t), 
            t, 
            ray, 
            &Vec3::new(0., 0., 1.), 
            (x - self.x0) / (self.x1 - self.x0), 
            (y - self.y0) / (self.y1 - self.y0), 
            &*self.material);
            
        Some(hit_record)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(
            AABB {
                min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
                max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
            }
        )
    }
}

pub struct XzRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k:  f32,
    material: Arc<dyn Material>,
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        XzRect {
            x0, x1, z0, z1, k, material
        }
    }
}

impl Hitable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let hit_record = HitRecord::new(
            ray.at(t), 
            t, 
            ray, 
            &Vec3::new(0., 1., 0.), 
            (x - self.x0) / (self.x1 - self.x0), 
            (z - self.z0) / (self.z1 - self.z0), 
            &*self.material);
            
        Some(hit_record)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(
            AABB {
                min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
                max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
            }
        )
    }
}

pub struct YzRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k:  f32,
    material: Arc<dyn Material>,
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        YzRect {
            y0, y1, z0, z1, k, material
        }
    }
}

impl Hitable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let hit_record = HitRecord::new(
            ray.at(t), 
            t, 
            ray, 
            &Vec3::new(1., 0., 0.), 
            (y - self.y0) / (self.y1 - self.y0), 
            (z - self.z0) / (self.z1 - self.z0), 
            &*self.material);
            
        Some(hit_record)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(
            AABB {
                min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
                max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
            }
        )
    }
}