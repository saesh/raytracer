use crate::structures::color::{Color};
use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::{Vec3, random_in_unit_sphere};
use crate::materials::material::Material;

// Metal
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {albedo: albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&ray_in.direction.normalize(), &hit_record.normal);
        let scattered_ray = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere(), ray_in.time);

        if scattered_ray.direction.dot(&hit_record.normal) > 0.0 {
            return Some((self.albedo, scattered_ray))
        } else {
            return None
        }
    }
}