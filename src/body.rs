use crate::point::Point;
use crate::raytracer::Ray;
pub trait Body {
    fn hit(&self, _ray: &Ray) -> Option<f64> {
        None
    }
    fn color(&self, _ray: &Ray, _angle: f64) -> Point {
        Point::new(1., 0., 0.)
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
    color: Point,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, color: Point) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Body for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2. * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        }
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }

    fn color(&self, ray: &Ray, angle: f64) -> Point {
        let N = (ray.at(angle) - Point::new(0., 0., -1.)).unit_vector();
        0.5 * Point::new(N.x + 1., N.y + 1., N.z + 1.)
    }
}
