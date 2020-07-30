use crate::data::{Color, vunit, vrandom};
use crate::engine::{HitRecord, Ray};
use crate::materials::Material;
use crate::textures::{Texture, SolidColor};
use std::sync::Arc;


pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: Color) -> Lambertian {
        Lambertian { albedo: SolidColor::new(albedo).share() }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: texture }
    }

    pub fn share(self) -> Arc<dyn Material> {
        Arc::new(self)
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + vunit(&vrandom());
        *scattered = *&mut Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo.value(record.u, record.v, &record.p);
        return true;
    }
}
