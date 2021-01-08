extern crate raytracer;

use std::sync::Arc;

use raytracer::io::png;
use raytracer::io::obj::load_file;
use raytracer::objects::sphere::Sphere;
use raytracer::structures::camera::Camera;
use raytracer::color::*;
use raytracer::hitable::{Hitable, HitableList};
use raytracer::materials::{Lambertian, Dielectric, Metal, DiffuseLight};
use raytracer::structures::vec3::*;
use raytracer::render;
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
    let lookfrom = Vec3::new(0.0, 8.0, 10.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera: Camera = Camera::new(
        lookfrom, 
        lookat, 
        vup, 
        30.0, 
        ASPECT_RATIO,
        aperture,
        dist_to_focus, 
        0.0, 
        1.0);

    // world
    let world = world();

    let image_data = render(&camera, &world, image_width, image_height, samples_per_pixel, max_depth);
    
    png::write_png("out/teapot.png", image_width, image_height, &image_data);
}

fn world() -> Box<dyn Hitable> {
    let mut world = HitableList::default();
    
    // ground
    let material_ground = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.5, 0.5, 0.5))));
    world.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    // teapot
    if let Some(model) = load_file("files/teapot.obj", Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 10.0))) {    
        world.push(BVH::new(model, 0.0, 0.0));
    };

    // spheres
    let glass = Arc::new(Dielectric::new(1.5));
    world.push(Sphere::new(Vec3::new(-1.8, 0.5, 2.5), 0.5, glass.clone()));

    let earth = Sphere::new(Vec3::new(-2.5, 0.8, 1.6), 0.8, Arc::new(Lambertian::new(ImageTexture::new("files/earthmap.png"))));
    world.push(earth);

    // light
    let main_light_color = Arc::new(DiffuseLight::new(SolidColor::new(Color::new(1.0, 0.95, 0.95))));
    let main_light = Sphere::new(Vec3::new(-100.0, 75.0, 0.0), 70.0, main_light_color);
    world.push(main_light);

    let other_light_color = Arc::new(DiffuseLight::new(SolidColor::new(Color::new(1.0, 0.95, 0.75))));
    let second_light = Sphere::new(Vec3::new(50.0, 75.0, -30.0), 40.0, other_light_color.clone());
    world.push(second_light);

    Box::new(BVH::new(world.list, 0.0, 0.0))
}