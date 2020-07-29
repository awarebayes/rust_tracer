
use rand::distributions::{Open01, Uniform};
use rand::{thread_rng, Rng};

pub fn rand_float(from: f64, to: f64) -> f64 {
    thread_rng().sample(Uniform::from(from..to))
}

pub fn rand_float01() -> f64 {
    thread_rng().sample(Open01)
}

