extern crate raytracer;
use raytracer::vec3::*;
use raytracer::solids::sphere::Sphere;
use raytracer::solids::triangle::Triangle;
use raytracer::solids::Object;

fn main() {
    /*
    let spheres: Vec<Box<Solid>> = vec![
        Box::new(Sphere::new(Vec3::new(0., -10004., -20.), 10000., Vec3::new(0.20, 0.20, 0.20),
            Vec3::new(0.1, 0.1, 0.1), 0., 0.)),
        Box::new(Sphere::new(Vec3::new(0.,      0., -20.),    4., Vec3::new(1.0, 0.32, 0.36),
            Vec3::default(), 1., 0.5)),
        Box::new(Sphere::new(Vec3::new(5.,     -1., -15.),    2., Vec3::new(0.90, 0.76, 0.46),
            Vec3::default(), 1., 0.)),
        Box::new(Sphere::new(Vec3::new(5.,      0., -25.),    3., Vec3::new(0.65, 0.77, 0.97),
            Vec3::default(), 1., 0.)),
        Box::new(Sphere::new(Vec3::new(-5.,     0., -15.),    3., Vec3::new(0.90, 0.90, 0.90),
            Vec3::default(), 1., 0.)),
        Box::new(Sphere::new(Vec3::new(0.,     20., -10.),    0.2, Vec3::new(0., 0., 0.),
            Vec3::new(8., 8., 8.), 1., 0.))];
    ];
            */
    /*
    let spheres: Vec<Object> = vec![
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
    ];*/
    let spheres = Object::from_file("in.rtcr").unwrap();
    raytracer::render(720, 360, &spheres);
}
