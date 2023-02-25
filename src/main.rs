use indicatif::ProgressBar;
use window::Window;
mod color;
mod window;
fn main() {
    let mut screen = Window::new(256, 256);

    let bar = ProgressBar::new((screen.width * screen.height).try_into().unwrap());

    for y in 0..255 {
        for x in 0..255 {
            screen.pixels[y][x].set_color(
                x.try_into().unwrap(),
                (255 - y).try_into().unwrap(),
                100,
            );
            bar.inc(1);
        }
    }
    bar.finish();
    println!("{}", screen.to_ppm());
}
