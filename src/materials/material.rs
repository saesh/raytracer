use crate::structures::color::{Color};
use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;

use dyn_clone::DynClone;

pub trait Material: DynClone + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

dyn_clone::clone_trait_object!(Material);