use crate::structures::ray::Ray;
use crate::materials::HitRecord;
use crate::aabb::{AABB, surrounding_box};
use crate::structures::vec3::Vec3;

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Default)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn push(&mut self, hitable: impl Hitable + 'static) {
        self.list.push(Box::new(hitable));
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit_record: Option<HitRecord> = None;

        for object in self.list.iter() {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                closest_hit_record = Some(hit_record);
            }
        }
        
        closest_hit_record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.list.is_empty() {
            return None;
        }

        let mut output_box: AABB = AABB { min: Vec3::ZERO, max: Vec3::ZERO };
        let mut first_box = true;
        
        for object in self.list.iter() {
            match object.bounding_box(t0, t1) {
                Some(temp_box) => {
                    if first_box {
                        output_box = temp_box;
                    } else {
                        output_box = surrounding_box(&output_box, &temp_box);
                        first_box = false;
                    }
                },
                None => {
                    return None;
                },
            }
        }

        Some(output_box)
    }
}