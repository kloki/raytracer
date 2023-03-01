use crate::point::Point;
use crate::raytracer::Ray;

pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub t: f64,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Point::default(),
            normal: Point::default(),
            t: 0.,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Point) {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = outward_normal;
        if !front_face {
            self.normal = -outward_normal;
        }
    }
}

pub trait Body {
    fn hit(&self, _ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            return false;
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(ray, (rec.p - self.center) / self.radius);
        true
    }

    fn color(&self, ray: &Ray, angle: f64) -> Point {
        let N = (ray.at(angle) - Point::new(0., 0., -1.)).unit_vector();
        0.5 * Point::new(N.x + 1., N.y + 1., N.z + 1.)
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
}