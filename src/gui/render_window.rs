extern crate image as im;
extern crate piston_window;

use crate::engine::Scene;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use piston_window::*;

pub fn render_window(
    canvas: Arc<Mutex<im::ImageBuffer<im::Rgba<u8>, Vec<u8>>>>,
    scene: Arc<Scene>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let opengl = OpenGL::V3_2;
        let (width, height) = (scene.image_width as u32, scene.image_height as u32);
        let mut window: PistonWindow = WindowSettings::new("Ray Tracer", (width, height))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };
        let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas.lock().unwrap().clone(),
            &TextureSettings::new(),
        )
        .unwrap();

        let update_every_ms = 100;
        let mut last_updated = Instant::now();

        while let Some(e) = window.next() {
            if let Some(_) = e.render_args() {
                if last_updated.elapsed().as_millis() > update_every_ms {
                    texture
                        .update(&mut texture_context, &canvas.lock().unwrap().clone())
                        .unwrap();
                    last_updated = Instant::now();
                }
                window.draw_2d(&e, |c, g, device| {
                    // Update texture before rendering.
                    texture_context.encoder.flush(device);

                    clear([1.0; 4], g);
                    image(&texture, c.transform, g);
                });
            }
            if window.should_close() {
                window.hide();
                return;
            }

            if scene.completed.load(Relaxed) {
                return;
            }
        }
    })
}
