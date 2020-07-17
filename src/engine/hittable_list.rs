use crate::engine::hittable::Hittable;
use crate::engine::export::{ Ray, HitRecord };

use std::rc::Rc;
use std::cell::RefCell;

pub struct HittableList {
    objects: Vec<Rc<RefCell<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> HittableList{
        HittableList{
            objects: Vec::new(),
        }
    }

    /*
    pub fn clear(&mut self){
        self.objects.clear();
    }
     */

    pub fn add(&mut self, object: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool{
        let mut temp = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let hit = object.borrow().hit(r, t_min, t_max, &mut temp);
            if hit && temp.t < closest_so_far {
                hit_anything = true;
                closest_so_far = temp.t;
                *record = temp.clone();
            }
        }
        return hit_anything;
    }
}
