use crate::color::Color;
pub struct Window {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels: Vec<Vec<Color>> = vec![vec![Color::default(); width]; height];
        Window {
            width,
            height,
            pixels,
        }
    }
    pub fn to_ppm(&self) -> String {
        let mut output = format!("P3\n{} {}\n255\n", self.width, self.height).to_owned();
        for row in self.pixels.iter().rev() {
            for pixel in row {
                output.push_str(&pixel.to_ppm());
                output.push_str(" ");
            }
            output.push_str("\n");
        }
        output
    }
}
