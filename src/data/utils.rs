use nalgebra::{Vector3, Unit};

use rand::distributions::{Open01, Uniform};
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub fn rand_float(from: f64, to: f64) -> f64 {
    thread_rng().sample(Uniform::from(from..to))
}

pub fn rand_float01() -> f64 {
    thread_rng().sample(Open01)
}

