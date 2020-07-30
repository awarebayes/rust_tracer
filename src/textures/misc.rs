use crate::data::Color;
use crate::textures::Texture;
use nalgebra::Vector3;
use std::sync::Arc;
use noise::{Perlin, NoiseFn};

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
    noise: Perlin,
}

impl NoiseTexture{
    pub fn new() ->  NoiseTexture {
        NoiseTexture{noise: Perlin::new()}
    }
}


impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3<f64>) -> Color {
        let noise_val = self.noise.get([p[0], p[1], p[2]]).abs();
        return Color::new(1.0, 1.0, 1.0) * noise_val;
    }
    fn share(self) -> Arc<dyn Texture> {
        Arc::new(self)
    }
}