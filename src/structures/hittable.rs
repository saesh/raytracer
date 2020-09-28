use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;
use crate::materials::material::Material;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn size(&self) -> usize;
}

pub struct HitRecord {
    pub p: Vec3,
    pub t: f32,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f32, ray: &Ray, outward_normal: &Vec3, material: Box<dyn Material>) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        HitRecord {
            p: p,
            t: t,
            front_face: front_face,
            normal: match front_face { 
                true => Vec3::new(outward_normal.x, outward_normal.y, outward_normal.z),
                false => -1.0 * Vec3::new(outward_normal.x, outward_normal.y, outward_normal.z)
            },
            material: material,
        }
    }
}