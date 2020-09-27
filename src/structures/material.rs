use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::structures::color::{Color, WHITE};
use crate::structures::vec3::{random_unit_vector, random_in_unit_sphere};
use crate::random::random_double;

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

// Dielectric
#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {ref_idx: ref_idx}
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let etai_over_etat = if hit_record.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = Vec3::dot(&(-1.0 * unit_direction), &hit_record.normal);
        let cos_theta = if cos_theta < 1.0 { cos_theta } else { 1.0 };
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &hit_record.normal);
            let scattered_ray = Ray::new(hit_record.p, reflected);
            return Some((WHITE, scattered_ray))
        } else if random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &hit_record.normal);
            let scattered_ray = Ray::new(hit_record.p, reflected);
            return Some((WHITE, scattered_ray));
        } else {
            let refracted = Vec3::refract(&unit_direction, &hit_record.normal, etai_over_etat);
            let scattered_ray = Ray::new(hit_record.p, refracted);
            return Some((WHITE, scattered_ray))
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0);
}