mod data;
mod engine;
mod gui;
mod materials;

// extern crates
extern crate image;

// crate imports
use crate::data::export::Vector;
use crate::engine::export::{random_world, render, Camera, Scene};
use std::sync::atomic::AtomicBool;

// std imports
use std::sync::Arc;

fn main() {
    let image_width = 720.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 1;
    let max_depth = 50;
    let look_from = Vector::new(13.0, 2.0, 3.0);
    let look_at = Vector::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;

    let cam = Camera::new(look_from, look_at, vup, 20.0, aspect_ratio, 0.1, focus_dist);
    let cam = Arc::new(cam);

    let world = random_world();
    let world = Arc::new(world);

    let completed = Arc::new(AtomicBool::new(false));

    let scene = Scene {
        cam,
        world,
        samples_per_pixel,
        max_depth,
        image_width,
        image_height,
        completed,
    };
    let scene = Arc::new(scene);
    render(scene, 12, "test.png".to_string())
}
