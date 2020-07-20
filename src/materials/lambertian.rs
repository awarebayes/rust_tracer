use crate::data::{Color, vunit, vrandom};
use crate::engine::{HitRecord, Ray};
use crate::materials::Material;
use std::sync::{Arc, Mutex};


pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
    pub fn share(self) -> Arc<Mutex<dyn Material>> {
        Arc::new(Mutex::new(self))
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
        *attenuation = self.albedo;
        return true;
    }
}
