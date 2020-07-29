use crate::engine::bound_box::AABB;
use crate::engine::{HitRecord, Hittable, Ray};
use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let hit = object.hit(r, t_min, t_max, &mut temp);
            if hit && temp.t < closest_so_far {
                hit_anything = true;
                closest_so_far = temp.t;
                *record = temp.clone();
            }
        }
        return hit_anything;
    }
    fn share(self) -> Arc<dyn Hittable> {
        Arc::new(self)
    }

    // Calculates biggest bounding box for all objects inside
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return Option::default();
        }

        let mut output_box = AABB::blank();
        let mut first_box = true;

        for obj in self.objects.clone() {
            match obj.bounding_box(t0, t1) {
                Some(temp_box) => {
                    if first_box {
                        output_box = temp_box;
                    } else {
                        output_box = AABB::surrounding_box(output_box, temp_box);
                    }
                    first_box = false;
                }
                None => (),
            }
        }

        return Option::from(output_box);
    }
}
