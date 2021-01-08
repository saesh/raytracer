extern crate raytracer;

use std::sync::Arc;

use raytracer::objects::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::color::{Color, BLACK, WHITE};
use raytracer::hitable::{Hitable, HitableList};
use raytracer::materials::{Lambertian, Metal, Dielectric, DiffuseLight};
use raytracer::structures::vec3::Vec3;
use raytracer::render;
use raytracer::io::png;
use raytracer::texture::*;
use raytracer::bvh::BVH;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth: u32 = 50;

    // camera
    let lookfrom = Vec3::new(0.0, 1.0, 3.0);
    let lookat = Vec3::new(0.0, 0.4, -0.5);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 2.5;
    let aperture = 0.06;

    let camera: Camera = Camera::new(
        lookfrom, 
        lookat, 
        vup, 
        40.0, 
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        0.1);

    // world
    let mut hitable_list = HitableList::default();
    let solid_color = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.6, 0.6, 0.6))));
    let _glass = Arc::new(Dielectric::new(1.5));
    let _metal = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.1), 1.5));
    let _checker = Arc::new(Lambertian::new(CheckerTexture::new(SolidColor::new(BLACK), SolidColor::new(WHITE))));
    
    // ground
    hitable_list.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000., solid_color));

    // light
    let difflight = Arc::new(DiffuseLight::new(SolidColor::new(Color::new(1., 0.95, 0.5))));
    let sun = Sphere::new(Vec3::new(0.0, 75.0, 0.0), 70.0, difflight);
    hitable_list.push(sun);

    // jupiter
    let jupiter = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(ImageTexture::new("files/jupiter2_1k.jpg"))));
    hitable_list.push(jupiter);

    // earth
    let earth = Sphere::new(Vec3::new(-0.4, 0.089, 0.5), 0.089, Arc::new(Lambertian::new(ImageTexture::new("files/earthmap.png"))));
    hitable_list.push(earth);
  
    // moon
    let moon = Sphere::new(Vec3::new(-0.3, 0.089 * 0.2731, 0.60), 0.089 * 0.2731, Arc::new(Lambertian::new(ImageTexture::new("files/moonmap1k.jpg"))));
    hitable_list.push(moon);
    
    let world: Box<dyn Hitable> = Box::new(BVH::new(hitable_list.list, 0.0, 0.0));
    
    // render
    let image_data = render(&camera, &world, image_width, image_height, samples_per_pixel, max_depth);
    png::write_png("out/spheres.png", image_width, image_height, &image_data);
}