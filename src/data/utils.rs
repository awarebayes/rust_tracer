
use rand::distributions::{Open01, Uniform};
use rand::{thread_rng, Rng};

pub fn rand_float(from: f64, to: f64) -> f64 {
    thread_rng().sample(Uniform::from(from..to))
}

pub fn rand_float01() -> f64 {
    thread_rng().sample(Open01)
}

pub fn rand_int(from: usize, to: usize) -> usize {
    thread_rng().sample(Uniform::from((from as f64)..(to as f64))).round() as usize
}