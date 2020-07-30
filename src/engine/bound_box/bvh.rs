use crate::data::rand_int;
use crate::engine::bound_box::aabb::{box_compare_x, box_compare_y, box_compare_z};
use crate::engine::bound_box::AABB;
use crate::engine::{HitRecord, Hittable, Ray};

use std::sync::Arc;

struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    fn build(mut objects: Vec<Arc<dyn Hittable>>) -> BvhNode {
        let axis = rand_int(0, 2);
        let comparator = match axis {
            0 => box_compare_x,
            1 => box_compare_y,
            2 => box_compare_z,
            _ => panic!("Value not in 0..2"),
        };

        let mut left: Arc<dyn Hittable>;
        let mut right: Arc<dyn Hittable>;

        match objects.len() {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => {
                if comparator(objects[0].clone(), objects[1].clone()) == std::cmp::Ordering::Less {
                    left = objects[0].clone();
                    right = objects[1].clone();
                } else {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            }
            _ => {
                objects.sort_by(|a, b| comparator(a.clone(), b.clone()));
                let mid = objects.len() / 2;
                left = BvhNode::build(objects[..mid].to_vec()).share();
                right = BvhNode::build(objects[mid..].to_vec()).share();
            }
        }

        if left.get_bounding_box().is_none() || right.get_bounding_box().is_none() {
            panic!("One of the bounding boxes (either left or right) does not exist!")
        }

        let bbox = AABB::surrounding_box(
            left.get_bounding_box().unwrap(),
            right.get_bounding_box().unwrap(),
        );

        BvhNode { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, tmin, tmax) {
            return false;
        }

        let hit_left = self.left.hit(r, tmin, tmax, record);
        let hit_right = self.right.hit(r, tmin, tmax, record);
        return hit_left || hit_right;
    }

    fn share(self) -> Arc<dyn Hittable> {
        return Arc::new(self);
    }
    fn get_bounding_box(&self) -> Option<AABB> {
        return Option::from(self.bbox);
    }
}
