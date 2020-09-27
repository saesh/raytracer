mod io;
mod geometry;
mod structures;

use std::time::{Instant};
use std::f32::{INFINITY};

use io::ppm;
use io::random;
use io::obj::load_teapot;
use geometry::sphere::Sphere;
use structures::camera::Camera;
use structures::color::*;
use structures::hittable::{Hittable, HitRecord};
use structures::ray::Ray;
use structures::vec3::*;
use structures::material::{Lambertian, Metal, Dielectric};

fn main() {
    let start = Instant::now();

    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    let image_width: i32 = 500;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 300;
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
        dist_to_focus);

    // world
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    match load_teapot() {
        Some(triangles) => {
            for triangle in triangles {
                objects.push(Box::new(triangle));
            }
        },
        None => {}
    };

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground))));

    // for a in -11..11 {
        
    //     for b in -11..11 {
        
    //         let choose_mat = random::random_double();
    //         let center = Vec3::new(a as f32 + 0.9 * random::random_double(), 0.2, b as f32 + 0.9 * random::random_double());
            
    //         if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

    //             if choose_mat < 0.8 {
    //                 // diffuse
    //                 let albedo = Color::random() * Color::random();
    //                 let sphere_material = Lambertian::new(albedo);
    //                 objects.push(Box::new(Sphere::new(center, 0.2, Box::new(sphere_material))));
    //             } else if choose_mat < 0.95 {
    //                 // metal
    //                 let albedo = Color::random_bounded(0.5, 1.0);
    //                 let fuzz = random::random_double_bounded(0.0, 0.5);
    //                 let sphere_material = Metal::new(albedo, fuzz);
    //                 objects.push(Box::new(Sphere::new(center, 0.2, Box::new(sphere_material))));
    //             } else {
    //                 // glass
    //                 let sphere_material = Dielectric::new(1.5);
    //                 objects.push(Box::new(Sphere::new(center, 0.2, Box::new(sphere_material))));
    //             }
    //         }
    //     }
    // }
    // let material1 = Dielectric::new(1.5);
    // objects.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(material1))));

    // let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    // objects.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(material2))));

    // let material3  = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    // objects.push(Box::new(Sphere::new(Vec3::new(4.0,1.0, 0.0), 1.0, Box::new(material3))));

    // render
    eprintln!("Image size: {} x {}, {} pixels", image_width, image_height, image_width * image_height);
    eprintln!("Samples per pixel: {}", samples_per_pixel);
    eprintln!("Maximum ray bounces: {}", max_depth);
    eprintln!("Geometries in scene: {}", objects.len());
    ppm::write_header(image_width, image_height);

    for pixel_y in (0..image_height).rev() {

        eprint!("\rScanlines remaining: {} (elapsed: {:?})", pixel_y, start.elapsed());

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