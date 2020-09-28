extern crate raytracer;

use raytracer::objects::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::structures::color::{Color};
use raytracer::structures::hittable::{Hittable};
use raytracer::materials::lambertian::Lambertian;
use raytracer::materials::metal::Metal;
use raytracer::structures::vec3::{Vec3};
use raytracer::run;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: i32 = 500;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 500;
    let max_depth = 10;

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
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground))));

    let gold = Metal::new(Color::new(1.0, 0.84, 0.0), 0.1);
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.5, 0.0), 0.5, Box::new(gold))));

    let green = Metal::new(Color::new(0.2, 0.8, 0.5), 0.8);
    objects.push(Box::new(Sphere::new(Vec3::new(0.2, 0.05, 0.5), 0.05, Box::new(green))));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.05, 0.5), 0.05, Box::new(green))));
    objects.push(Box::new(Sphere::new(Vec3::new(-0.2, 0.05, 0.5), 0.05, Box::new(green))));

    // render
    run(camera, &objects, image_width, image_height, samples_per_pixel, max_depth);
}