use crate::bodies::collision::HitRecord;
use crate::point::Point;
use crate::raytracer::Ray;

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
