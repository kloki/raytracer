use crate::bodies::bodyprops::BodyProps;
use crate::point::Point;
use crate::raytracer::Ray;
use std::iter::zip;
#[derive(Clone, Copy)]
pub struct AABB {
    a: Point,
    b: Point,
}
impl AABB {
    pub fn new(a: Point, b: Point) -> Self {
        AABB { a, b }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let zipped = zip(
            zip(self.a.as_array(), self.b.as_array()),
            zip(ray.origin.as_array(), ray.direction.as_array()),
        );
        for ((a, b), (origin, direction)) in zipped {
            let t0 = (a - origin / direction).min(b - origin / direction);
            let t1 = (a - origin / direction).max(b - origin / direction);
            if t1.min(t_max) <= t0.max(t_min) {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(&self, other: AABB) -> AABB {
        Self::new(
            Point::new(
                self.a.x.min(other.a.x),
                self.a.y.min(other.a.y),
                self.a.z.min(other.a.z),
            ),
            Point::new(
                self.b.x.max(other.b.x),
                self.b.y.max(other.b.y),
                self.b.z.max(other.b.z),
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
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Point::default(),
            normal: Point::default(),
            body_props: BodyProps::null(),
            front_face: true,
            t: 0.,
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

pub trait Body: Sync + Send {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
    fn bounding_box(&self) -> AABB;
    fn color(&self, _ray: &Ray, _angle: f64) -> Point {
        Point::new(1., 0., 0.)
    }
}
pub struct World {
    bodies: Vec<Box<dyn Body>>,
}

impl World {
    pub fn new() -> World {
        World { bodies: vec![] }
    }
    pub fn add(&mut self, body: Box<dyn Body>) {
        self.bodies.push(body);
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut rec = HitRecord::default();
        let mut closest_so_for = t_max;

        for body in &self.bodies {
            if body.hit(ray, t_min, closest_so_for, &mut rec) {
                hit_anything = true;
                closest_so_for = rec.t;
            }
        }
        if !hit_anything {
            return None;
        }
        Some(rec)
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        if self.bodies.is_empty() {
            return None;
        }
        let mut bounding_box = self.bodies[0].bounding_box();
        for body in self.bodies.iter().skip(1) {
            bounding_box = bounding_box.surrounding_box(body.bounding_box())
        }
        Some(bounding_box)
    }
}
