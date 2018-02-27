use std::str::FromStr;
use super::triangle::Triangle;
use super::{Vec3, Solid};

pub struct Rectangle {
    pub p0: Vec3<f64>,
    pub p1: Vec3<f64>,
    pub p2: Vec3<f64>,
    pub p3: Vec3<f64>,

    pub t0: Triangle,
    pub t1: Triangle
}

impl Rectangle {
    pub fn new(p0: Vec3<f64>, p1: Vec3<f64>, p2: Vec3<f64>, p3: Vec3<f64>) -> Self {
        let t0 = Triangle::new(p0, p1, p2);
        let t1 = Triangle::new(p3, p1, p2);

        Rectangle {p0, p1, p2, p3, t0, t1}
    }
}

impl Solid for Rectangle {
    fn intersect(&self, org: Vec3<f64>, dir: Vec3<f64>) -> Option<f64> {
        match self.t0.intersect(org, dir) {
            r @ Some(_) => r,
            None => self.t1.intersect(org, dir)
        }
    }

    fn position(&self) -> Vec3<f64> {self.p0}

    fn normal_at(&self, hit: Vec3<f64>, dir: Vec3<f64>) -> Vec3<f64> {
        self.t0.normal_at(hit, dir)
    }
}

impl FromStr for Rectangle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vectors = s.split(", ").collect::<Vec<&str>>();
        
        if vectors.len() != 4 {
            return Err(());
        }

        let p0 = Vec3::from_str(vectors[0]).or_else(|_| Err(()))?;
        let p1 = Vec3::from_str(vectors[1]).or_else(|_| Err(()))?;
        let p2 = Vec3::from_str(vectors[2]).or_else(|_| Err(()))?;
        let p3 = Vec3::from_str(vectors[3]).or_else(|_| Err(()))?;
        Ok(Self::new(p0, p1, p2, p3)) 
    }
}
