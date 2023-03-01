use crate::point::Point;
#[derive(Clone, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn default() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn transform_to_color(&self, value: f64, samples: usize) -> u8 {
        match value / samples as f64 {
            v if v < 0. => 0,
            v if v > 0.999 => 255,
            v => (256. * v) as u8,
        }
    }
    pub fn set_color(&mut self, point: Point, samples: usize) {
        self.red = self.transform_to_color(point.x, samples);
        self.green = self.transform_to_color(point.y, samples);
        self.blue = self.transform_to_color(point.z, samples);
    }

    pub fn to_ppm(&self) -> String {
        format!("{:3} {:3} {:3}", self.red, self.green, self.blue)
    }
}
