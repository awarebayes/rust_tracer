use nalgebra::Vector3;

use crate::engine::bound_box::AABB;
pub use crate::engine::hittable::Hittable;
use crate::engine::{HitRecord, Ray};
use crate::materials::Material;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, mat_ptr: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
    pub fn get_uv(p: &Vector3<f64>, u: &mut f64, v: &mut f64) {
        let phi = p[2].atan2(p[0]);
        let theta = p[1].asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
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
                Sphere::get_uv(&outward_normal, &mut record.u, &mut record.v);
                record.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let temp = (-b_2 + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = r.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(r, &outward_normal);
                Sphere::get_uv(&outward_normal, &mut record.u, &mut record.v);
                record.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }

    fn share(self) -> Arc<dyn Hittable> {
        Arc::new(self)
    }

    fn get_bounding_box(&self) -> Option<AABB> {
        Option::from(AABB::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        ))
    }
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}


