use crate::data::export::{Color, Vector};
use crate::engine::export::{HitRecord, Ray};
use crate::materials::export::Material;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
    pub fn share(self) -> Rc<RefCell<dyn Material>> {
        Rc::new(RefCell::new(self))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + Vector::unit_vector(&Vector::random());
        *scattered = *&mut Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
