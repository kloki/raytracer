use std::fmt::Debug;

use crate::bodies::bodyprops::BodyProps;
use crate::point::Point;
use crate::raytracer::Ray;
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}
impl AABB {
    pub fn new(min: Point, max: Point) -> Self {
        AABB { min, max }
    }

    pub fn from_bodies(bodies: &Vec<Box<dyn Body>>) -> Option<AABB> {
        if bodies.is_empty() {
            return None;
        }
        let mut bbox = bodies[0].bounding_box();
        for body in bodies.iter().skip(1) {
            bbox = bbox.surrounding_box(body.bounding_box())
        }
        Some(bbox)
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let min = self.min.as_array();
        let max = self.max.as_array();
        let origin = ray.origin.as_array();
        let direction = ray.direction.as_array();
        for i in 0..3 {
            let t0 = ((min[i] - origin[i]) / direction[i]).min((max[i] - origin[i]) / direction[i]);
            let t1 = ((min[i] - origin[i]) / direction[i]).max((max[i] - origin[i]) / direction[i]);
            if t1.min(t_max) <= t0.max(t_min) {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(&self, other: AABB) -> AABB {
        Self::new(
            Point::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            Point::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub body_props: BodyProps,
    pub front_face: bool,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Point::default(),
            normal: Point::default(),
            body_props: BodyProps::null(),
            front_face: true,
            t: 0.,
            u: 0.,
            v: 0.,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Point) {
        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = outward_normal;
        if !self.front_face {
            self.normal = -outward_normal;
        }
    }
}

pub trait Body: Sync + Send + Debug {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
    fn bounding_box(&self) -> AABB;
    fn color(&self, _ray: &Ray, _angle: f64) -> Point {
        Point::new(1., 0., 0.)
    }
}
