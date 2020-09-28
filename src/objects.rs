pub mod sphere;
pub mod moving_sphere;
pub mod triangle;

use crate::structures::ray::Ray;
use crate::materials::HitRecord;

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit_record: Option<HitRecord> = None;

        for object in self.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    closest_hit_record = Some(hit_record);
                },
                None => {}
            }
        }
        
        closest_hit_record
    }
}