extern crate rand;
extern crate tobj;

pub mod color;
pub mod io;
pub mod materials;
pub mod objects;
pub mod structures;

use std::f32::{INFINITY};
use std::time::{Instant};

use crate::io::random;
use crate::io::ppm;
use crate::structures::camera::Camera;
use crate::objects::Hitable;
use crate::color::{Color, BLACK, WHITE, linear_blend, gamma_correct, map_color_256};
use crate::structures::ray::Ray;

use rayon::prelude::*;

pub fn run(camera: Camera, objects: &mut Vec<Box<dyn Hitable>>, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32) {
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

            let pixel_color = (0..samples_per_pixel).into_par_iter().map(|_x| {
                let u: f32 = (pixel_x as f32 + random::random_double()) / (image_width as f32 - 1.0);
                let v: f32 = (pixel_y as f32 + random::random_double()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                ray_color(&ray, &objects, max_depth)
            })
            .reduce(|| BLACK, |final_color, next_color| final_color + next_color);

            let final_color = map_color_256(gamma_correct(sample(pixel_color, samples_per_pixel)));
            ppm::write_pixel(final_color.0, final_color.1, final_color.2);
        }
    }

    eprintln!("\nDone in {:?}.", start.elapsed());
}

fn ray_color(ray: &Ray, objects: &Vec<Box<dyn Hitable>>, depth: i32) -> Color {
    if depth <= 0 {
        return BLACK
    }

    match objects.hit(ray, 0.001, INFINITY) {
        Some(hit_record) => {
            return match hit_record.material.scatter(ray, &hit_record) {
                Some((color, new_ray)) => color * ray_color(&new_ray, objects, depth - 1),
                None => BLACK
            }
        },
        None => background_color(&ray),
    }
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