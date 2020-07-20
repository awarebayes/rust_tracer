use nalgebra::Vector3;


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Vector3<f64> {
        self.orig
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.dir
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.orig + t * self.dir
    }
}
