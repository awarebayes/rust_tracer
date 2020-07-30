use crate::materials::Material;
use crate::textures::{ Texture, SolidColor };
use crate::data::Color;
use std::sync::Arc;


pub struct DiffuseLight {
    emitter: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(c: Color) -> DiffuseLight{
        DiffuseLight {emitter: SolidColor::new(c).share() }
    }

    pub fn from_texture(t: Arc<dyn Texture>) -> DiffuseLight{
        DiffuseLight { emitter:t }
    }
    pub fn share(self) -> Arc<dyn Material> {
        Arc::new(self)
    }
}


impl Material for DiffuseLight {
    fn emit(&self, u: f64, v: f64, p: &nalgebra::Vector3<f64>) -> crate::data::Color {
        return self.emitter.value(u, v, p);
    }
    fn scatter(
        &self,
        r_in: &crate::engine::Ray,
        record: &crate::engine::HitRecord,
        attenuation: &mut crate::data::Color,
        scattered: &mut crate::engine::Ray,
    ) -> bool {
        return false;
    }
}
