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

pub trait Body {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
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

    fn color(&self, ray: &Ray, angle: f64) -> Point {
        let n = (ray.at(angle) - Point::new(0., 0., -1.)).unit_vector();
        0.5 * Point::new(n.x + 1., n.y + 1., n.z + 1.)
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
    Metal,
    Ether,
}

#[derive(Debug, Clone, Copy)]
pub struct BodyProps {
    color: Point,
    material: Material,
    fuzziness: f64,
}

impl BodyProps {
    pub fn new(color: Point, material: Material, fuzziness: f64) -> Self {
        BodyProps {
            color,
            material,
            fuzziness,
        }
    }
    pub fn metal(color: Point, fuzziness: f64) -> Self {
        BodyProps {
            color,
            material: Material::Metal,
            fuzziness,
        }
    }

    pub fn lambertian(color: Point) -> Self {
        BodyProps {
            color,
            material: Material::Lambertian,
            fuzziness: 0.,
        }
    }
    pub fn null() -> Self {
        BodyProps {
            color: Point::default(),
            material: Material::Ether,
            fuzziness: 1.,
        }
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
            Material::Metal => {
                let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + self.fuzziness * Point::random_in_unit_sphere(),
                );
                Some((self.color, scattered))
            }
        }
    }
}
