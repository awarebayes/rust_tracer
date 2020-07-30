use crate::engine::{Hittable, Ray};
use nalgebra::Vector3;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct AABB {
    min: Vector3<f64>,
    max: Vector3<f64>,
}

fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}
fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

impl AABB {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> AABB {
        AABB { min, max }
    }

    pub fn blank() -> AABB {
        AABB::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0))
    }

    pub fn hit(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            }
            if t0 > tmin {
                tmin = t0
            }
            if t1 < tmax {
                tmax = t1
            }
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }

    pub fn min(&self) -> Vector3<f64> {
        self.min
    }
    pub fn max(&self) -> Vector3<f64> {
        self.max
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vector3::new(
            min(box0.min()[0], box1.min()[0]),
            min(box0.min()[1], box1.min()[1]),
            min(box0.min()[2], box1.min()[2]),
        );
        let big = Vector3::new(
            max(box0.max()[0], box1.max()[0]),
            max(box0.max()[1], box1.max()[1]),
            max(box0.max()[2], box1.max()[2]),
        );
        return AABB::new(small, big);
    }
}

pub fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.get_bounding_box();
    let box_b = b.get_bounding_box();
    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in BVH constructor")
    }
    let v1 = box_a.unwrap().min()[axis];
    let v2 = box_b.unwrap().min()[axis];

    if v1 > v2 { return Ordering::Greater }
    if v1 == v2 { return Ordering::Equal }
    if v1 < v2  { return Ordering::Less }
    return Ordering::Equal;
}

pub fn box_compare_x(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
    box_compare(a.clone(), b.clone(), 0)
}

pub fn box_compare_y(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
    box_compare(a.clone(), b.clone(), 1)
}

pub fn box_compare_z(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
    box_compare(a.clone(), b.clone(), 2)
}