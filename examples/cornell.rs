extern crate raytracer;

use std::sync::Arc;

use raytracer::structures::camera::Camera;
use raytracer::color::Color;
use raytracer::hitable::{Hitable, HitableList};
use raytracer::materials::{Lambertian, DiffuseLight};
use raytracer::structures::vec3::Vec3;
use raytracer::render;
use raytracer::io::png;
use raytracer::texture::*;
use raytracer::objects::rect::{XyRect, XzRect, YzRect};
use raytracer::objects::rectbox::RectBox;
use raytracer::objects::{Translate, RotateY};

fn main() {
    // image
    const ASPECT_RATIO: f32 = 1.0;
    let image_width = 400;
    let image_height = (image_width as f32 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

    // camera
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera: Camera = Camera::new(
        lookfrom, 
        lookat, 
        vup, 
        40.0, 
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0);

    // box
    let red   = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05))));
    let white = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73))));
    let green = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15))));
    let light = Arc::new(DiffuseLight::new(SolidColor::new(Color::new(15., 15., 15.))));

    let mut hitable_list = HitableList::default();

    hitable_list.push(YzRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, green)); 
    hitable_list.push(YzRect::new(  0.0, 555.0,   0.0, 555.0,   0.0,   red));
    hitable_list.push(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone()));
    hitable_list.push(XzRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, white.clone()));
    hitable_list.push(XzRect::new(  0.0, 555.0,   0.0, 555.0,   0.0, white.clone()));
    hitable_list.push(XyRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, white.clone()));

    hitable_list.push(Translate::translate(RotateY::new(RectBox::new(&Vec3::new(0., 0., 0.), &Vec3::new(165., 165., 165.), white.clone()), -18.0), &Vec3::new(130., 0.,  65.)));
    hitable_list.push(Translate::translate(RotateY::new(RectBox::new(&Vec3::new(0., 0., 0.), &Vec3::new(165., 330., 165.), white.clone()),  15.0), &Vec3::new(265., 0., 295.)));

    let world: Box<dyn Hitable> = Box::new(hitable_list);
    
    // render
    let image_data = render(&camera, &world, image_width, image_height, samples_per_pixel, max_depth);
    png::write_png("out/cornell.png", image_width, image_height, &image_data);
}