use crate::body::{BodyProps, Sphere, World};
use crate::point::Point;
use crate::raytracer::{Camera, Tracer};
use rand::Rng;
#[allow(dead_code)]
pub fn three_balls() -> Tracer {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        BodyProps::matte(Point::new(0.8, 0.8, 0.)),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        BodyProps::matte(Point::new(0.1, 0.2, 0.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        BodyProps::glass(1.5),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        BodyProps::metal(Point::new(0.8, 0.6, 0.2), 0.),
    )));
    let aspect_ratio = 16. / 9.;
    let look_from = Point::new(1., 1., 5.);
    let look_at = Point::new(0., 0., -1.);
    let camera = Camera::new(
        look_from,
        look_at,
        Point::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        5.,
    );
    Tracer::new(400, (400. / aspect_ratio) as usize, camera, world, 100, 50)
}
#[allow(dead_code)]
pub fn two_metal_balls() -> Tracer {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        BodyProps::matte(Point::new(0.8, 0.8, 0.)),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        BodyProps::matte(Point::new(0.7, 0.3, 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        BodyProps::metal(Point::new(0.8, 0.8, 0.8), 0.),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        BodyProps::metal(Point::new(0.8, 0.6, 0.2), 0.),
    )));
    let aspect_ratio = 16. / 9.;
    let look_from = Point::new(1., 1., 5.);
    let look_at = Point::new(0., 0., -1.);
    let camera = Camera::new(
        look_from,
        look_at,
        Point::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        5.,
    );
    Tracer::new(400, (400. / aspect_ratio) as usize, camera, world, 100, 50)
}

#[allow(dead_code)]
pub fn book_cover() -> Tracer {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        Point::new(0., -1000., -1.),
        1000.0,
        BodyProps::matte(Point::new(0.5, 0.5, 0.5)),
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let location = Point::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            match rng.gen::<f64>() {
                x if x < 0.8 => {
                    world.add(Box::new(Sphere::new(
                        location,
                        0.2,
                        BodyProps::matte(Point::random() * Point::random()),
                    )));
                }
                x if x < 0.95 => {
                    world.add(Box::new(Sphere::new(
                        location,
                        0.2,
                        BodyProps::metal(Point::random(), rng.gen_range(0.5..1.)),
                    )));
                }
                _ => {
                    world.add(Box::new(Sphere::new(location, 0.2, BodyProps::glass(1.5))));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point::new(0., 1., 0.),
        1.,
        BodyProps::glass(1.5),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-4., 1., 0.),
        1.,
        BodyProps::matte(Point::new(0.8, 0.8, 0.)),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(4., 1., 0.),
        1.,
        BodyProps::metal(Point::new(0.7, 0.6, 0.5), 0.),
    )));
    let aspect_ratio = 3. / 2.;
    let image_width: usize = 1200;
    let image_height = (1200. / aspect_ratio) as usize;
    let look_from = Point::new(13., 2., 3.);
    let look_at = Point::new(0., 0., 0.);
    let camera = Camera::new(
        look_from,
        look_at,
        Point::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        10.,
    );
    Tracer::new(image_width, image_height, camera, world, 500, 50)
}
