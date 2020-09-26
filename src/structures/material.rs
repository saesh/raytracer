use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::structures::color::Color;
use crate::structures::vec3::random_unit_vector;

use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

dyn_clone::clone_trait_object!(Material);

// Lambertian
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

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Lambertian {
            albedo: self.albedo
        }
    }
}

// Metal
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal {albedo: albedo}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&ray_in.direction.normalize(), &hit_record.normal);
        let scattered_ray = Ray::new(hit_record.p, reflected);

        if Vec3::dot(&scattered_ray.direction, &hit_record.normal) > 0.0 {
            return Some((self.albedo, scattered_ray))
        } else {
            return None
        }
    }
}

impl Clone for Metal {
    fn clone(&self) -> Self {
        Metal {
            albedo: self.albedo
        }
    }
}
