use crate::data::Color;
use crate::textures::Texture;
use nalgebra::Vector3;
use noise::{NoiseFn, Perlin, Turbulence};
use std::sync::Arc;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Color {
        return self.color_value;
    }
    fn share(self) -> Arc<dyn Texture> {
        Arc::new(self)
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Color, odd: Color) -> CheckerTexture {
        CheckerTexture {
            even: SolidColor::new(even).share(),
            odd: SolidColor::new(odd).share(),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Color {
        let sines = (p[0] * 10.0).sin() * (p[1] * 10.0).sin() * (p[2] * 10.0).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }

    fn share(self) -> Arc<dyn Texture> {
        Arc::new(self)
    }
}

pub struct NoiseTexture {
    noise: Turbulence<Perlin>,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Turbulence::new(Perlin::new()),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3<f64>) -> Color {
        let ps = p * self.scale;
        let mut noise_val = self.noise.get([ps[0], ps[1], ps[2]]);
        noise_val = (noise_val + 1.0) / 2.0; // [-1, 1] -> [0, 1]
        noise_val =
            0.5 * (1.0 + (self.scale * p[2] + 5.0 * noise_val + (1.0 - p[1]) * self.scale).sin());
        return Color::new(1.0, 1.0, 1.0) * noise_val;
    }
    fn share(self) -> Arc<dyn Texture> {
        Arc::new(self)
    }
}
