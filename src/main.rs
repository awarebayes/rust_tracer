mod data;
mod engine;
mod gui;
mod materials;

// extern crates
extern crate nalgebra;
use nalgebra::Vector3;

// crate imports
use crate::engine::{random_world, render, Camera, Scene};
use crate::gui::prerender;


// std imports
use std::sync::Arc; 
use std::sync::atomic::AtomicBool;

fn main() {
    let image_width = 720.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 10;
    let max_depth = 50;
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;

    let cam = Camera::new(look_from, look_at, vup, 20.0, aspect_ratio, 0.1, focus_dist);
    let cam = Arc::new(cam);

    let world = random_world();
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
    };
    
    // TODO: prerender with piston
    //prerender(scene.clone());
    //writeln!("Press enter to render...");
    //std::io::stdin();
    scene.prerender_finished.store(true, std::sync::atomic::Ordering::Relaxed);
    
    render(scene, 4, "test.png".to_string())
}
