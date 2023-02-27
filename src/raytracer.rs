use crate::point::Point;
pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Point {
        self.origin + distance * self.direction
    }
}
