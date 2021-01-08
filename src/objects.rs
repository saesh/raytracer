pub mod sphere;
pub mod moving_sphere;
pub mod triangle;
pub mod rect;
pub mod rectbox;

use std::f32::consts::PI;
use std::f32::MAX;

use crate::structures::vec3::Vec3;
use crate::hitable::Hitable;
use crate::materials::HitRecord;
use crate::aabb::AABB;
use crate::structures::ray::Ray;

pub struct Translate {
    offset: Vec3,
    hitable: Box<dyn Hitable>,
}

impl Translate {
    pub fn translate(hitable: impl Hitable + 'static, displacement: &Vec3) -> Self {
        Translate {
            offset: *displacement,
            hitable: Box::new(hitable),
        }
    }
}

impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if let Some(hit_record) = self.hitable.hit(&moved_r, t_min, t_max) {
            Some(
                HitRecord {
                    p: hit_record.p + self.offset,
                    ..hit_record
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bounding_box) = self.hitable.bounding_box(t0, t1) {
            Some(
                AABB {
                    min: bounding_box.min + self.offset,
                    max: bounding_box.max + self.offset,
                }
            )
        } else {
            None
        }
    }
}

pub struct RotateY {
    hitable: Box<dyn Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}

impl RotateY {
    pub fn new(hitable: impl Hitable + 'static, angle: f32) -> Self {
        let radians = (PI / 180.) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut min = Vec3::new(MAX, MAX, MAX);
        let mut max = Vec3::new(-MAX, -MAX, -MAX);

        let bbox = hitable.bounding_box(0., 1.).unwrap();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.max.x + ((1 - i) as f32) * bbox.min.x;
                    let y = j as f32 * bbox.max.y + ((1 - j) as f32) * bbox.min.y;
                    let z = k as f32 * bbox.max.z + ((1 - k) as f32) * bbox.min.z;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    
                    if tester.x > max.x {
                        max = Vec3 {
                            x: tester.x,
                            ..max
                        }
                    }
                    if tester.x < max.x {
                        min = Vec3 {
                            x: tester.x,
                            ..min
                        }
                    }
                    if tester.y > max.y {
                        max = Vec3 {
                            y: tester.y,
                            ..max
                        }
                    }
                    if tester.y < max.y {
                        min = Vec3 {
                            z: tester.z,
                            ..min
                        }
                    }
                }
            }
        }

        RotateY {
            hitable: Box::new(hitable),
            sin_theta,
            cos_theta,
            bbox: AABB {min, max }
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = ray.origin;
        let direction = ray.direction;

        let origin = Vec3 { x: self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z, ..origin }; 
        let origin = Vec3 { z: self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z, ..origin }; 

        let direction = Vec3 { x: self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z, ..direction };
        let direction = Vec3 { z: self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z, ..direction };
        
        let rotated_r = Ray::new(origin, direction, ray.time);

        if let Some(hit_record) = self.hitable.hit(&rotated_r, t_min, t_max) {
            let p = hit_record.p;
            let normal = hit_record.normal;
            let p = Vec3 { x: self.cos_theta * hit_record.p.x + self.sin_theta * hit_record.p.z, ..p };
            let p = Vec3 { z: -self.sin_theta * hit_record.p.x + self.cos_theta * hit_record.p.z, ..p };
            let normal = Vec3 { x: self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z, ..normal };
            let normal = Vec3 { z: -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z, ..normal };

            Some(
                HitRecord {
                    p,
                    normal,
                    ..hit_record
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}