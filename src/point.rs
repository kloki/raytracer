use rand::Rng;
use std::ops;
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
    pub fn default() -> Self {
        Point {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Point {
                x: rng.gen_range(0f64..2.) - 1.,
                y: rng.gen_range(0f64..2.) - 1.,
                z: rng.gen_range(0f64..2.) - 1.,
            };
            if p.length_squared() >= 1. {
                return p;
            }
        }
    }
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Point {
                x: rng.gen_range(0f64..2.) - 1.,
                y: rng.gen_range(0f64..2.) - 1.,
                z: 0.,
            };
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }
    pub fn random_in_hemisphere(normal: Point) -> Self {
        let in_unit = Self::random_in_unit_sphere();
        if in_unit.dot(normal) > 0. {
            return in_unit;
        } else {
            return -in_unit;
        }
    }

    pub fn red() -> Self {
        Point {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn green() -> Self {
        Point {
            x: 0.,
            y: 1.,
            z: 0.,
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 0.0000000000001;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, n: Point) -> Point {
        *self - n * self.dot(n) * 2.
    }
    pub fn refract(&self, n: Point, etai_over_etat: f64) -> Point {
        let cos_theta = (-*self).dot(n).min(1.);
        let r_out_perp = (*self + (n * cos_theta)) * etai_over_etat;
        let r_out_parallel = n * -(((1.0 - r_out_perp.length_squared()).abs()).sqrt());
        r_out_perp + r_out_parallel
    }
    pub fn dot(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Point) -> Point {
        Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Point {
        self / self.length()
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

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

impl ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        other * self
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
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
        let sum = Point::new(1., 2., 3.) + Point::new(3., 2., 1.);
        assert_eq!(sum.x, 4.);
        assert_eq!(sum.y, 4.);
        assert_eq!(sum.z, 4.);
    }
    #[test]
    fn test_scalar_multiplication() {
        let scaled = Point::new(1., 2., 3.) * 3.;
        assert_eq!(scaled.x, 3.);
        assert_eq!(scaled.y, 6.);
        assert_eq!(scaled.z, 9.);
    }
}
