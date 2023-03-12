use crate::point::Point;
use crate::raytracer::Ray;
use std::iter::zip;

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
                self.b.x.min(other.b.x),
                self.b.y.min(other.b.y),
                self.b.z.min(other.b.z),
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

pub trait Body {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
    fn bounding_box(&self) -> AABB;
    fn color(&self, _ray: &Ray, _angle: f64) -> Point {
        Point::new(1., 0., 0.)
    }
}

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

    fn color(&self, ray: &Ray, angle: f64) -> Point {
        let n = (ray.at(angle) - Point::new(0., 0., -1.)).unit_vector();
        0.5 * Point::new(n.x + 1., n.y + 1., n.z + 1.)
    }
}

pub struct World {
    bodies: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World { bodies: vec![] }
    }
    pub fn add(&mut self, body: Sphere) {
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

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian,
    Dielectric(f64),
    Metal(f64),
    Ether,
}

#[derive(Debug, Clone, Copy)]
pub struct BodyProps {
    color: Point,
    material: Material,
}

impl BodyProps {
    #[allow(dead_code)]
    pub fn new(color: Point, material: Material) -> Self {
        BodyProps { color, material }
    }
    pub fn metal(color: Point, fuzziness: f64) -> Self {
        BodyProps {
            color,
            material: Material::Metal(fuzziness),
        }
    }

    pub fn matte(color: Point) -> Self {
        BodyProps {
            color,
            material: Material::Lambertian,
        }
    }
    pub fn glass(index_refraction: f64) -> Self {
        BodyProps {
            color: Point::new(1., 1., 1.),
            material: Material::Dielectric(index_refraction),
        }
    }
    pub fn null() -> Self {
        BodyProps {
            color: Point::default(),
            material: Material::Ether,
        }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Point, Ray)> {
        match self.material {
            Material::Ether => None,
            Material::Lambertian => {
                let mut scatter_direction = rec.normal + Point::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::new(rec.p, scatter_direction);
                Some((self.color, scattered))
            }
            Material::Metal(fuzziness) => {
                let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + fuzziness * Point::random_in_unit_sphere(),
                );
                Some((self.color, scattered))
            }
            Material::Dielectric(index_refraction) => {
                let refraction_ratio = if rec.front_face {
                    1.0 / index_refraction
                } else {
                    index_refraction
                };

                let unit_direction = ray_in.direction.unit_vector();

                let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();
                let directed: Point;
                let cannot_refract = (refraction_ratio * sin_theta) > 1.;
                let schlick_approximation =
                    Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>();
                if cannot_refract || schlick_approximation {
                    directed = unit_direction.reflect(rec.normal);
                } else {
                    directed = unit_direction.refract(rec.normal, refraction_ratio);
                };
                let scattered = Ray::new(rec.p, directed);
                Some((self.color, scattered))
            }
        }
    }
}
