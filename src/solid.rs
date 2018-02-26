use super::vec3::{Vec3, solve_quadratic};

pub trait Solid {
    fn intersect(&self, origin: Vec3<f64>, direction: Vec3<f64>) -> Option<f64>;
    fn surface_color(&self) -> Vec3<f64>;
    fn emission_color(&self) -> Vec3<f64>;
    fn transparency(&self) -> f64;
    fn reflection(&self) -> f64;
    fn normal_at(&self, hit: Vec3<f64>) -> Vec3<f64>;
    fn position(&self) -> Vec3<f64>;
}

