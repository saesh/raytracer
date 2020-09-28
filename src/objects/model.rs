use crate::structures::hittable::{Hittable, HitRecord};
use crate::structures::ray::Ray;
use crate::objects::triangle::Triangle;

pub struct Model {
    pub triangles: Vec<Triangle>
}

impl Model {
    pub fn new(triangles: Vec<Triangle>) -> Model {
        Model {triangles: triangles}
    }

    pub fn size(&self) -> usize {
        self.triangles.len()
    }
}

impl Hittable for Model {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let mut hit_record = None;

        for triangle in &self.triangles {
            match triangle.hit(ray, t_min, t_max) {
                Some(hr) => hit_record = Some(hr),
                None => continue
            }
        }

        return hit_record;
    }

    fn size(&self) -> usize {
        self.triangles.len()
    }
}