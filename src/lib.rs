pub mod vec3;
pub mod solid;
use vec3::*;
use solid::Solid;
use std::fs::File;
use std::io::prelude::*;

const MAX_DEPTH: i32 = 5;

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub radius2: f64,
    pub surface_color: Vec3<f64>,
    pub emission_color: Vec3<f64>,
    pub transparency: f64,
    pub reflection: f64
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64, surface_color: Vec3<f64>,
               emission_color: Vec3<f64>, transparency: f64, reflection: f64) -> Self {
        Sphere {center, radius, radius2: radius * radius, surface_color,
            emission_color, transparency, reflection}
    }
}

impl Solid for Sphere {
    fn intersect(&self, org: Vec3<f64>, dir: Vec3<f64>) -> Option<f64> {
        /*
        let l = self.center - org;
        let tca = l.dot(&dir);
        if tca < 0. {return None;};
        let d2 = l.dot(&l) - tca * tca;
        if d2 > self.radius2 {return None;};
        let thc = (self.radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 > t1 {
            if t1 < 0. {if t0 < 0. {None} else {Some(t0)}} else {Some(t1)}
        } else {
            if t0 < 0. {if t1 < 0. {None} else {Some(t1)}} else{Some(t0)}
        }
        */
        let l = org - self.center;
        match solve_quadratic(&Vec3::new(dir.dot(&dir), 2. * dir.dot(&l),
            l.dot(&l) - self.radius2)) {

            None => None,
            Some((t0, t1)) => {
                let (t0, t1) = if t0 > t1 {(t1, t0)} else {(t0, t1)};
                if t0 < 0. {
                    if t1 < 0. {None} else {Some(t1)}
                } else {
                    Some(t0)
                }
            }
        }
    }

    fn emission_color(&self) -> Vec3<f64> {self.emission_color}
    fn surface_color(&self) -> Vec3<f64> {self.surface_color}
    fn transparency(&self) -> f64 {self.transparency}
    fn reflection(&self) -> f64 {self.reflection}
    fn position(&self) -> Vec3<f64> {self.center}

    fn normal_at(&self, hit: Vec3<f64>) -> Vec3<f64> {
        let mut res = hit - self.center;
        res.normalize();
        res
    }
}

fn mix(a: f64, b: f64, mix: f64) -> f64 {b * mix + a * (1. - mix)}

pub fn trace(org: Vec3<f64>, dir: Vec3<f64>, objects: &Vec<Box<Solid>>, depth: i32) -> Vec3<f64> {
    let mut tnear = ::std::f64::MAX;
    let mut obj: Option<&Box<Solid>> = None;

    for object in objects {
        let t = match object.intersect(org, dir) {
            Some(v) => v,
            None => {continue;}
        };
        if t < tnear {
            tnear = t;
            obj = Some(&object);
        }
    }
    let obj = match obj {
        None => { return Vec3::new(0., 0., 0.); },
        Some(o) => o
    };

    let mut surface_color: Vec3<f64> = Vec3::default();
    let phit = org + dir * tnear;
    let mut nhit = obj.normal_at(phit);

    let bias = 1e-4f64;
    let inside = if dir.dot(&nhit) > 0. {
        nhit = -nhit;
        true
    } else {false};

    if (obj.transparency() > 0. || obj.reflection() > 0.) && depth < MAX_DEPTH {
        let facingratio = -dir.dot(&nhit);
        let fresneleffect = mix((1. - facingratio).powi(3), 1., 0.1);

        let mut refldir = dir - nhit * 2. * dir.dot(&nhit);
        let reflection = trace(phit + nhit * bias, *refldir.normalize(), objects, depth + 1);

        let mut refraction = Vec3::default();
        if obj.transparency() > 0. {
            let ior = 1.1;
            let eta = if inside {ior} else {1. / ior};
            let cosi = -nhit.dot(&dir);
            let k = 1. - eta * eta * (1. - cosi * cosi);

            let mut refrdir = dir * eta + nhit * (eta * cosi - k.sqrt());
            refraction = trace(phit - nhit * bias, *refrdir.normalize(), objects, depth + 1);
        }
        surface_color = obj.surface_color() * ((reflection * fresneleffect) + refraction *
            (1. - fresneleffect) * obj.transparency());
    } else {
        for (i, o) in objects.iter().enumerate() {
            if o.emission_color().x > 0. {
                let mut transmission = Vec3::new(1., 1., 1.);
                let mut light_direction = o.position() - phit;
                light_direction.normalize();
                for (j, x) in objects.iter().enumerate() {
                    if i != j {
                        match x.intersect(phit + nhit * bias, light_direction) {
                            Some(_) => {transmission = Vec3::default(); break;},
                            None => ()
                        }
                    }
                }
                surface_color = surface_color + obj.surface_color() * transmission *
                    (nhit.dot(&light_direction).max(0.)) * o.emission_color();
            }
        }
    }
    surface_color + obj.emission_color()
}

pub fn render(objects: &Vec<Box<Solid>>) {
    let width = 1366;
    let height = 768;
    let mut image = vec![Vec3::default(); width * height];
    //let mut pixel = &image[..];
    let inv_width = 1. / (width as f64);
    let inv_height = 1. / (height as f64);
    let fov = 30.;
    let aspect_ratio = width as f64 * inv_height;
    let angle = (::std::f64::consts::PI * 0.5 * fov / 180.).tan();

    for y in 0..height {
        let line = y * width;
        let yy = (1. - 2. * ((y as f64 + 0.5) * inv_height)) * angle;
        for x in 0..width {
            let xx = (2. * ((x as f64 + 0.5) * inv_width) - 1.) * angle * aspect_ratio;
            let mut dir = Vec3::new(xx, yy, -1.);
            dir.normalize();
            image[line + x] = trace(Vec3::default(), dir, objects, 0);
        }
    }

    let mut file = File::create("out.ppm").unwrap();
    file.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes());
    for i in 0..(width * height) {
        let p = &image[i];
        file.write_all(&vec![(p.x.min(1.) * 255.) as u8,
                            (p.y.min(1.) * 255.) as u8,
                            (p.z.min(1.) * 255.) as u8]);
    }
}
