use crate::data::Color;
use crate::engine::{HitRecord, Ray};
use nalgebra::Vector3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emit(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}
