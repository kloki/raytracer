use std::fs::File;
use std::io::Write;
mod bodies;
mod point;
mod raytracer;
mod scenes;
fn main() -> std::io::Result<()> {
    let mut tracer = scenes::three_balls();
    tracer.render();
    let mut file = File::create("output.ppm")?;
    file.write_all(&tracer.image().as_bytes().to_vec())?;
    Ok(())
}
