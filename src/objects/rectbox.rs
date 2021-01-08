use std::sync::Arc;

use crate::structures::vec3::Vec3;
use crate::hitable::{Hitable, HitableList};
use crate::materials::{Material, HitRecord};
use crate::structures::ray::Ray;
use crate::objects::rect::{XyRect, XzRect, YzRect};
use crate::aabb::AABB;

pub struct RectBox {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub hitable: Box<dyn Hitable>,
}

impl RectBox {
    pub fn new(p0: &Vec3, p1: &Vec3, material: Arc<dyn Material>) -> Self {
        let mut hitable_list = HitableList::default();
        hitable_list.push(XyRect::new( 
            p0.x, p1.x, 
            p0.y, p1.y, 
            p1.z, 
            material.clone()
        ));

        hitable_list.push(XyRect::new( 
            p0.x, p1.x, 
            p0.y, p1.y, 
            p0.z, 
            material.clone()
        ));

        hitable_list.push(XzRect::new( 
            p0.x, p1.x, 
            p0.z, p1.z, 
            p1.y, 
            material.clone()
        ));

        hitable_list.push(XzRect::new( 
            p0.x, p1.x, 
            p0.z, p1.z, 
            p0.y, 
            material.clone()
        ));

        hitable_list.push(YzRect::new( 
            p0.y, p1.y, 
            p0.z, p1.z, 
            p1.x, 
            material.clone()
        ));

        hitable_list.push(YzRect::new( 
            p0.y, p1.y, 
            p0.z, p1.z, 
            p0.x, 
            material.clone()
        ));

        RectBox {
            pmin: *p0,
            pmax: *p1,
            hitable: Box::new(hitable_list),
        }
    }
}

impl Hitable for RectBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitable.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(
            AABB {
                min: self.pmin,
                max: self.pmax,
            }
        )
    }
}