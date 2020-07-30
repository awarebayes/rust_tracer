use crate::engine::bound_box::AABB;
use crate::engine::Hittable;
use crate::materials::Material;
use nalgebra::Vector3;
use std::sync::Arc;

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    mat_ptr: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, mat_ptr: Arc<dyn Material>) -> XYRect {
        XYRect {x0, x1, y0, y1, z, mat_ptr}
    }

    pub fn share(self) -> Arc<dyn Hittable> {
        Arc::new(self)
    }
}

impl Hittable for XYRect {
    fn hit(
        &self,
        r: &crate::engine::Ray,
        t_min: f64,
        t_max: f64,
        record: &mut crate::engine::HitRecord,
    ) -> bool {
        let t = (self.z - r.origin()[2]) / r.direction()[2];
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin()[0] + t * r.direction()[0];
        let y = r.origin()[1] + t * r.direction()[1];
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        record.u = (x - self.x0) / (self.x1 - self.x0);
        record.v = (y - self.y0) / (self.y1 - self.y0);
        record.t = t;
        let outward_normal = Vector3::new(0.0, 0.0, 1.0);
        record.set_face_normal(r, &outward_normal);
        record.mat_ptr = self.mat_ptr.clone();
        record.p = r.at(t);
        return true;
    }
    fn share(self) -> Arc<dyn Hittable> {
        Arc::new(self)
    }
    fn get_bounding_box(&self) -> Option<AABB> {
        let output_box = AABB::new(
            Vector3::new(self.x0, self.y0, self.z - 0.0001),
            Vector3::new(self.x1, self.y1, self.z + 0.0001),
        );
        return Option::from(output_box);
    }
}

unsafe impl Send for XYRect {}
unsafe impl Sync for XYRect {}
