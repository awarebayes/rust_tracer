use nalgebra::Vector3;

use crate::engine::{HitRecord, Ray};
use crate::engine::hittable::Hittable;
use crate::materials::Material;
use std::sync::{Arc, Mutex};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    mat_ptr: Arc<Mutex<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, mat_ptr: Arc<Mutex<dyn Material>>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }

    pub fn share(self) -> Arc<Mutex<Sphere>> {
        Arc::new(Mutex::new(self))
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction();
        let a = a.dot(&a);
        let b_2 = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b_2 * b_2 - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-b_2 - root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(r, &outward_normal);
                record.mat_ptr = Arc::clone(&self.mat_ptr);
                return true;
            }
            let temp = (-b_2 + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(r, &outward_normal);
                record.mat_ptr = Arc::clone(&self.mat_ptr);
                return true;
            }
        }
        return false;
    }
    
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}