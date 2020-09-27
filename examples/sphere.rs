extern crate raytracer;

use raytracer::io::random;
use raytracer::geometry::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::structures::color::{Color};
use raytracer::structures::hittable::{Hittable};
use raytracer::structures::material::{Lambertian, Metal};
use raytracer::structures::vec3::{Vec3};
use raytracer::run;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: i32 = 500;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 50;
    let max_depth = 10;

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
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground))));

    let albedo = Color::random_bounded(0.5, 1.0);
    let fuzz = random::random_double_bounded(0.0, 0.5);
    let sphere_material = Metal::new(albedo, fuzz);
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.5, 0.0), 0.5, Box::new(sphere_material))));

    // render
    run(camera, objects, image_width, image_height, samples_per_pixel, max_depth);
}