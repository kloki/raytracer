use body::{Sphere, World};
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
        Point::new(0., 0., -1.),
        0.5,
        Point::red(),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
        Point::green(),
    )));

    let mut tracer = Tracer::new(400, 255, 1., world, 100);
    tracer.render();
    println!("{}", tracer.image());
}
