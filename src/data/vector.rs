use rand::distributions::{Open01, Uniform};
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::ops;

/*
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn unit_vector(&self) -> Vector {
        let k = 1.0 / self.len();
        Vector {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vector, rhs: &Vector) -> Vector {
        Vector {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: -(lhs.x * rhs.z - lhs.z * rhs.x),
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn random() -> Vector {
        Vector::new(rand_float01(), rand_float01(), rand_float01())
    }

    pub fn random_range(min: f64, max: f64) -> Vector {
        Vector::new(
            rand_float(min, max),
            rand_float(min, max),
            rand_float(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Vector {
        let a: f64 = rand_float(0.0, 2.0 * PI);
        let z: f64 = rand_float(-1.0, 1.0);
        let r: f64 = (1.0 - z * z).sqrt();
        return Vector::new(r * a.cos(), r * a.sin(), z);
    }

    pub fn random_in_hemisphere(normal: &Vector) -> Vector {
        let in_unit_sphere = Vector::random_in_unit_sphere();
        if Vector::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -1.0 * in_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vector {
        loop {
            let p = Vector::new(rand_float(-1.0, 1.0), rand_float(-1.0, 1.0), 0.0);
            if p.len_squared() > 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn reflect(v: &Vector, n: &Vector) -> Vector {
        v - 2.0 * Vector::dot(v, n) * n
    }

    pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = Vector::dot(&(-1.0 * uv), n);
        let r_out_parallel: Vector = etai_over_etat * (uv + cos_theta * n);
        let r_out_perp: Vector = -(1.0 - r_out_parallel.len_squared()).sqrt() * n;
        return r_out_parallel + r_out_perp;
    }
}
*/

use nalgebra::{Vector3, Unit};
use crate::data::{rand_float01, rand_float};



pub fn vrandom() -> Vector3<f64> {
    Vector3::new(rand_float01(), rand_float01(), rand_float01())
}

pub fn vrandom_range(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        rand_float(min, max),
        rand_float(min, max),
        rand_float(min, max),
    )
}
pub fn vrandom_in_unit_sphere() -> Vector3<f64> {
    let a: f64 = rand_float(0.0, 2.0 * PI);
    let z: f64 = rand_float(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();
    return Vector3::new(r * a.cos(), r * a.sin(), z);
}

pub fn vrandom_in_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = vrandom_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -1.0 * in_unit_sphere;
    }
}

pub fn vrandom_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(rand_float(-1.0, 1.0), rand_float(-1.0, 1.0), 0.0);
        if p.dot(&p) > 1.0 {
            continue;
        }
        return p;
    }
}

pub fn vunit(v: &Vector3<f64>) -> Vector3<f64>{
    Unit::new_normalize(*v).into_inner()
}

pub fn vlen(v: &Vector3<f64>) -> f64 {
    v.dot(v).sqrt()
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * (v.dot(&n)) * n
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-1.0 * uv).dot(&n);
    let r_out_parallel: Vector3<f64> = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp: Vector3<f64> = -(1.0 - r_out_parallel.dot(&r_out_parallel)).sqrt() * n;
    return r_out_parallel + r_out_perp;
}