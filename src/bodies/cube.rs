use crate::bodies::bodyprops::BodyProps;
use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::bodies::rect::{Axis, Rect};
use crate::point::Point;
use crate::raytracer::Ray;
pub struct Cube {
    min: Point,
    max: Point,
    sides: [Rect; 6],
}

impl Cube {
    pub fn new(p0: Point, p1: Point, body_props: BodyProps) -> Cube {
        Cube {
            min: p0,
            max: p1,
            sides: [
                Rect::new(p0.x, p1.x, p0.y, p1.y, p1.z, Axis::XY, body_props),
                Rect::new(p0.x, p1.x, p0.y, p1.y, p0.z, Axis::XY, body_props),
                Rect::new(p0.x, p1.x, p0.z, p1.z, p1.y, Axis::XZ, body_props),
                Rect::new(p0.x, p1.x, p0.z, p1.z, p0.y, Axis::XZ, body_props),
                Rect::new(p0.y, p1.y, p0.z, p1.z, p1.x, Axis::YZ, body_props),
                Rect::new(p0.y, p1.y, p0.z, p1.z, p0.x, Axis::YZ, body_props),
            ],
        }
    }
    pub fn new_as_sphere(center: Point, radius: f64, body_props: BodyProps) -> Self {
        let offset = Point::new(radius, radius, radius);
        Cube::new(center - offset, center + offset, body_props)
    }
}

impl Body for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_for = t_max;

        for body in &self.sides {
            if body.hit(ray, t_min, closest_so_for, rec) {
                hit_anything = true;
                closest_so_for = rec.t;
            }
        }
        return hit_anything;
    }
    fn bounding_box(&self) -> AABB {
        AABB::new(self.min, self.max)
    }
}
