use raytracer::Tracer;
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
    let mut tracer = Tracer::new(400, (400. / aspect_ratio) as usize, 1., world, 100, 50);
    tracer.render();
    let mut file = File::create("output.ppm")?;
    file.write_all(&tracer.image().as_bytes().to_vec())?;
    Ok(())
}
