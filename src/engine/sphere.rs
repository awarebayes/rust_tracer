use crate::data::export::Vector;
use crate::engine::hittable::Hittable;
use crate::engine::export::{Ray, HitRecord};
use crate::materials::export::Material;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Sphere {
    center: Vector,
    radius: f64,
    mat_ptr: Rc<RefCell<dyn Material>>
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, mat_ptr: Rc<RefCell<dyn Material>>) -> Sphere {
        Sphere{ center, radius, mat_ptr }
    }

    pub fn share(self) -> Rc<RefCell<Sphere>>{
        Rc::new(RefCell::new(self))
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().len_squared();
        let b_2 = Vector::dot(&oc, &r.direction());
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = b_2*b_2 - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-b_2 - root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(r, &outward_normal);
                record.mat_ptr = Rc::clone( &self.mat_ptr);
                return true
            }
            let temp = (-b_2 + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(r, &outward_normal);
                record.mat_ptr = Rc::clone( &self.mat_ptr);
                return true
            }
        }
        return false
    }
}