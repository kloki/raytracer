use std::ops;
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }

    pub fn dot(&self, other: Point) -> isize {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Point) -> Point {
        Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<isize> for Point {
    type Output = Point;

    fn mul(self, other: isize) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl ops::Mul<Point> for isize {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        other * self
    }
}

impl ops::Div<isize> for Point {
    type Output = Point;

    fn div(self, other: isize) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn test_additon() {
        let sum = Point::new(1, 2, 3) + Point::new(3, 2, 1);
        assert_eq!(sum.x, 4);
        assert_eq!(sum.y, 4);
        assert_eq!(sum.z, 4);
    }
    #[test]
    fn test_scalar_multiplication() {
        let scaled = Point::new(1, 2, 3) * 3;
        assert_eq!(scaled.x, 3);
        assert_eq!(scaled.y, 6);
        assert_eq!(scaled.z, 9);
    }
}
