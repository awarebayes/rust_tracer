use crate::engine::export::{ Ray, HitRecord };
use crate::data::export::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
