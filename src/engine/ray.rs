use crate::data::export::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Vector,
    dir: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Vector {
        self.orig
    }
    pub fn direction(&self) -> Vector {
        self.dir
    }
    pub fn at(&self, t: f64) -> Vector {
        self.orig + t * self.dir
    }
}
