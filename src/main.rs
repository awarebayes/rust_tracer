mod data;
mod engine;
mod materials;

// extern crates
extern crate image;
use indicatif::{ProgressBar, ProgressStyle};

// crate imports
use crate::data::export::{Color, Vector};
use crate::data::vector::{rand_float, rand_float01};
use crate::engine::export::{Camera, HitRecord, HittableList, Ray, Sphere};
use crate::materials::export::{Dielectric, Lambertian, Metal};
use std::sync::{mpsc, Arc, Mutex};

// std imports
use std::f64::INFINITY;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut record = HitRecord::new();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut record) {
        let mut scattered = Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        if record
            .mat_ptr
            .lock()
            .unwrap()
            .scatter(r, &record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
        // let target = record.p + record.normal + Vector::random_in_hemisphere(&record.normal);
        // return 0.5 * ray_color(&Ray::new(record.p, target-record.p), world, depth-1)
    }
    let unit_direction = Vector::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_matrial = Lambertian::new(Color::new(0.5, 0.5, 0.5)).share();
    world.add(Sphere::new(Vector::new(0.0, -1000.0, 0.0), 1000.0, ground_matrial).share());

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_material = rand_float01();
            let center = Vector::new(a + 0.9 * rand_float01(), 0.2, b + 0.9 * rand_float01());

            if (center - Vector::new(4.0, 0.2, 0.0)).len() <= 0.9 {
                continue;
            }
            if choose_material < 0.8 {
                let albedo = Color::random() * Color::random();
                let sphere_material = Lambertian::new(albedo).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share())
            } else if choose_material < 0.95 {
                let albedo = Color::from_vector(&Vector::random_range(0.5, 1.0));
                let fuzz = rand_float(0.0, 0.5);
                let sphere_material = Metal::new(albedo, fuzz).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share())
            } else {
                let sphere_material = Dielectric::new(1.5).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share());
            }
        }
    }

    let material1 = Dielectric::new(1.5).share();
    world.add(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, material1).share());

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1)).share();
    world.add(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, material2).share());

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0).share();
    world.add(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, material3).share());

    return world;
}

fn process_pixel(
    x: f64,
    y: f64,
    cam: &Camera,
    world: &HittableList,
    samples_per_pixel: u64,
    max_depth: i32,
    image_width: f64,
    image_height: f64,
) -> image::Rgb<u8> {
    
    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    for _ in 0..samples_per_pixel {
        let u = (x + rand_float01()) / (image_width - 1.0);
        let v = (y + rand_float01()) / (image_height - 1.0);
        let v = 1.0 - v;
        let r = cam.get_ray(u, v);
        let rc = ray_color(&r, &world, max_depth);
        pixel_color = pixel_color + rc;
    }
    return pixel_color.normalize_samples(samples_per_pixel).to_rgb();
}

fn main() {
    let image_width = 1080.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let look_from = Vector::new(13.0, 2.0, 3.0);
    let look_at = Vector::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;

    let cam = Camera::new(look_from, look_at, vup, 20.0, aspect_ratio, 0.1, focus_dist);

    let world = random_scene();
    let mut imgbuf = image::ImageBuffer::new(image_width as u32, image_height as u32);

    let pb = ProgressBar::new((image_width * image_height) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed}] [{wide_bar}] {pos}/{len} ({eta})"),
    );

    //crossbeam::scope(|scope| {

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        pb.inc(1);
        *pixel = process_pixel(
            x as f64,
            y as f64,
            &cam,
            &world,
            samples_per_pixel,
            max_depth,
            image_width,
            image_height,
        );
    }
    //}).expect("Shit happened");

    pb.finish_with_message("Your image is complete, sir! Enjoy!");

    imgbuf.save("image_multithreaded.png").unwrap();
}
