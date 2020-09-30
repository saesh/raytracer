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

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
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