mod io;
mod structures;

use std::time::{Instant};

use io::ppm;
use structures::color::*;
use structures::geometry::Sphere;
use structures::hittable::Hittable;
use structures::ray::Ray;
use structures::vec3::Vec3;

fn main() {
    let start = Instant::now();

    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width: i32= 400;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;

    // camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Vec3::ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // objects
    // spheres are mirrored, is my coordinate system off?
    let sphere_left = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(sphere_left));

    // render
    ppm::write_header(image_width, image_height);

    for pixel_y in (0..image_height).rev() {

        eprint!("\rScanlines remaining: {} ", pixel_y);

        for pixel_x in 0..image_width {

            let u: f32 = pixel_x as f32 / (image_width as f32 - 1.0);
            let v: f32 = pixel_y as f32 / (image_height as f32 - 1.0);
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            
            let color_at_point = ray_color(&ray, &objects);

            ppm::write_pixel(color_at_point);
        }
    }

    let duration = start.elapsed();
    eprintln!("\nDone in {:?}.", duration);
}

fn ray_color(ray: &Ray, objects: &Vec<Box<dyn Hittable>>) -> Color {

    // TODO: can this be done without a mutable variable
    let mut color = Color::new(0.0, 0.0, 0.0);

    for object in objects {
        color = match object.hit(ray, 1.0, 1.0) {
            // TODO: remove static sphere center -> breaks on two spheres
            Some(hit_record) => compute_normal_color(hit_record.t, ray, Vec3::new(0.0, 0.0, -1.0)),
            None => background_color(&ray),
        };
    }

    return color;
}

fn compute_normal_color(t: f32, ray: &Ray, object_center: Vec3) -> Color {
    let n = (ray.at(t) - object_center).normalize();

    return Color {
        r: (n.x + 1.0) * 0.5,
        g: (n.y + 1.0) * 0.5,
        b: (n.z + 1.0) * 0.5
    }
}

fn background_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);

    return linear_blend(t, WHITE, Color::new(0.5, 0.7, 1.0));
}
