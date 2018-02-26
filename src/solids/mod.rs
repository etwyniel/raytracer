pub mod sphere;
pub mod triangle;

use super::vec3::{Vec3, solve_quadratic};

pub struct Object {
    pub pos: Vec3<f64>,
    pub emission_color: Vec3<f64>,
    pub surface_color: Vec3<f64>,
    pub transparency: f64,
    pub reflection: f64,
    pub solid: Box<Solid>
}

impl Object {
    pub fn new(surface_color: Vec3<f64>, emission_color: Vec3<f64>,
               transparency: f64, reflection: f64, solid: Box<Solid>) -> Self {
        Object {pos: solid.position(), emission_color, surface_color,
            transparency, reflection, solid}
    }
}

pub trait Solid {
    fn intersect(&self, origin: Vec3<f64>, direction: Vec3<f64>) -> Option<f64>;
    fn normal_at(&self, hit: Vec3<f64>) -> Vec3<f64>;
    fn position(&self) -> Vec3<f64>;
    /*
    fn surface_color(&self) -> Vec3<f64>;
    fn emission_color(&self) -> Vec3<f64>;
    fn transparency(&self) -> f64;
    fn reflection(&self) -> f64;
    */
}

