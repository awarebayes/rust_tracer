use std::ops;
use rand::{thread_rng, Rng};
use rand::distributions::{Open01, Uniform};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    x:f64,
    y:f64,
    z:f64,
}

impl Vector{
    pub fn new(x:f64, y:f64, z:f64) -> Vector {
        Vector { x,y,z }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }


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
        Vector::new(
            thread_rng().sample(Open01),
            thread_rng().sample(Open01),
            thread_rng().sample(Open01),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Vector {
        Vector::new(
            thread_rng().sample(Uniform::from(min..max)),
            thread_rng().sample(Uniform::from(min..max)),
            thread_rng().sample(Uniform::from(min..max)),
        )
    }
    pub fn random_in_unit_sphere() -> Vector {
        let a:f64 = thread_rng().sample(Uniform::from(0.0..2.0*PI));
        let z: f64 = thread_rng().sample(Uniform::from(-1.0..1.0));
        let r: f64 = (1.0-z*z).sqrt();
        return Vector::new(r*a.cos(), r*a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vector) -> Vector {
        let in_unit_sphere = Vector::random_in_unit_sphere();
        if Vector::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -1.0 * in_unit_sphere
        }
    }

    pub fn reflect(v: &Vector, n: &Vector) -> Vector {
        v - 2.0 * Vector::dot(v,n) * n
    }

    pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = Vector::dot(&(-1.0*uv), n);
        let r_out_parallel: Vector = etai_over_etat * (uv + cos_theta * n);
        let r_out_perp: Vector = -(1.0 - r_out_parallel.len_squared()).sqrt() * n;
        return r_out_parallel + r_out_perp;
    }
}

fn add_vectors(lhs: &Vector, rhs: &Vector) -> Vector {
    Vector {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
}

fn subtract_vectors(lhs: &Vector, rhs: &Vector) -> Vector {
    Vector {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
}
fn neg_vector(vector: &Vector) -> Vector {
    Vector {
        x: -vector.x,
        y: -vector.y,
        z: -vector.z,
    }
}

fn add_vector_and_scalar(lhs: &Vector, rhs: f64) -> Vector {
    Vector {
        x: lhs.x + rhs,
        y: lhs.y + rhs,
        z: lhs.z + rhs,
    }
}

fn mul_vector_and_scalar(lhs: &Vector, rhs: f64) -> Vector {
    Vector {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
}

fn div_vector_and_scalar(lhs: &Vector, rhs: f64) -> Vector {
    Vector {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
        z: lhs.z / rhs,
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        add_vectors(self, rhs)
    }
}

impl ops::Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        add_vectors(self, &rhs)
    }
}

impl ops::Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        add_vectors(&self, rhs)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        add_vectors(&self, &rhs)
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        subtract_vectors(self, rhs)
    }
}

impl ops::Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        subtract_vectors(self, &rhs)
    }
}

impl ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        subtract_vectors(&self, rhs)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        subtract_vectors(&self, &rhs)
    }
}

impl ops::Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        neg_vector(&self)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        neg_vector(&self)
    }
}

impl ops::Add<f64> for &Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Vector {
        add_vector_and_scalar(self, rhs)
    }
}

impl ops::Add<&Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        add_vector_and_scalar(rhs, self)
    }
}

impl ops::Add<f64> for Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Vector {
        add_vector_and_scalar(&self, rhs)
    }
}

impl ops::Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        add_vector_and_scalar(&rhs, self)
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        mul_vector_and_scalar(self, rhs)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        mul_vector_and_scalar(&self, rhs)
    }
}

impl ops::Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        mul_vector_and_scalar(rhs, self)
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        mul_vector_and_scalar(&rhs, self)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        div_vector_and_scalar(&self, rhs)
    }
}

impl ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        div_vector_and_scalar(&self, rhs)
    }
}

