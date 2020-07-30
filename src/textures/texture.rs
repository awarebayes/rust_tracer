use nalgebra::Vector3;
use crate::data::Color;
use std::sync::Arc;


pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Color;
    fn share(self) -> Arc<dyn Texture>;
}

