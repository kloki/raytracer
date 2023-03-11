use point::Point;
use raytracer::{Camera, Tracer};
use std::fs::File;
use std::io::Write;
mod body;
mod color;
mod point;
mod raytracer;
mod scenes;
mod window;
fn main() -> std::io::Result<()> {
    let world = scenes::three_balls();
    let aspect_ratio = 16. / 9.;
    let look_from = Point::new(3., 3., 2.);
    let look_at = Point::new(0., 0., -1.);
    let camera = Camera::new(
        look_from,
        look_at,
        Point::new(0., 1., 0.),
        20.,
        aspect_ratio,
        2.,
        (look_from - look_at).length(),
    );
    let mut tracer = Tracer::new(400, (400. / aspect_ratio) as usize, camera, world, 100, 50);
    tracer.render();
    let mut file = File::create("output.ppm")?;
    file.write_all(&tracer.image().as_bytes().to_vec())?;
    Ok(())
}
