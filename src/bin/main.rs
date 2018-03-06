extern crate raytracer;
use raytracer::vec3::*;
use raytracer::solids::sphere::Sphere;
use raytracer::solids::triangle::Triangle;
use raytracer::solids::Object;
use std::env::args;

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
    let args = args().collect::<Vec<String>>();
    if args.len() > 1 {
        spheres = Object::from_file(&args[1]).unwrap();
    }
    if args.len() > 2 {
        out_name = args[2].clone();
    }
    raytracer::render(1366, 768, &spheres, &out_name);
}
