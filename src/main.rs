use std::fs::File;
use std::io::Write;
extern crate ordered_float;
mod bodies;
mod point;
mod raytracer;
mod scenes;
fn main() -> std::io::Result<()> {
    let tracer = scenes::book_cover();
    let mut file = File::create("output.ppm")?;
    file.write_all(&tracer.image().as_bytes().to_vec())?;
    Ok(())
}
