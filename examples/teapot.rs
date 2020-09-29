extern crate raytracer;

use std::sync::Arc;

use raytracer::io::png;
use raytracer::io::obj::load_teapot;
use raytracer::objects::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::color::*;
use raytracer::objects::Hitable;
use raytracer::materials::Lambertian;
use raytracer::structures::vec3::*;
use raytracer::run;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: u32 = 500;
    let image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    let samples_per_pixel: u32 = 300;
    let max_depth: u32 = 50;

    // camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera: Camera = Camera::new(
        lookfrom, 
        lookat, 
        vup, 
        20.0, 
        ASPECT_RATIO,
        aperture,
        dist_to_focus, 
        0.0, 
        0.1);

    // world
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();

    match load_teapot() {
        Some(triangles) => { for triangle in triangles { objects.push(Box::new(triangle)) }},
        None => {}
    };

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));

    let image_data = run(camera, &mut objects, image_width, image_height, samples_per_pixel, max_depth);
    
    png::write_png("out/teapot.png", image_width, image_width, &image_data);
}