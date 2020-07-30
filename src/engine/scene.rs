// extern crates
extern crate image;
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::Vector3;

// crate imports
use crate::data::Color;
use crate::data::{rand_float, rand_float01, vunit, vlen, vrandom_range};
use crate::engine::{Camera, HitRecord, HittableList, Ray, Sphere, Hittable};
use crate::gui::render_window;
use crate::materials::{Dielectric, Lambertian, Metal};


// std imports
use std::f64::INFINITY;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct SimplePixel {
    pub color: image::Rgba<u8>,
    pub x: u64,
    pub y: u64,
}

pub struct Scene {
    pub cam: Arc<Camera>,
    pub world: Arc<HittableList>,
    pub samples_per_pixel: u64,
    pub max_depth: i32,
    pub image_width: f64,
    pub image_height: f64,
    pub prerender_finished: Arc<AtomicBool>,
    pub completed: Arc<AtomicBool>,
}

fn ray_color(r: &Ray, world: Arc<HittableList>, depth: i32) -> Color {
    let mut record = HitRecord::new();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut record) {
        let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        if record
            .mat_ptr
            .scatter(r, &record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
        // let target = record.p + record.normal + Vector::random_in_hemisphere(&record.normal);
        // return 0.5 * ray_color(&Ray::new(record.p, target-record.p), world, depth-1)
    }
    let unit_direction = vunit(&r.direction());
    let t = 0.5 * (unit_direction[1] + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

pub fn random_world() -> HittableList {
    let mut world = HittableList::new();
    let ground_matrial = Lambertian::new(Color::new(0.5, 0.5, 0.5)).share();
    world.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, ground_matrial).share());


    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_material = rand_float01();
            let center = Vector3::new(a + 0.9 * rand_float01(), 0.2, b + 0.9 * rand_float01());

            if vlen(&(center - Vector3::new(4.0, 0.2, 0.0))) <= 0.9 {
                continue;
            }
            if choose_material < 0.8 {
                let albedo = Color::random() * Color::random();
                let sphere_material = Lambertian::new(albedo).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share())
            } else if choose_material < 0.95 {
                let albedo = Color::from_vector(&vrandom_range(0.5, 1.0));
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
    world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1).share());

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1)).share();
    world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2).share());

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0).share();
    world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3).share());

    return world;
}

fn pixel_processor(x: f64, y: f64, scene: Arc<Scene>) -> image::Rgba<u8> {
    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    for _ in 0..scene.samples_per_pixel {
        let u = (x + rand_float01()) / (scene.image_width - 1.0);
        let v = (y + rand_float01()) / (scene.image_height - 1.0);
        let v = 1.0 - v;
        let r = scene.cam.get_ray(u, v);
        let rc = ray_color(&r, scene.world.clone(), scene.max_depth);
        pixel_color = pixel_color + rc;
    }
    pixel_color
        .normalize_samples(scene.samples_per_pixel)
        .to_rgba()
}

#[derive(Clone, Copy)]
pub struct ProducerRange {
    pub from_x: u64,
    pub to_x: u64,
    pub from_y: u64,
    pub to_y: u64,
}

struct Pair {
    pub x: u64,
    pub y: u64,
}

fn send(
    x: u64,
    y: u64,
    scene: Arc<Scene>,
    tx: Arc<Mutex<mpsc::Sender<SimplePixel>>>,
    range: ProducerRange,
    success_table: &mut Vec<Vec<bool>>,
) {
    let color = pixel_processor(x as f64, y as f64, scene.clone());
    let pixel = SimplePixel { x, y, color };
    match tx.lock() {
        Ok(sender) => match sender.send(pixel) {
            Ok(_) => {
                success_table[(x - range.from_x) as usize][(y - range.from_y) as usize] = true;
            }
            Err(_) => println!("unable to send!"),
        },
        Err(_) => println!("unable to obtain lock "),
    }
}

