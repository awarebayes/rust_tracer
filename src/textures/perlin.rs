use crate::data::{ rand_float01, rand_int };
use nalgebra::Vector3;

/*
const COUNT: usize = 256;

pub struct Perlin {
    ranfloat: [f64; COUNT],
    perm_x: [usize; COUNT],
    perm_y: [usize; COUNT],
    perm_z: [usize; COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat: [f64; COUNT] = [0.0; COUNT];
        for i in 0..COUNT {
            ranfloat[i] = rand_float01();
        }
        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();
        Perlin { ranfloat, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: &Vector3<f64>) -> f64 {
        let u = p[0] - p[0].floor();
        let v = p[1] - p[1].floor();
        let w = p[2] - p[2].floor();

        let i = (4 * p[0].round() as usize) & (COUNT-1);
        let j = (4 * p[1].round() as usize) & (COUNT-1);
        let k = (4 * p[2].round() as usize) & (COUNT-1);

        return self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
    }

    fn generate_perm() -> [usize; COUNT] {
        let mut p: [usize; COUNT] = [0; COUNT];
        for i in 0..COUNT {
            p[i] = i;
        }
        Perlin::permute(&mut p);
        return p
    }

    fn permute(p: &mut [usize; COUNT]) {
        for i in 1..COUNT-1 {
            let j = COUNT-1-i;
            let target = rand_int(0, j);
            let tmp = p[j];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}

*/

