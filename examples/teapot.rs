extern crate raytracer;

use raytracer::io::obj::{load_model, load_model_triangles};
use raytracer::objects::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::structures::color::*;
use raytracer::structures::hittable::Hittable;
use raytracer::materials::lambertian::Lambertian;
use raytracer::structures::vec3::*;
use raytracer::run;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: i32 = 500;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 5;
    let max_depth = 10;

    // camera
    let lookfrom = Vec3::new(13.0, 4.0, 3.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
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

    match load_model_triangles("files/teapot.obj") {
        Some(triangles) => { for triangle in triangles { objects.push(Box::new(triangle)) }},
        None => {}
    };

    // match load_model("files/teapot.obj") {
    //     Some(model) => objects.push(Box::new(model)),
    //     None => {}
    // };

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground))));

    run(camera, &objects, image_width, image_height, samples_per_pixel, max_depth);   
}