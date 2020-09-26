use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::structures::color::Color;
use crate::structures::vec3::{random_unit_vector, random_in_unit_sphere};

use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

dyn_clone::clone_trait_object!(Material);

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
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = hit_record.normal + random_unit_vector();
        let scattered_ray = Ray::new(hit_record.p, scatter_direction);
        return Some((self.albedo, scattered_ray))
    }
}

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
        let scattered_ray = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere());

        if Vec3::dot(&scattered_ray.direction, &hit_record.normal) > 0.0 {
            return Some((self.albedo, scattered_ray))
        } else {
            return None
        }
    }
}
