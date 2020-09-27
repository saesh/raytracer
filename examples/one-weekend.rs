extern crate raytracer;

use raytracer::io::random;
use raytracer::objects::sphere::Sphere;
use raytracer::objects::moving_sphere::MovingSphere;
use raytracer::structures::camera::Camera;
use raytracer::structures::color::{Color};
use raytracer::structures::hittable::{Hittable};
use raytracer::materials::dielectric::Dielectric;
use raytracer::materials::lambertian::Lambertian;
use raytracer::materials::metal::Metal;
use raytracer::structures::vec3::{Vec3};
use raytracer::run;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: i32 = 820;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

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
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground))));

    for a in -11..11 {
        
        for b in -11..11 {
        
            let choose_mat = random::random_double();
            let center = Vec3::new(a as f32 + 0.9 * random::random_double(), 0.2, b as f32 + 0.9 * random::random_double());
            
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);

                    let center2 = center + Vec3::new(0.0, random::random_double_bounded(0.0, 0.5), 0.0);

                    objects.push(Box::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, Box::new(sphere_material))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_bounded(0.5, 1.0);
                    let fuzz = random::random_double_bounded(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    objects.push(Box::new(Sphere::new(center, 0.2, Box::new(sphere_material))));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    objects.push(Box::new(Sphere::new(center, 0.2, Box::new(sphere_material))));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(material1))));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    objects.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(material2))));

    let material3  = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    objects.push(Box::new(Sphere::new(Vec3::new(4.0,1.0, 0.0), 1.0, Box::new(material3))));

    // render
    run(camera, objects, image_width, image_height, samples_per_pixel, max_depth);
}