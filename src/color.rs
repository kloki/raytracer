#[derive(Clone, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
    pub fn default() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.red = red;
        self.green = green;
        self.blue = blue;
    }
    pub fn to_ppm(&self) -> String {
        format!("{:3} {:3} {:3}", self.red, self.green, self.blue)
    }
}
