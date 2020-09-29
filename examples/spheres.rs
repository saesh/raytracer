extern crate raytracer;

use std::sync::Arc;

use raytracer::objects::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::color::Color;
use raytracer::objects::Hitable;
use raytracer::materials::Lambertian;
use raytracer::materials::Metal;
use raytracer::structures::vec3::Vec3;
use raytracer::render;
use raytracer::io::png;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: u32 = 500;
    let image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth: u32 = 10;

    // camera
    let lookfrom = Vec3::new(0.0, 1.0, 3.0);
    let lookat = Vec3::new(0.0, 0.4, -0.5);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 3.2;
    let aperture = 0.01;

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
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));

    let gold = Arc::new(Metal::new(Color::new(1.0, 0.84, 0.0), 0.1));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.5, 0.0), 0.5, gold)));

    let green = Arc::new(Metal::new(Color::new(0.2, 0.8, 0.5), 0.8));
    objects.push(Box::new(Sphere::new(Vec3::new(0.2, 0.05, 0.5), 0.05, green.clone())));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.05, 0.5), 0.05, green.clone())));
    objects.push(Box::new(Sphere::new(Vec3::new(-0.2, 0.05, 0.5), 0.05, green.clone())));

    // render
    let image_data = render(camera, &mut objects, image_width, image_height, samples_per_pixel, max_depth);
    png::write_png("out/spheres.png", image_width, image_height, &image_data);
}