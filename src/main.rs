mod data;
mod engine;
mod materials;

// extern crates
extern crate image;
use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::Open01;
use rand::{thread_rng, Rng};

// crate imports
use crate::data::export::{Color, Vector};
use crate::engine::export::{Camera, HitRecord, HittableList, Ray, Sphere};
use crate::materials::export::{Dielectric, Lambertian, Metal};

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
            .borrow()
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

fn main() {
    let image_width = 480.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 100;
    let max_depth = 50;
    
    let look_from = Vector::new(3.0, 3.0, 2.0);
    let look_at = Vector::new(0.0, 0.0, -1.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let focus_dist = (look_from - look_at).len();


    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        image_width / image_height,
        2.0,
        focus_dist
    );

    let mut world = HittableList::new();
    world.add(
        Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::new(0.1, 0.2, 0.5)).share(),
        )
        .share(),
    );

    world.add(
        Sphere::new(
            Vector::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::new(0.8, 0.8, 0.8)).share(),
        )
        .share(),
    );

    world.add(
        Sphere::new(
            Vector::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Color::new(0.99, 0.99, 0.99), 0.3).share(),
        )
        .share(),
    );

    world.add(
        Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(1.5).share(),
        )
        .share(),
    );
    world.add(
        Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            -0.45,
            Dielectric::new(1.5).share(),
        )
        .share(),
    );
    let mut imgbuf = image::ImageBuffer::new(image_width as u32, image_height as u32);
    let pb = ProgressBar::new((image_width * image_height) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed}] [{wide_bar}] {pos}/{len} ({eta})"),
    );
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        pb.inc(1);
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let xf = x as f64;
            let yf = y as f64;
            let xr: f64 = thread_rng().sample(Open01);
            let yr: f64 = thread_rng().sample(Open01);
            let u = (xf + xr) / (image_width - 1.0);
            let v = (yf + yr) / (image_height - 1.0);
            let v = 1.0 - v;
            let r = cam.get_ray(u, v);
            let rc = ray_color(&r, &world, max_depth);
            pixel_color = pixel_color + rc;
        }
        *pixel = pixel_color.normalize_samples(samples_per_pixel).to_rgb();
    }
    pb.finish_with_message("Your image is complete, sir! Enjoy!");

    imgbuf.save("image.png").unwrap();
}
