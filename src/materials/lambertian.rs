use crate::structures::color::{Color};
use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::{random_unit_vector};
use crate::materials::material::Material;

// Lambertian
#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {albedo: albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = hit_record.normal + random_unit_vector();
        let scattered_ray = Ray::new(hit_record.p, scatter_direction, ray_in.time);
        return Some((self.albedo, scattered_ray))
    }
}