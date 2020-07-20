use crate::data::Color;
use crate::engine::{HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
