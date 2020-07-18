use crate::data::export::Color;
use crate::engine::export::{HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
