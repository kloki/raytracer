use crate::bodies::bodyprops::BodyProps;
use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::point::Point;
use crate::raytracer::Ray;
#[derive(Debug)]
pub struct Rect {
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    axis: Axis,
    body_props: BodyProps,
}

pub enum Axis {
    XY,
    XZ,
    YZ,
}
impl Rect {
    pub fn new(
        a0: f64,
        a1: f64,
        b0: f64,
        b1: f64,
        k: f64,
        axis: Axis,
        body_props: BodyProps,
    ) -> Rect {
        Rect {
            a0,
            a1,
            b0,
            b1,
            k,
            axis,
            body_props,
        }
    }
}

impl Body for Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = match self.axis {
            Axis::XY => (self.k - ray.origin.z) / ray.direction.z,
            Axis::XZ => (self.k - ray.origin.y) / ray.direction.y,
            Axis::YZ => (self.k - ray.origin.x) / ray.direction.x,
        };
        if t < t_min || t > t_max {
            return false;
        }
        let a = match self.axis {
            Axis::XY => ray.origin.x + t * ray.direction.x,
            Axis::XZ => ray.origin.x + t * ray.direction.x,
            Axis::YZ => ray.origin.y + t * ray.direction.y,
        };
        let b = match self.axis {
            Axis::XY => ray.origin.y + t * ray.direction.y,
            Axis::XZ => ray.origin.z + t * ray.direction.z,
            Axis::YZ => ray.origin.z + t * ray.direction.z,
        };
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return false;
        }
        rec.t = t;
        rec.set_face_normal(ray, Point::new(0., 0., 1.));
        rec.p = ray.at(t);
        rec.body_props = self.body_props;
        true
    }

    fn bounding_box(&self) -> AABB {
        match self.axis {
            Axis::XY => AABB::new(
                Point::new(self.a0, self.b0, self.k - 0.0001),
                Point::new(self.a1, self.b1, self.k + 0.0001),
            ),
            Axis::XZ => AABB::new(
                Point::new(self.a0, self.k - 0.0001, self.b0),
                Point::new(self.a1, self.k + 0.0001, self.b1),
            ),
            Axis::YZ => AABB::new(
                Point::new(self.k - 0.0001, self.a0, self.b0),
                Point::new(self.k + 0.0001, self.a1, self.b1),
            ),
        }
    }
}
