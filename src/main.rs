use body::{BodyProps, Material, Sphere, World};
use point::Point;
use raytracer::Tracer;
mod body;
mod color;
mod point;
mod raytracer;
mod window;
fn main() {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        BodyProps::new(Point::new(0.8, 0.8, 0.), Material::Lambertian, 0.),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        BodyProps::new(Point::new(0.7, 0.3, 0.3), Material::Lambertian, 0.),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        BodyProps::new(Point::new(0.8, 0.8, 0.8), Material::Metal, 0.3),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        BodyProps::new(Point::new(0.8, 0.6, 0.2), Material::Metal, 1.),
    )));

    let mut tracer = Tracer::new(400, 255, 1., world, 250, 50);
    tracer.render();
    println!("{}", tracer.image());
}
