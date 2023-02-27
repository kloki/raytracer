use raytracer::Tracer;
mod color;
mod point;
mod raytracer;
mod window;
fn main() {
    let mut tracer = Tracer::new(400, 255, 2., 1.25, 1.);
    // let mut screen = Window::new(256, 256);

    // let bar = ProgressBar::new((screen.width * screen.height).try_into().unwrap());

    // for y in 0..255 {
    //     for x in 0..255 {
    //         screen.pixels[y][x].set_color(
    //             x.try_into().unwrap(),
    //             (255 - y).try_into().unwrap(),
    //             100,
    //         );
    //         bar.inc(1);
    //     }
    // }
    // bar.finish();
    tracer.render();
    println!("{}", tracer.image());
}
