use crate::data::export::{Vector, Color};
use crate::engine::export::Ray;
use crate::materials::export::{ Lambertian, Material };
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Vector,
    pub(crate) normal: Vector,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
    pub(crate) mat_ptr: Rc<RefCell<dyn Material>>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector) {
        self.front_face = Vector::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = *outward_normal;
        if !self.front_face {
            self.normal = self.normal * -1.0;
        }
    }
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vector::new(0.0,0.0,0.0),
            normal: Vector::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: Rc::new(RefCell::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))))
        }
    }
}



pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

