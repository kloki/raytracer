use crate::point::Point;
use crate::raytracer::Ray;

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
}

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
    ) -> Self {
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
}

#[allow(dead_code)]
pub struct Cube {
    min: Point,
    max: Point,
    sides: [Rect; 6],
}

impl Cube {
    pub fn new(p0: Point, p1: Point, body_props: BodyProps) -> Self {
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
