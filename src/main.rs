mod data;
mod engine;
mod gui;
mod materials;
mod textures;

// extern crates
extern crate nalgebra;
use nalgebra::Vector3;

// crate imports
use crate::data::{scenes, Color};
use crate::engine::{render, Camera, Scene};
// use crate::gui::prerender;

// std imports
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn main() {
    let image_width = 700.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let look_from = Vector3::new(26.0, 3.0, 6.0);
    let look_at = Vector3::new(0.0, 2.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

    let cam = Camera::new(look_from, look_at, vup, 20.0, aspect_ratio, 0.0, focus_dist);
    let cam = Arc::new(cam);

    let world = scenes::simple_light();
    let world = Arc::new(world);

    let completed = Arc::new(AtomicBool::new(false));
    let prerender_finished = Arc::new(AtomicBool::new(false));

    let scene = Scene {
        cam,
        world,
        samples_per_pixel,
        max_depth,
        image_width,
        image_height,
        completed,
        prerender_finished,
        background,
    };
    // TODO: prerender with piston
    //prerender(scene.clone());
    //writeln!("Press enter to render...");
    //std::io::stdin();
    scene
        .prerender_finished
        .store(true, std::sync::atomic::Ordering::Relaxed);
    render(scene, 4, "result.png".to_string())
}
