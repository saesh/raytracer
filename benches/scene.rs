use criterion::{criterion_group, BatchSize, Criterion};
use std::sync::Arc;

use raytracer::structures::camera::Camera;
use raytracer::structures::vec3::Vec3;
use raytracer::materials::Lambertian;
use raytracer::materials::DiffuseLight;
use raytracer::texture::SolidColor;
use raytracer::color::Color;
use raytracer::objects::rect::*;
use raytracer::hitable::{Hitable, HitableList};
use raytracer::render;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scene/cornell/10x10x4", |b| {
        // image
        const ASPECT_RATIO: f32 = 1.0;
        let image_width = 10;
        let image_height = (image_width as f32 / ASPECT_RATIO) as u32;
        let samples_per_pixel = 4;
        let max_depth = 10;

        // camera
        let lookfrom = Vec3::new(1.0, 1.0, -2.8);
        let lookat = Vec3::new(1.0, 1.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.0;

        let camera: Camera = Camera::new(
            lookfrom, 
            lookat, 
            vup, 
            40.0, 
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
            0.0,
            1.0);

        // box
        let red   = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05))));
        let white = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73))));
        let green = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15))));
        let light = Arc::new(DiffuseLight::new(SolidColor::new(Color::new(2.,   2.,   2.))));

        let mut hitable_list = HitableList::default();

        hitable_list.push(YzRect::new(  0.,   2.,   0.,   2.,   0., green)); 
        hitable_list.push(YzRect::new(  0.,   2.,   0.,   2.,   2.,   red));
        hitable_list.push(XzRect::new(0.85, 1.15, 0.85, 1.15, 1.99, light.clone()));
        hitable_list.push(XzRect::new(  0.,   2.,   0.,   2.,   0., white.clone()));
        hitable_list.push(XzRect::new(  0.,   2.,   0.,   2.,   2., white.clone()));
        hitable_list.push(XyRect::new(  0.,   2.,   0.,   2.,   2., white.clone()));

        let world: Box<dyn Hitable> = Box::new(hitable_list);
        
        b.iter_batched(
            | | (),
            |_| render(&camera, &world, image_width, image_height, samples_per_pixel, max_depth),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);