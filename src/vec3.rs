use std::ops::{Add, Sub, Mul, Neg};

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
