use std::f32::consts::PI;

use crate::body::{BodyProps, Sphere, World};
use crate::point::Point;
#[allow(dead_code)]
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
    world
}
#[allow(dead_code)]
pub fn two_metal_balls() -> World {
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
    world
}

#[allow(dead_code)]
pub fn two_balls() -> World {
    let mut world = World::new();
    let r: f64 = (PI / 4.).cos().into();
    world.add(Box::new(Sphere::new(
        Point::new(-r, 0., -1.),
        r,
        BodyProps::matte(Point::new(0., 0., 1.)),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(r, 0., -1.),
        r,
        BodyProps::matte(Point::new(1., 0., 0.)),
    )));

    world
}
