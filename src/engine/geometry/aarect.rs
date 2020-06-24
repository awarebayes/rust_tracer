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


pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    y: f64,
    mat_ptr: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, mat_ptr: Arc<dyn Material>) -> XZRect {
        XZRect {x0, x1, z0, z1, y, mat_ptr}
    }

    pub fn share(self) -> Arc<dyn Hittable> {
        Arc::new(self)
    }
}

impl Hittable for XZRect {
    fn hit(
        &self,
        r: &crate::engine::Ray,
        t_min: f64,
        t_max: f64,
        record: &mut crate::engine::HitRecord,
    ) -> bool {
        let t = (self.y - r.origin()[1]) / r.direction()[1];
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin()[0] + t * r.direction()[0];
        let z = r.origin()[2] + t * r.direction()[2];
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        record.u = (x - self.x0) / (self.x1 - self.x0);
        record.v = (z - self.z0) / (self.z1 - self.z0);
        record.t = t;
        let outward_normal = Vector3::new(0.0, 1.0, 0.0);
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
            Vector3::new(self.x0, self.y + 0.0001, self.z0),
            Vector3::new(self.x1, self.y - 0.0001, self.z1),
        );
        return Option::from(output_box);
    }
}

unsafe impl Send for XZRect {}
unsafe impl Sync for XZRect {}



pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64,
    mat_ptr: Arc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, mat_ptr: Arc<dyn Material>) -> YZRect {
        YZRect {y0, y1, z0, z1, x, mat_ptr}
    }

    pub fn share(self) -> Arc<dyn Hittable> {
        Arc::new(self)
    }
}

impl Hittable for YZRect {
    fn hit(
        &self,
        r: &crate::engine::Ray,
        t_min: f64,
        t_max: f64,
        record: &mut crate::engine::HitRecord,
    ) -> bool {
        let t = (self.x - r.origin()[0]) / r.direction()[0];
        if t < t_min || t > t_max {
            return false;
        }

        let y = r.origin()[1] + t * r.direction()[1];
        let z = r.origin()[2] + t * r.direction()[2];
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        record.u = (y - self.y0) / (self.y1 - self.y0);
        record.v = (z - self.z0) / (self.z1 - self.z0);
        record.t = t;
        let outward_normal = Vector3::new(1.0, 0.0, 0.0);
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
            Vector3::new(self.x+0.0001, self.y0, self.z0),
            Vector3::new(self.x-0.0001, self.y1, self.z1),
        );
        return Option::from(output_box);
    }
}

unsafe impl Send for YZRect {}
unsafe impl Sync for YZRect {}
