use crate::data::export::{Color, Vector};
use crate::engine::export::{HitRecord, Ray};
use crate::materials::export::Material;
use std::sync::{Arc, Mutex};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
    pub fn share(self) -> Arc<Mutex<dyn Material>> {
        Arc::new(Mutex::new(self))
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vector::reflect(&Vector::unit_vector(&r_in.direction()), &record.normal);
        *scattered = Ray::new(
            record.p,
            reflected + self.fuzz * Vector::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        return Vector::dot(&scattered.direction(), &record.normal) > 0.0;
    }
}
