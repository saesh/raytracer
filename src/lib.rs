extern crate dyn_clone;
extern crate rand;
extern crate tobj;

pub mod io;
pub mod materials;
pub mod objects;
pub mod structures;

use std::f32::{INFINITY};
use std::time::{Instant};

use crate::io::random;
use crate::io::ppm;
use crate::structures::camera::Camera;
use crate::structures::hittable::{Hittable, HitRecord};
use crate::structures::color::{Color, BLACK, WHITE, linear_blend};
use crate::structures::ray::Ray;

pub fn run(camera: Camera, objects: Vec<Box<dyn Hittable>>, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32) {
    let start = Instant::now();

    eprintln!("Image size: {} x {}, {} pixels", image_width, image_height, image_width * image_height);
    eprintln!("Samples per pixel: {}", samples_per_pixel);
    eprintln!("Maximum ray bounces: {}", max_depth);
    eprintln!("Geometries in scene: {}", objects.len());
    eprintln!("Shutter speed: {}s", camera.time1 - camera.time0);

    ppm::write_header(image_width, image_height);

    for pixel_y in (0..image_height).rev() {

        eprint!("\rScanlines remaining: {}/{} (elapsed: {:?})", pixel_y, image_height, start.elapsed());

        for pixel_x in 0..image_width {

            let mut pixel_color = BLACK;

            for _s in 0..samples_per_pixel {
                let u: f32 = (pixel_x as f32 + random::random_double()) / (image_width as f32 - 1.0);
                let v: f32 = (pixel_y as f32 + random::random_double()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &objects, max_depth);
            }

            let final_color = map_color_256(gamma_correct(sample(pixel_color, samples_per_pixel)));
            ppm::write_pixel(final_color.0, final_color.1, final_color.2);
        }
    }

    eprintln!("\nDone in {:?}.", start.elapsed());
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

fn sample(color: Color, samples_per_pixel: i32) -> Color {
    let scale = 1.0 / samples_per_pixel as f32;

    return color * scale;
}

fn gamma_correct(color: Color) -> Color {
    Color {
        r: color.r.sqrt(),
        g: color.g.sqrt(),
        b: color.b.sqrt(),
    }
}
fn map_color_256(color: Color) -> (i32, i32, i32) {
    (
        (256.0 * clamp(color.r, 0.0, 0.999)) as i32,
        (256.0 * clamp(color.g, 0.0, 0.999)) as i32, 
        (256.0 * clamp(color.b, 0.0, 0.999)) as i32,
    )
}

#[inline]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    return if x < min { 
        min 
    } else if x > max {
        max
    } else {
        x
    };
}