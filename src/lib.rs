extern crate png;
extern crate rayon;

pub mod vec3;
pub mod solids;
use vec3::*;
use solids::Object;
use std::fs::File;
use std::io::BufWriter;
use png::{Encoder, HasParameters, ColorType, BitDepth};
use rayon::prelude::*;

const MAX_DEPTH: i32 = 5;

fn mix(a: f64, b: f64, mix: f64) -> f64 {b * mix + a * (1. - mix)}

pub fn trace(org: Vec3<f64>, dir: Vec3<f64>, objects: &Vec<Object>, depth: i32) -> Vec3<f64> {
    let mut tnear = ::std::f64::MAX;
    let mut obj: Option<&Object> = None;

    for object in objects.iter() {
        let t = match object.solid.intersect(org, dir) {
            Some(v) => v,
            None => {continue;}
        };
        if t < tnear {
            tnear = t;
            obj = Some(&object);
        }
    }
    let obj = match obj {
        // Making the background a gradient instead of a solid color
        None => { let mut c =  Vec3::new(0.1, 0.3, 0.5) *
                            dir.dot(&Vec3::new(0., 0., -1.)).powi(2);
                  let intensity = c.len();
                  return *c.normalize() * intensity;},
        Some(o) => o
    };

    let mut surface_color: Vec3<f64> = Vec3::default();
    let phit = org + dir * tnear;
    let mut nhit = obj.solid.normal_at(phit, dir);

    let bias = 1e-4f64;
    let inside = if dir.dot(&nhit) > 0. {
        nhit = -nhit;
        true
    } else {false};

    if (obj.transparency > 0. || obj.reflection > 0.) && depth < MAX_DEPTH {
        let facingratio = -dir.dot(&nhit);
        let fresneleffect = mix((1. - facingratio).powi(3), 1., 0.1);

        let mut refldir = dir - nhit * 2. * dir.dot(&nhit);
        let reflection = trace(phit + nhit * bias, *refldir.normalize(), objects, depth + 1);

        let mut refraction = Vec3::<f64>::default();
        if obj.transparency > 0. {
            let ior = 1.1;
            let eta = if inside {ior} else {1. / ior};
            let cosi = -nhit.dot(&dir);
            let k = 1. - eta * eta * (1. - cosi * cosi);

            let mut refrdir = dir * eta + nhit * (eta * cosi - k.sqrt());
            refraction = trace(phit - nhit * bias, *refrdir.normalize(), objects, depth + 1);
        }
        surface_color = obj.surface_color * (reflection * fresneleffect +
                            refraction *(1. - fresneleffect) * obj.transparency);
    } else {
        for (i, o) in objects.iter().enumerate() {
            if o.emission_color.x > 0. {
                let mut light_direction = o.pos - phit;

                // Light fall-off with distance
                let dist2 = light_direction.len_sqr();
                let val = 1. -  0.3 * dist2 / (1. + dist2.abs());
                //let mut transmission = Vec3::new(1., 1., 1.);
                let mut transmission = Vec3::new(val, val, val);

                light_direction.normalize();
                for (j, x) in objects.iter().enumerate() {
                    if i != j {
                        match x.solid.intersect(phit + nhit * bias, light_direction) {
                            Some(_) => {transmission = Vec3::default(); break;},
                            None => ()
                        }
                    }
                }
                surface_color = surface_color + obj.surface_color * transmission *
                    (nhit.dot(&light_direction).max(0.)) * o.emission_color;
            }
        }
    }
    /*
    if surface_color.len_sqr() > 1. {
        surface_color.normalize();
    }*/
    let mut color = surface_color + obj.emission_color;
    //let intensity = color.len().min(1.);
    if color.len_sqr() > 1. {
        color.normalize();
    }
    //surface_color * intensity + obj.emission_color
    color
}

fn get_hit_object_id(org: Vec3<f64>, dir: Vec3<f64>, objects: &Vec<Object>) -> isize {
    let mut tnear = ::std::f64::MAX;
    let mut id = -1;
    for (i, o) in objects.iter().enumerate() {
        let t = match o.solid.intersect(org, dir) {
            Some(v) => v,
            None => {continue;}
        };
        if t < tnear {
            tnear = t;
            id = i as isize;
        }
    }

    id
}

