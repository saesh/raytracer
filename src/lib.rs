extern crate rand;
extern crate tobj;

pub mod color;
pub mod io;
pub mod materials;
pub mod objects;
pub mod structures;
pub mod random;
pub mod hitable;

use std::f32::INFINITY;
use std::time::Instant;

use crate::random::random_double;
use crate::structures::camera::Camera;
use crate::hitable::Hitable;
use crate::color::{Color, BLACK, WHITE, linear_blend, gamma_correct, map_color_256};
use crate::structures::ray::Ray;

use indicatif::{ProgressBar, ProgressStyle, HumanDuration};
use rayon::prelude::*;

pub fn render(camera: Camera, world: &Box<dyn Hitable>, image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> Vec<u8> {

    let start = Instant::now();
    let pixel_total = image_width * image_height;
    let mut pixel_processed = 0;

    let pb = ProgressBar::new(pixel_total as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta_precise})")
        .progress_chars("#>-"));

    println!("\nImage size: {} x {}, {} pixels", image_width, image_height, image_width * image_height);
    println!("Samples per pixel: {}", samples_per_pixel);
    println!("Maximum ray bounces: {}", max_depth);
    // println!("Geometries in scene: {}", world.len());
    println!("Shutter speed: {}s\n", camera.time1 - camera.time0);

    let mut image_data: Vec<u8> = Vec::new();

    for pixel_y in (0..image_height).rev() {

        for pixel_x in 0..image_width {

            let pixel_color = (0..samples_per_pixel).into_par_iter().map(|_x| {
                let u: f32 = (pixel_x as f32 + random_double()) / (image_width as f32 - 1.0);
                let v: f32 = (pixel_y as f32 + random_double()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                ray_color(&ray, &world, max_depth)
            })
            .reduce(|| BLACK, |final_color, next_color| final_color + next_color);

            let final_color = 
                map_color_256(
                    gamma_correct(
                        average_samples(pixel_color, samples_per_pixel)));
            image_data.push(final_color.0);
            image_data.push(final_color.1);
            image_data.push(final_color.2);

            pixel_processed = pixel_processed + 1;
            pb.set_position(pixel_processed);
        }
    }

    pb.finish();
    println!("");
    println!("Finished rendering in {}", HumanDuration(start.elapsed()));

    image_data
}

fn ray_color(ray: &Ray, world: &Box<dyn Hitable>, depth: u32) -> Color {
    if depth <= 0 {
        return BLACK
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(hit_record) => {
            return match hit_record.material.scatter(ray, &hit_record) {
                Some((color, new_ray)) => color * ray_color(&new_ray, world, depth - 1),
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

fn average_samples(color: Color, samples_per_pixel: u32) -> Color {
    let scale = 1.0 / samples_per_pixel as f32;

    return color * scale;
}