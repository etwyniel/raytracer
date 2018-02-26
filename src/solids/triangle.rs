use super::{Vec3, Solid};

pub struct Triangle {
    pub p0: Vec3<f64>,
    pub p1: Vec3<f64>,
    pub p2: Vec3<f64>,
    pub u: Vec3<f64>,
    pub v: Vec3<f64>
}

impl Triangle {
    pub fn new(p0: Vec3<f64>, p1: Vec3<f64>, p2: Vec3<f64>) -> Self {
        Triangle{p0, p1, p2, u: p1 - p0, v: p2 - p0}
    }
}

impl Solid for Triangle {
    fn position(&self) -> Vec3<f64> {self.p0}

    fn intersect(&self, org: Vec3<f64>, dir: Vec3<f64>) -> Option<f64> {
        let EPSILON = 1e-4f64;

        let h = dir * self.v;
        let a = self.u.dot(&h);
        if a > -EPSILON && a < EPSILON {
            return None;
        }
        let f = 1. / a;
        let s = org - self.p0;
        let w = f * s.dot(&h);

        if w < 0. || w > 1. {
            return None;
        }
        let q = s * self.u;
        let z = f * dir.dot(&q);
        if z < 0. || w + z > 1. {
            return None;
        }
        let t = f * self.v.dot(&q);
        Some(t)
    }

    fn normal_at(&self, p: Vec3<f64>) -> Vec3<f64> {
        Vec3::default()
    }
}
