#[derive(Clone, Debug)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue }
    }
    pub fn default() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.red = red as f32 / 255.0;
        self.green = green as f32 / 255.0;
        self.blue = blue as f32 / 255.0;
    }
    pub fn to_ppm(&self) -> String {
        format!(
            "{:3} {:3} {:3}",
            (255.0 * self.red) as u8,
            (255.0 * self.green) as u8,
            (255.0 * self.blue) as u8
        )
    }
}
