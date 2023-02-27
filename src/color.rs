use crate::point::Point;
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
    pub fn set_color(&mut self, point: Point) {
        self.red = (255. * point.x) as u8;
        self.green = (255. * point.y) as u8;
        self.blue = (255. * point.z) as u8;
    }

    pub fn to_ppm(&self) -> String {
        format!("{:3} {:3} {:3}", self.red, self.green, self.blue)
    }
}
