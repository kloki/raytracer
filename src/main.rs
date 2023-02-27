use body::{Body, Sphere};
use point::Point;
use raytracer::Tracer;
mod body;
mod color;
mod point;
mod raytracer;
mod window;
fn main() {
    // let bodies: Vec<Box<dyn Body>> = vec![
    //     Box::new(Sphere::new(Point::new(0., 0., -2.), 0.5, Point::red())),
    //     Box::new(Sphere::new(Point::new(1.2, 0.5, -3.), 0.9, Point::green())),
    // ];

    let bodies: Vec<Box<dyn Body>> = vec![Box::new(Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        Point::red(),
    ))];
    let mut tracer = Tracer::new(400, 255, 2., 1.25, 1., bodies);
    tracer.render();
    println!("{}", tracer.image());
}
