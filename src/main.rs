use raytracer::Tracer;
use scenes::three_balls;
mod body;
mod color;
mod point;
mod raytracer;
mod scenes;
mod window;
fn main() {
    let world = three_balls();
    let mut tracer = Tracer::new(400, 255, 1., world, 30, 50);
    tracer.render();
    println!("{}", tracer.image());
}
