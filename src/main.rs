use raytracer::Tracer;
use scenes::three_balls;
use std::fs::File;
use std::io::Write;
mod body;
mod color;
mod point;
mod raytracer;
mod scenes;
mod window;
fn main() -> std::io::Result<()> {
    let world = three_balls();
    let mut tracer = Tracer::new(400, 255, 1., world, 100, 50);
    tracer.render();
    let mut file = File::create("output.ppm")?;
    file.write_all(&tracer.image().as_bytes().to_vec())?;
    Ok(())
}
