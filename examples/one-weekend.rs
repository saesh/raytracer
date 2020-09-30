extern crate raytracer;

use std::sync::Arc;

use raytracer::random::{random_double, random_double_bounded};
use raytracer::objects::sphere::Sphere;
use raytracer::objects::moving_sphere::MovingSphere;
use raytracer::structures::camera::Camera;
use raytracer::color::Color;
use raytracer::hitable::{Hitable, HitableList};
use raytracer::materials::Dielectric;
use raytracer::materials::Lambertian;
use raytracer::materials::Metal;
use raytracer::structures::vec3::Vec3;
use raytracer::render;
use raytracer::io::png;
use raytracer::bvh::BVH;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: u32 = 200;
    let image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    let samples_per_pixel: u32 = 10;
    let max_depth: u32 = 10;

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
        1.0);

    // world
    let mut world = HitableList::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for a in -11..11 {
        
        for b in -11..11 {
        
            let choose_mat = random_double();
            let center = Vec3::new(a as f32 + 0.9 * random_double(), 0.2, b as f32 + 0.9 * random_double());
            
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));

                    let center2 = center + Vec3::new(0.0, random_double_bounded(0.0, 0.5), 0.0);

                    world.push(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_bounded(0.5, 1.0);
                    let fuzz = random_double_bounded(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3  = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vec3::new(4.0,1.0, 0.0), 1.0, material3));

    let world: Box<dyn Hitable> = Box::new(BVH::new(world.list, 0.0, 1.0));

    // render
    let image_data = render(camera, &world, image_width, image_height, samples_per_pixel, max_depth);
    png::write_png("out/one-weekend.png", image_width, image_height, &image_data);
}