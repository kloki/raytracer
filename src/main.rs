use std::fs::File;
use std::io::Write;
mod body;
mod color;
mod point;
mod raytracer;
mod scenes;
mod window;
fn main() -> std::io::Result<()> {
    let mut tracer = scenes::phone_wallpaper();
    tracer.render();
    let mut file = File::create("output.ppm")?;
    file.write_all(&tracer.image().as_bytes().to_vec())?;
    Ok(())
}
