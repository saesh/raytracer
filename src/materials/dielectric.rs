use crate::io::random::{random_double};
use crate::structures::color::{Color, WHITE};
use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::{Vec3};
use crate::materials::material::Material;

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
        let cos_theta = (-1.0 * unit_direction).dot(&hit_record.normal);
        let cos_theta = if cos_theta < 1.0 { cos_theta } else { 1.0 };
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &hit_record.normal);
            let scattered_ray = Ray::new(hit_record.p, reflected, ray_in.time);
            return Some((WHITE, scattered_ray))
        } else if random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &hit_record.normal);
            let scattered_ray = Ray::new(hit_record.p, reflected, ray_in.time);
            return Some((WHITE, scattered_ray));
        } else {
            let refracted = Vec3::refract(&unit_direction, &hit_record.normal, etai_over_etat);
            let scattered_ray = Ray::new(hit_record.p, refracted, ray_in.time);
            return Some((WHITE, scattered_ray))
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0);
}