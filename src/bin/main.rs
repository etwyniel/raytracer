extern crate raytracer;
use raytracer::vec3::*;
use raytracer::solids::sphere::Sphere;
use raytracer::solids::triangle::Triangle;
use raytracer::solids::Object;
use std::env::args;
use std::process::exit;

fn main() {
    let default: Vec<Object> = vec![
        Object::new(Vec3::new(0.20, 0.20, 0.20), Vec3::new(0.1, 0.1, 0.1), 0., 0.,
            Box::new(Sphere::new(Vec3::new(0., -10004., -20.), 10000.))),
        Object::new(Vec3::new(1.0, 0.32, 0.36), Vec3::default(), 0.9, 0.5,
            Box::new(Sphere::new(Vec3::new(0.,      0., -20.),    4.))),
        Object::new(Vec3::new(0.90, 0.76, 0.46), Vec3::default(), 1., 0.,
            Box::new(Sphere::new(Vec3::new(5.,     -1., -15.),    2.))),
        Object::new(Vec3::new(0.65, 0.77, 0.97), Vec3::default(), 0.2, 0.0,
            Box::new(Sphere::new(Vec3::new(5.,      0., -25.),    3.))),
        Object::new(Vec3::new(0.90, 0.90, 0.90), Vec3::default(), 1., 0.8,
            Box::new(Sphere::new(Vec3::new(-5.,     0., -15.),    3.))),
        Object::new(Vec3::new(0., 0., 0.), Vec3::new(5., 5., 5.), 1., 0.,
            Box::new(Sphere::new(Vec3::new(0.,     20., -10.),    0.2, ))),
        Object::new(Vec3::new(0., 0., 0.), Vec3::new(2.5, 2., 2.), 1., 0.,
            Box::new(Sphere::new(Vec3::new(0.,      0., 1.),    0.2, ))),
        Object::new(Vec3::new(0.3, 0.9, 0.3), Vec3::default(), 1., 0.,
            Box::new(Triangle::new(
                        Vec3::new(5., -3.5, -15.),
                        Vec3::new(-3., -3.5, -10.),
                        Vec3::new(1., -3.5, -18.)
                    )))
    ];
    let mut spheres = default;

    let mut out_name = "out.png".to_string();
    let mut width = 1280;
    let mut height = 720;
    let mut func: Box<fn(usize, usize, &Vec<Object>, &str)> = Box::new(raytracer::render);

    let mut names: Vec<String> = Vec::with_capacity(2);
    let mut ar = args().skip(1);
    loop {
        match ar.next() {
            None => {break;},

            // stdin/stdout
            Some(ref s) if s == "-" => {names.push(s.to_string());}

            // Long option names
            Some(ref s) if s.starts_with("--") => {
                match &s[2..s.len()] {
                    "wireframe" => {func = Box::new(raytracer::render_wireframe);}
                    opt @ "width" | opt @ "height" => {
                        let val = match ar.next() {
                            None => {eprintln!("Option --{} requires a value", opt);
                                exit(1);},
                            Some(ref st) => match st.parse::<usize>() {
                                Ok(v) => v,
                                _ => {eprintln!("Invalid {} value", opt); exit(1);}
                            }
                        };
                        match opt {
                            "width" => {width = val;}
                            "height" => {height = val;}
                            _ => {panic!();}
                        };
                    },
                    o => {eprintln!("Unknown option: --{}", o); exit(1);}
                }
            },

            // Short option names
            Some(ref s) if s.starts_with("-") => {
                let mut flags = s.chars().skip(1);
                let mut i = 1;
                loop {
                    i += 1;
                    match flags.next() {
                        None => {break;},
                        Some('W') => {func = Box::new(raytracer::render_wireframe);},

                        // Value in this arg or the next?
                        Some(c) if c == 'w' || c == 'h' => {
                            let sval = if i < s.len() {
                                flags.collect::<String>()
                            } else {
                                match ar.next() {
                                    Some(ref st) => st.clone(),
                                    None => {eprintln!("Option -{} requires an argument", c);
                                        exit(1);
                                    }
                                }
                            };
                            let val = match sval.parse::<usize>() {
                                Ok(v) => v,
                                _ => {eprintln!("Invalid value for option -{}", c); exit(1);}
                            };
                            match c {
                                'w' => {width = val;},
                                'h' => {height = val;}
                                _ => {panic!();}
                            }
                            break;
                        },
                        Some(f) => {eprintln!("Unknown flag: -{}", f); exit(1);}
                    }
                }
            },
            Some(ref s) => {names.push(s.to_string());}
        }
    }

    if names.len() > 0 {
        out_name = names[names.len() - 1].clone();


        //TODO expand spheres with every input file
        if names.len() > 1 {
            spheres = match Object::from_file(&names[0]) {
                Ok(o) => o,
                Err(s) => {eprintln!("Error reading file: {}", s); exit(1);}
            };
        }
    }
    func(width, height, &spheres, &out_name);

    //raytracer::surface_test();
}
