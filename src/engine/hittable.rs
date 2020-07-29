use crate::data::Color;
use crate::engine::Ray;
use crate::materials::{Lambertian, Material};
use std::sync::Arc;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Vector3<f64>,
    pub(crate) normal: Vector3<f64>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
    pub(crate) mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = *outward_normal;
        if !self.front_face {
            self.normal = self.normal * -1.0;
        }
    }
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
    fn share(self) -> Arc<dyn Hittable>;
}

