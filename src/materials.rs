use crate::random::random_double;
use crate::color::{Color, WHITE};
use crate::structures::ray::Ray;
use crate::structures::vec3::{Vec3, random_in_unit_sphere};
use crate::texture::Texture;
use crate::color::BLACK;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, t: f32, ray: &Ray, outward_normal: &Vec3, u: f32, v: f32, material: &'a dyn Material) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        HitRecord {
            p,
            t,
            u,
            v,
            front_face,
            normal: match front_face { 
                true  =>        Vec3::new(outward_normal.x, outward_normal.y, outward_normal.z),
                false => -1.0 * Vec3::new(outward_normal.x, outward_normal.y, outward_normal.z)
            },
            material,
        }
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, u: f32, v: f32, hit_record: &HitRecord) -> Color;
}

pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered_ray = Ray::new(hit_record.p, target - hit_record.p, ray_in.time);
        
        Some((self.albedo.color(hit_record.u, hit_record.v, &hit_record.p), scattered_ray))
    }

    fn emitted(&self, _u: f32, _v: f32, _hit_record: &HitRecord) -> Color {
        BLACK
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&ray_in.direction.normalize(), &hit_record.normal);
        let scattered_ray = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere(), ray_in.time);

        if scattered_ray.direction.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }

    fn emitted(&self, _u: f32, _v: f32, _hit_record: &HitRecord) -> Color {
        BLACK
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
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

        if etai_over_etat * sin_theta > 1.0 || random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &hit_record.normal);
            let scattered_ray = Ray::new(hit_record.p, reflected, ray_in.time);
            
            Some((WHITE, scattered_ray))
        } else {
            let refracted = Vec3::refract(&unit_direction, &hit_record.normal, etai_over_etat);
            let scattered_ray = Ray::new(hit_record.p, refracted, ray_in.time);
            
            Some((WHITE, scattered_ray))
        }
    }

    fn emitted(&self, _u: f32, _v: f32, _hit_record: &HitRecord) -> Color {
        BLACK
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
}

pub struct DiffuseLight<T: Texture> {
    emit: T
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        DiffuseLight { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, hit_record: &HitRecord) -> Color {
        self.emit.color(u, v, &hit_record.p)
    }
}