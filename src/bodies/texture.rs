use crate::point::Point;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum Texture {
    SolidColor(Point),
    Checkered(Point, Point),
}

impl Texture {
    pub fn new_color(x: f64, y: f64, z: f64) -> Self {
        Texture::SolidColor(Point::new(x, y, z))
    }
    pub fn random_color() -> Self {
        Texture::SolidColor(Point::random())
    }
    pub fn color(&self, _u: f64, _v: f64, p: Point) -> Point {
        match self {
            Texture::SolidColor(s) => *s,
            Texture::Checkered(even, odd) => {
                let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
                if sines < 0. {
                    return *odd;
                }
                *even
            }
        }
    }
}
