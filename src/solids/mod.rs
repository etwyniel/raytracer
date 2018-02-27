pub mod sphere;
pub mod triangle;
pub mod rectangle;

//use std::f64;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use super::vec3::Vec3;

pub struct Object {
    pub pos: Vec3<f64>,
    pub emission_color: Vec3<f64>,
    pub surface_color: Vec3<f64>,
    pub transparency: f64,
    pub reflection: f64,
    pub solid: Box<Solid>
}

pub fn html_color_to_vec3(s: &str) -> Result<Vec3<f64>, ()> {
    if s.len() < 7 || !s.starts_with('#') {
        return Err(());
    }

    let r = (u8::from_str_radix(&s[1..3], 16).or_else(|_| Err(()))? as f64) / 255.;
    let g = (u8::from_str_radix(&s[3..5], 16).or_else(|_| Err(()))? as f64) / 255.;
    let b = (u8::from_str_radix(&s[5..7], 16).or_else(|_| Err(()))? as f64) / 255.;

    Ok(Vec3::new(r, g, b))
}

impl Object {
    pub fn new(surface_color: Vec3<f64>, emission_color: Vec3<f64>,
               transparency: f64, reflection: f64, solid: Box<Solid>) -> Self {
        Object {pos: solid.position(), emission_color, surface_color,
            transparency, reflection, solid}
    }

    pub fn from_file(path: &str) -> Result<Vec<Object>, String> {
        let mut file = File::open(path).or_else(|_e| Err(format!("Could not read file {}", path)))?;
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).or_else(|_e| Err(format!("Could not read file {}", path)))?;

        let mut r = Vec::<Object>::new();
        
        for (i, line) in file_str.split('\n').enumerate() {
            let line = line.trim();
            if line.len() == 0 || line.starts_with("//") {
                continue;
            }
            let mut tokens = line.split(", ").collect::<Vec<&str>>();
            if tokens.len() < 5 {
                return Err(format!("Invalid line: {}", i + 1));
            }

            let surface_color = Vec3::from_str(tokens[0])
                .or_else(|_| Err(format!("Invalid reflection value: line {}", i + 1)))? * (1. / 255.);
            let emission_color = Vec3::from_str(tokens[1])
                .or_else(|_| Err(format!("Invalid reflection value: line {}", i + 1)))? * (1. / 255.);

            let reflection = f64::from_str(&tokens[2])
                .or_else(|_| Err(format!("Invalid reflection value: line {}", i + 1)))?;
            let transparency = f64::from_str(&tokens[3])
                .or_else(|_| Err(format!("Invalid transparency value: line {}", i + 1)))?;

            let solid: Box<Solid> = match tokens[4] {
                "sphere" => Box::new(sphere::Sphere::from_str(
                        &tokens[5..].join(", ")).or_else(|_|
                        Err(format!("Invalid sphere definition: line {}", i + 1)))?),

                "triangle" => Box::new(triangle::Triangle::from_str(
                        &tokens[5..].join(", ")).or_else(|_|
                        Err(format!("Invalid triangle definition: line {}", i + 1)))?),

                "rectangle" => Box::new(rectangle::Rectangle::from_str(
                        &tokens[5..].join(", ")).or_else(|_|
                        Err(format!("Invalid rectangle definition: line {}", i + 1)))?),
                _ => {return Err("Invalid input file.".to_string());}
            };

            r.push(Object::new(surface_color, emission_color, reflection, transparency, solid));
        }

        Ok(r)
    }
}

pub trait Solid {
    fn intersect(&self, origin: Vec3<f64>, direction: Vec3<f64>) -> Option<f64>;
    fn normal_at(&self, hit: Vec3<f64>, dir: Vec3<f64>) -> Vec3<f64>;
    fn position(&self) -> Vec3<f64>;
    //fn from_str(&self) -> Result<Self>;
    /*
    fn surface_color(&self) -> Vec3<f64>;
    fn emission_color(&self) -> Vec3<f64>;
    fn transparency(&self) -> f64;
    fn reflection(&self) -> f64;
    */
}
