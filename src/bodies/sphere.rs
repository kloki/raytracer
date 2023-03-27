use crate::bodies::bodyprops::BodyProps;
use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::point::Point;
use crate::raytracer::Ray;
pub struct Sphere {
    center: Point,
    radius: f64,
    body_props: BodyProps,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, body_props: BodyProps) -> Self {
        Sphere {
            center,
            radius,
            body_props,
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
        rec.body_props = self.body_props;
        true
    }
    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.center - Point::new(self.radius, self.radius, self.radius),
            self.center + Point::new(self.radius, self.radius, self.radius),
        )
    }
}
