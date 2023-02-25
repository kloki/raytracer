use window::Window;
mod color;
mod window;
fn main() {
    let mut screen = Window::new(256, 256);

    for y in 0..255 {
        for x in 0..255 {
            screen.pixels[y][x].set_color(
                x.try_into().unwrap(),
                (255 - y).try_into().unwrap(),
                100,
            );
        }
    }
    println!("{}", screen.to_ppm());
}
