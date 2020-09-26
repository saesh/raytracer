mod io;
mod structures;

use std::time::{Instant};
use std::f32::{INFINITY};

use io::ppm;
use io::random;
use structures::camera::Camera;
use structures::color::*;
use structures::geometry::Sphere;
use structures::hittable::{Hittable, HitRecord};
use structures::ray::Ray;
use structures::vec3::*;
use structures::material::{Lambertian, Metal};

fn main() {
    let start = Instant::now();

    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width: i32= 800;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // camera
    let camera: Camera = Camera::new();

    // world
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left   = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right  = Metal::new(Color::new(0.8, 0.6, 0.2));

    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(material_ground))));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0,    0.0, -1.0), 0.5,   Box::new(material_center))));
    objects.push(Box::new(Sphere::new(Vec3::new(-1.0,   0.0, -1.0), 0.5,   Box::new(material_left))));
    objects.push(Box::new(Sphere::new(Vec3::new( 1.0,   0.0, -1.0), 0.5,   Box::new(material_right))));

    // render
    ppm::write_header(image_width, image_height);

    for pixel_y in (0..image_height).rev() {

        eprint!("\rScanlines remaining: {} ", pixel_y);

        for pixel_x in 0..image_width {

            let mut pixel_color = BLACK;

            for _s in 0..samples_per_pixel {
                let u: f32 = (pixel_x as f32 + random::random_double()) / (image_width as f32 - 1.0);
                let v: f32 = (pixel_y as f32 + random::random_double()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &objects, max_depth);
            }

            ppm::write_pixel(pixel_color, samples_per_pixel);
        }
    }

    let duration = start.elapsed();
    eprintln!("\nDone in {:?}.", duration);
}

fn ray_color(ray: &Ray, objects: &Vec<Box<dyn Hittable>>, depth: i32) -> Color {
    if depth <= 0 {
        return BLACK
    }

    match hit_scene(ray, objects) {
        Some(hit_record) => {
            return match hit_record.material.scatter(ray, &hit_record) {
                Some((color, new_ray)) => color * ray_color(&new_ray, objects, depth - 1),
                None => BLACK
            }
        },
        None => background_color(&ray),
    }
}

fn hit_scene(ray: &Ray, objects: &Vec<Box<dyn Hittable>>) -> Option<HitRecord> {
    let mut closest_so_far = INFINITY;
    let mut closest_hit_record: Option<HitRecord> = None;

    for object in objects {
        match object.hit(ray, 0.001, closest_so_far) {
            Some(hit_record) => {
                closest_so_far = hit_record.t;
                closest_hit_record = Some(hit_record);
            },
            None => {}
        }
    }
    
    return closest_hit_record;
}

fn background_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);

    return linear_blend(t, WHITE, Color::new(0.5, 0.7, 1.0));
}