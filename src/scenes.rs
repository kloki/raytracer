use crate::body::{BodyProps, Sphere, World};
use crate::point::Point;
pub fn three_balls() -> World {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        BodyProps::lambertian(Point::new(0.8, 0.8, 0.)),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        BodyProps::lambertian(Point::new(0.7, 0.3, 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        BodyProps::metal(Point::new(0.8, 0.8, 0.8), 0.3),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        BodyProps::metal(Point::new(0.8, 0.6, 0.2), 1.),
    )));
    world
}
