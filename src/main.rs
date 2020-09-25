mod io;
mod structures;

use std::time::{Instant};
use std::f32::{INFINITY};

use io::ppm;
use structures::camera::Camera;
use structures::color::*;
use structures::geometry::Sphere;
use structures::hittable::{Hittable, HitRecord};
use structures::ray::Ray;
use structures::vec3::Vec3;

use rand::Rng;

fn main() {
    let start = Instant::now();

    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width: i32= 400;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 100;

    // camera
    let camera: Camera = Camera::new();

    // world
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // render
    ppm::write_header(image_width, image_height);

    for pixel_y in (0..image_height).rev() {

        eprint!("\rScanlines remaining: {} ", pixel_y);

        for pixel_x in 0..image_width {

            let mut pixel_color = BLACK;

            for _s in 0..samples_per_pixel {
                let u: f32 = (pixel_x as f32 + random_double()) / (image_width as f32 - 1.0);
                let v: f32 = (pixel_y as f32 +random_double()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &objects);
            }

            ppm::write_pixel(pixel_color, samples_per_pixel);
        }
    }

    let duration = start.elapsed();
    eprintln!("\nDone in {:?}.", duration);
}

fn ray_color(ray: &Ray, objects: &Vec<Box<dyn Hittable>>) -> Color {
    match hit_scene(ray, objects) {
        Some(hit_record) => compute_normal_color(&hit_record),
        None => background_color(&ray),
    }
}

fn hit_scene(ray: &Ray, objects: &Vec<Box<dyn Hittable>>) -> Option<HitRecord> {
    let mut closest_so_far = INFINITY;
    let mut closest_hit_record: Option<HitRecord> = None;

    for object in objects {
        match object.hit(ray, 0.0, closest_so_far) {
            Some(hit_record) => {
                closest_so_far = hit_record.t;
                closest_hit_record = Some(hit_record);
            },
            None => {}
        }
    }
    
    return closest_hit_record;
}

fn compute_normal_color(hit_record: &HitRecord) -> Color {
    Color {
        r: (hit_record.normal.x + 1.0) * 0.5,
        g: (hit_record.normal.y + 1.0) * 0.5,
        b: (hit_record.normal.z + 1.0) * 0.5
    }
}

fn background_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);

    return linear_blend(t, WHITE, Color::new(0.5, 0.7, 1.0));
}

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0, 1.0);
}