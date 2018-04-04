use super::Vec3;
use super::vec3::Lerp;
use super::rand::{
    Rng,
    SeedableRng,
    isaac::IsaacRng
};

pub const PERMUTATIONS: usize = 512;
pub const TAU: f64 = 2. * ::std::f64::consts::PI;

pub enum Surface {
    Solid(Vec3<f64>),
    Noise(NoiseSurface)
}

use self::Surface::*;

impl Surface {
    pub fn color_at(&self, x: f64, y: f64) -> Vec3<f64> {
        match self {
            &Solid(c) => c,
            &Noise(ref s) => s.color_at(x, y)
        }
    }
}

pub struct NoiseSurface {
    pub color: Vec3<f64>,
    pub values: Vec<(f64, f64)>,
    perm_table: Vec<usize>
}

fn dot(p0: (f64, f64), p1: (f64, f64)) -> f64 {
    p0.0 * p1.0 + p0.1 * p1.1
}

impl NoiseSurface {
    pub fn new_seeded(color: Vec3<f64>, seed: u64) -> Self {
        let perm_mask = PERMUTATIONS - 1;
        let s = [(seed / (1 << 32)) as u32, (seed & (1 << 32 - 1)) as u32];
        let mut rng = IsaacRng::from_seed(&s);
        let mut values: Vec<(f64, f64)> = vec![(0., 0.); PERMUTATIONS];
        let mut perm_table = vec![0; PERMUTATIONS * 2];
        for i in 0..PERMUTATIONS {
            /*
            let mut x = 2.;
            let mut y = 1.;
            let mut len: f64 = 2.;
            while len > 1. {
                x = rng.gen_range(-1., 1.);
                y = rng.gen_range(-1., 1.);
                len = x * x + y * y;
            }
            len = len.sqrt();

            values[i] = (x / len, y / len);
            */
            let theta = rng.gen_range(0., TAU);
            values[i] = (theta.cos(), theta.sin());
            perm_table[i] = i;
        }

        for k in 0..PERMUTATIONS {
            let i = rng.next_u64() as usize & perm_mask;
            perm_table.swap(k, i);
            perm_table[k + PERMUTATIONS] = perm_table[k];
        }

        NoiseSurface {color, values, perm_table}
    }

    fn hash(&self, x: usize, y: usize) -> usize {
        self.perm_table[self.perm_table[x] + y]
    }

    pub fn color_at(&self, x: f64, y: f64) -> Vec3<f64> {
        let perm_mask = PERMUTATIONS - 1;
        let xi = (x.floor() as usize) & perm_mask;
        let yi = (y.floor() as usize) & perm_mask;

        let tx = x - x.floor() as f64;
        let ty = y - y.floor() as f64;

        let rx0 = xi & perm_mask;
        let rx1 = (xi + 1) & perm_mask;
        let ry0 = yi & perm_mask;
        let ry1 = (yi + 1) & perm_mask;

        let c00 = self.values[self.hash(rx0, ry0)];
        let c10 = self.values[self.hash(rx1, ry0)];
        let c01 = self.values[self.hash(rx0, ry1)];
        let c11 = self.values[self.hash(rx1, ry1)];

        let x0 = tx;
        let x1 = tx - 1.;
        let y0 = ty;
        let y1 = ty - 1.;

        let p00 = (x0, y0);
        let p10 = (x1, y0);
        let p01 = (x0, y1);
        let p11 = (x1, y1);

        let nx0 = dot(c00, p00).slerp(dot(c10, p10), tx);
        let nx1 = dot(c01, p01).slerp(dot(c11, p11), tx);
        
        let res = nx0.slerp(nx1, ty) as f64;
        /*
        if res < 0. {
            println!("less than 0");
        }
        */

        self.color * ((res + 1.) * 0.5)
    }
}
