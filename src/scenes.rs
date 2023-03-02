use crate::body::{BodyProps, Sphere, World};
use crate::point::Point;
pub fn three_balls() -> World {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        BodyProps::matte(Point::new(0.8, 0.8, 0.)),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        // BodyProps::matte(Point::new(0.1, 0.2, 0.5)),
        BodyProps::glass(1.5),
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
    world
}