pub fn render_wireframe(width: usize, height: usize, objects: &Vec<Object>, filename: &str) {
    let mut img = vec![Vec3::default(); width * height];
    let inv_width = 1. / (width as f64);
    let inv_height = 1. / (height as f64);
    let fov = 50.;
    let aspect_ratio = width as f64 * inv_height;
    let angle = (::std::f64::consts::PI * 0.5 * fov / 180.).tan();

    let mut hits = vec![-1; width * height];

    // Doing first row
    let yy = (1. - 2. * (0.5 * inv_height)) * angle;
    for x in 0..width {
        let xx = (2. * ((x as f64 + 0.5) * inv_width) - 1.) * angle * aspect_ratio;
        let mut dir = Vec3::new(xx, yy, -1.);
        dir.normalize();
        
        hits[x] = get_hit_object_id(Vec3::default(), dir, objects);
    }

    // Doing first column
    let xx = (2. * (0.5 * inv_height) - 1.) * angle * aspect_ratio;
    for y in 0..height {
        let yy = (1. - 2. * ((y as f64 + 0.5) * inv_height)) * angle;
        let mut dir = Vec3::new(xx, yy, -1.);
        dir.normalize();
        
        hits[width * y] = get_hit_object_id(Vec3::default(), dir, objects);
    }

    for y in 1..height {
        let line = y * width;
        let yy = (1. - 2. * ((y as f64 + 0.5) * inv_height)) * angle;
        for x in 1..width {
            let xx = (2. * ((x as f64 + 0.5) * inv_width) - 1.) * angle * aspect_ratio;
            let mut dir = Vec3::new(xx, yy, -1.);
            dir.normalize();
            
            let val = get_hit_object_id(Vec3::default(), dir, objects);
            hits[line + x] = val;
            if val != hits[line + x - 1] || val != hits[line + x - width] {
                img[line + x] = Vec3::new(1., 1., 1.);
            }
        }
    }

    /*
    for y in 1..height {
        let line = y * width;
        for x in 1..width {
            let cur = hits[line + x];
            img[line + x] = if cur != hits[line + x - 1] || cur != hits[line + x - width] {
                Vec3::new(1., 1., 1.)
            } else {
                Vec3::default()
            };
        }
    }
    */

    write_to_file(width, height, &img, filename).unwrap();
}

fn get_yy(y: usize, inv_height: f64, angle: f64) -> f64 {
    (1. - 2. * ((y as f64 + 0.5) * inv_height)) * angle
}

fn get_xx(x: usize, inv_width: f64, angle: f64, aspect_ratio: f64) -> f64 {
    (2. * ((x as f64 + 0.5) * inv_width) - 1.) * angle * aspect_ratio
}

pub fn render(width: usize, height: usize, objects: &Vec<Object>, filename: &str) {
    let mut img = vec![Vec3::default(); width * height];
    //let mut pixel = &image[..];
    let inv_width = 1. / (width as f64);
    let inv_height = 1. / (height as f64);
    let fov = 50.;
    let aspect_ratio = width as f64 * inv_height;
    let angle = (::std::f64::consts::PI * 0.5 * fov / 180.).tan();
    {
        let mut rows: Vec<(usize, &mut [Vec3<f64>])> = 
            img.chunks_mut(width)
            .enumerate()
            .collect();

        rows.par_iter_mut()
            .for_each(move |&mut (y, ref mut row)| {
                let yy = get_yy(y, inv_height, angle);
                let xx = (inv_width - 1.) * angle * aspect_ratio;
                let mut dir = Vec3::new(xx, yy, -1.);
                dir.normalize();
                row[0] = trace(Vec3::default(), dir, &objects, 0);

                for hx in 0..(width / 2 - 1) {
                    let mut x = 2 * (hx + 1);
                    let mut xx = get_xx(x, inv_width, angle, aspect_ratio);
                    let mut dir = Vec3::new(xx, yy, -1.);
                    dir.normalize();
                    row[x] = trace(Vec3::default(), dir, &objects, 0);
                    
                    if (row[x - 2] - row[x]).len_sqr() > 0.5 {
                        x -= 1;
                        xx = get_xx(x, inv_width, angle, aspect_ratio);
                        dir = Vec3::new(xx, yy, -1.);
                        dir.normalize();
                        row[x] = trace(Vec3::default(), dir, &objects, 0);
                    } else {
                        row[x - 1] = (row[x - 2] + row[x]) * 0.5;
                    }
              }
            });
    }
    // Single threaded version
    /*
    for y in 0..height {
        let line = y * width;
        let yy = (1. - 2. * ((y as f64 + 0.5) * inv_height)) * angle;
        for x in 0..width {
            let xx = (2. * ((x as f64 + 0.5) * inv_width) - 1.) * angle * aspect_ratio;
            let mut dir = Vec3::new(xx, yy, -1.);
            dir.normalize();
            img[line + x] = trace(Vec3::default(), dir, objects, 0);
        }
    }
    */

    write_to_file(width, height, &img, filename).unwrap();
}

fn write_to_file(width: usize, height: usize, img: &Vec<Vec3<f64>>, filename: &str) ->
    Result<(), ()> {

    let mut bytes = Vec::with_capacity(width * height * 3);

    for pix in img {
        bytes.push((pix.x.min(1.) * 255.) as u8);
        bytes.push((pix.y.min(1.) * 255.) as u8);
        bytes.push((pix.z.min(1.) * 255.) as u8);
    }

    let w: BufWriter<Box<::std::io::Write>> = match filename {
        "-" => BufWriter::new(Box::new(::std::io::stdout())),
        f => BufWriter::new(Box::new(File::create(f).or_else(|_| Err(()))?))
    };

    let mut encoder = Encoder::new(w, width as u32, height as u32);
    encoder.set(ColorType::RGB).set(BitDepth::Eight);
    let mut writer = encoder.write_header().or_else(|_| Err(()) )?;
    writer.write_image_data(&bytes).or_else(|_| Err(()))
}
