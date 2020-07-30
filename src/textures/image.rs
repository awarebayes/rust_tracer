use crate::textures::Texture;

use image;
use nalgebra::Vector3;
use std::path::Path;
use std::sync::Arc;

pub struct ImageTexture {
    width: u32,
    height: u32,
    image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl ImageTexture {
    pub fn new(path: String) -> ImageTexture {
        let img = image::open(Path::new(&path)).unwrap();
        let img = img.into_rgba();
        ImageTexture {
            width: img.width(),
            height: img.height(),
            image: img.clone(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: &Vector3<f64>) -> crate::data::Color {
        u = u.min(1.0).max(0.0);
        v = 1.0 - v.min(1.0).max(0.0);
        let mut i = (u * self.width as f64).round() as u32;
        let mut j = (v * self.height as f64).round() as u32;
        if i >= self.width {
            i -= 1
        }
        if j >= self.height {
            j -= 1
        }

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);
        return crate::data::Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
            * color_scale;
    }
    fn share(self) -> Arc<dyn Texture> {
        Arc::new(self)
    }
}
