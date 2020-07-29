use crate::data::{Color, vrandom_in_unit_sphere, vunit, reflect};
use crate::engine::{HitRecord, Ray};
use crate::materials::Material;
use std::sync::Arc;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
    pub fn share(self) -> Arc<dyn Material> {
        Arc::new(self)
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
        let reflected = reflect(&vunit(&r_in.direction()), &record.normal);
        *scattered = Ray::new(
            record.p,
            reflected + self.fuzz * vrandom_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        return scattered.direction().dot(&record.normal) > 0.0;
    }
}
