use crate::data::Color;
use crate::engine::bound_box::AABB;
use crate::engine::Ray;
use crate::materials::{Lambertian, Material};
use nalgebra::Vector3;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Vector3<f64>,
    pub(crate) normal: Vector3<f64>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
    pub(crate) mat_ptr: Arc<dyn Material>,
    pub(crate) u: f64,
    pub(crate) v: f64,
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
            mat_ptr: Arc::new(Lambertian::from_color(Color::new(0.0, 0.0, 0.0))),
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
    fn share(self) -> Arc<dyn Hittable>;
    fn get_bounding_box(&self) -> Option<AABB>;
}
