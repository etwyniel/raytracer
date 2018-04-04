use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Neg};

pub trait Lerp {
    fn lerp(&self, dst: Self, t: f64) -> Self
        where Self: ::std::marker::Sized;

    fn slerp(&self, dst: Self, t: f64) -> Self 
        where Self: ::std::marker::Sized {
        // cos remap
        //self.lerp(dst, (1. - (t * ::std::f64::consts::PI).cos()) * 0.5)

        // smoothstep remap
        //self.lerp(dst, t * t * (3. - 2. * t))

        // smoothstep remap v2
        self.lerp(dst, t * t * t * (6. * t * t - 15. * t + 10.))
    }
}

impl Lerp for f32 {
    fn lerp(&self, dst: f32, t: f64) -> f32 {
        self + (dst - self) * (t as f32)
    }
}

impl Lerp for f64 {
    fn lerp(&self, dst: f64, t: f64) -> f64 {
        self + (dst - self) * t
    }
}

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Default for Vec3<T> where T: Default {
    fn default() -> Self {
        Vec3{x: T::default(), y: T::default(), z: T::default()}
    }
}

impl Clone for Vec3<f64> {
    fn clone(&self) -> Self {
        Vec3::<f64>::new(self.x, self.y, self.z)
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl FromStr for Vec3<f64> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        if tokens.len() != 3 {
            return Err(());
        }
        let x = f64::from_str(tokens[0].trim()).or_else(|_| Err(()))?;
        let y = f64::from_str(tokens[1].trim()).or_else(|_| Err(()))?;
        let z = f64::from_str(tokens[2].trim()).or_else(|_| Err(()))?;
        Ok(Vec3::new(x, y, z))
    }
}

impl Copy for Vec3<f64> {}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {x, y, z}
    }
}

impl<T> Vec3<T> where T: Mul<T, Output=T> + Add<T, Output=T> + Copy {
    pub fn len_sqr(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Vec3<f64> {
    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn normalize(&mut self) -> &Self {
        let nor_sqr = self.len_sqr();
        if nor_sqr > 0. {
            let inv_nor = 1. / nor_sqr.sqrt();
            self.x *= inv_nor;
            self.y *= inv_nor;
            self.z *= inv_nor;
        }
        self
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Vec3<f64> {
        Vec3::new(self.y * rhs.z - self.z * rhs.y,
                  self.z * rhs.x - self.x * rhs.z,
                  self.x * rhs.y - self.y * rhs.x)
    }

    pub fn abs(&self) -> Self {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn powi(&self, i: i32) -> Self {
        Vec3::new(self.x.powi(i), self.y.powi(i), self.z.powi(i))
    }

    pub fn powf(&self, f: f64) -> Self {
        Vec3::new(self.x.powf(f), self.y.powf(f), self.z.powf(f))
    }

    pub fn cartesian_to_spherical(&self) -> Vec3<f64> {
        let r = self.len();
        let theta = (self.z / r).acos();
        let phi = (self.y / self.x).atan();
        Vec3::new(r, theta, phi)
    }

    // Probably not going to use it, but might as well implement it
    // for completeness' sake
    pub fn spherical_to_cartesian(&self) -> Vec3<f64> {
        let sin_th = self.y.sin();
        let x = self.x * sin_th * self.z.cos();
        let y = self.x * sin_th * self.z.sin();
        let z = self.x * self.y.cos();
        Vec3::<f64>::new(x, y, z)
    }
}

impl Lerp for Vec3<f64> {
    fn lerp(&self, dst: Vec3<f64>, t: f64) -> Vec3<f64> {
        /*
        let x = self.x + (dst.x - self.x) * t;
        let y = self.y + (dst.y - self.z) * t;
        let z = self.z + (dst.z - self.y) * t;
        Vec3 {x, y, z}
        */
        Vec3::new(self.x.lerp(dst.x, t),
                  self.y.lerp(dst.y, t),
                  self.z.lerp(dst.z, t))
    }
}

impl<T, O> Neg for Vec3<T> where T: Neg<Output=O> {
    type Output = Vec3<O>;
    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl<L, R, O> Mul<R> for Vec3<L> where L: Mul<R, Output=O>, R: Copy + PartialOrd {
    type Output = Vec3<O>;
    fn mul(self, rhs: R) -> Self::Output {
        Vec3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl<T> Add for Vec3<T> where T: Add<Output=T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<T> Sub for Vec3<T> where T: Sub<Output=T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T> Mul for Vec3<T> where T: Mul<Output=T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: Vec3<T>) -> Self {
        Vec3 {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

pub fn solve_quadratic(eq: &Vec3<f64>) -> Option<(f64, f64)> {
    let del = eq.y * eq.y - 4. * eq.x * eq.z;
    if del < 0. {
        return None;
    }
    let root = del.sqrt();
    let x0 = (-eq.y - root) / (2. * eq.x);
    // Not really useful; unlikely to happen with misc f64 values
    //if root == 0 {return Some((x0, x0));}
    let x1 = (-eq.y + root) / (2. * eq.x);
    Some((x0, x1))
}

#[test]
fn quadratic_test() {
    let (x0, x1) = match solve_quadratic(&Vec3::new(1., 1., -20.)) {
        Some((t0, t1)) => (t0, t1),
        None => {panic!("Sould have two roots");}
    };
    assert_eq!((x0, x1), (-5., 4.));
}