fn producer(
    tx: Arc<Mutex<mpsc::Sender<SimplePixel>>>,
    scene: Arc<Scene>,
    range: ProducerRange,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut success_table = vec![
            vec![false; (range.to_y - range.from_y) as usize];
            (range.to_x - range.from_x) as usize
        ];
        for x in range.from_x..range.to_x {
            for y in range.from_y..range.to_y {
                send(x, y, scene.clone(), tx.clone(), range, &mut success_table);
            }
        }

        let mut failed = Vec::new();

        for x in 0..(range.to_x - range.from_x) {
            for y in 0..(range.to_y - range.from_y) {
                if !success_table[(x) as usize][(y) as usize] {
                    failed.push(Pair {
                        x: range.from_x + x,
                        y: range.from_y + y,
                    });
                }
            }
        }

        if !failed.is_empty() {
            println!("Producer failed: {}", failed.len());
        }

        while !failed.is_empty() {
            let p = failed.last().unwrap();
            send(
                p.x,
                p.y,
                scene.clone(),
                tx.clone(),
                range,
                &mut success_table,
            );
        }
    })
}

fn consumer(
    rx: mpsc::Receiver<SimplePixel>,
    buffer: Arc<Mutex<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>>,
    progress: Arc<ProgressBar>,
    success_table: Arc<Mutex<Vec<Vec<bool>>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        match rx.recv_timeout(Duration::from_secs(4)) {
            Ok(p) => match buffer.lock() {
                Ok(mut lock) => {
                    lock.put_pixel(p.x as u32, p.y as u32, p.color);
                    progress.inc(1);
                    success_table.lock().unwrap()[p.x as usize][p.y as usize] = true;
                }
                Err(_) => println!("Encountered error in consumer"),
            },
            Err(_) => {
                println!("Timeout Reached");
                return;
            }
        }
    })
}

fn split_even(from: u64, to: u64, n: u64) -> Vec<(u64, u64)> {
    let split_size = (to - from) / n + 1;
    let mut current_from = from;
    let mut current_to = split_size;
    let mut result = Vec::new();
    for _ in 0..n {
        result.push((current_from, current_to));
        current_from = current_to;
        current_to += split_size;
        if current_to > to {
            current_to = to
        };
    }
    return result;
}

pub fn render(scene: Scene, n_workers: u64, path: String) {
    let imgbuf: Arc<Mutex<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>> = Arc::new(Mutex::new(
        image::ImageBuffer::new(scene.image_width as u32, scene.image_height as u32),
    ));

    let pb = ProgressBar::new((scene.image_width * scene.image_height) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed}] [{wide_bar}] {pos}/{len} ({eta})"),
    );
    let pb = Arc::new(pb);

    let success_table = Arc::new(Mutex::new(vec![
        vec![false; (scene.image_height) as usize];
        (scene.image_width) as usize
    ]));

    let (tx, rx) = mpsc::channel();
    let shared_tx = Arc::new(Mutex::new(tx));
    let split = split_even(0, scene.image_width as u64 - 1, n_workers);

    let scene = Arc::new(scene); // Make scene shared across threads. (Immutable)

    let render_handle = render_window(imgbuf.clone(), scene.clone());

    for (from_x, to_x) in split.iter() {
        let range = ProducerRange {
            from_x: *from_x,
            to_x: *to_x,
            from_y: 0,
            to_y: scene.image_height as u64,
        };
        producer(shared_tx.clone(), scene.clone(), range);
    }

    match consumer(rx, imgbuf.clone(), pb.clone(), success_table.clone()).join() {
        Ok(_) => println!("Consumer jobs finished"),
        Err(_) => println!("Error joining consumer..."),
    };

    let mut failed = Vec::new();
    for x in 0..(scene.image_width as u64) {
        for y in 0..(scene.image_height as u64) {
            if !success_table.lock().unwrap()[x as usize][y as usize] {
                failed.push(Pair { x, y });
            }
        }
    }
    if !failed.is_empty() {
        println!("Observer failed: {}", failed.len());
    }

    while !failed.is_empty() {
        let p = failed.last().unwrap();
        let color = pixel_processor(p.x as f64, p.y as f64, scene.clone());
        imgbuf
            .lock()
            .unwrap()
            .put_pixel(p.x as u32, p.y as u32, color);
        failed.pop();
    }

    pb.clone()
        .finish_with_message("Your image is complete, sir! Enjoy!");

    println!("saving...");
    imgbuf.lock().unwrap().save(path).unwrap();

    scene.completed.store(true, Relaxed);
    
    render_handle.join().unwrap();
}
