use crate::bodies::collision::HitRecord;
use crate::bodies::texture::Texture;
use crate::point::Point;
use crate::raytracer::Ray;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian,
    Dielectric(f64),
    Metal(f64),
    DiffuseLight,
    Ether,
}

#[derive(Clone, Copy, Debug)]
pub struct BodyProps {
    texture: Texture,
    material: Material,
}

impl BodyProps {
    #[allow(dead_code)]
    pub fn new(texture: Texture, material: Material) -> Self {
        BodyProps { texture, material }
    }
    pub fn metal(texture: Texture, fuzziness: f64) -> Self {
        BodyProps {
            texture,
            material: Material::Metal(fuzziness),
        }
    }

    pub fn matte(texture: Texture) -> Self {
        BodyProps {
            texture,
            material: Material::Lambertian,
        }
    }
    pub fn light(texture: Texture) -> Self {
        BodyProps {
            texture,
            material: Material::DiffuseLight,
        }
    }
    pub fn glass(index_refraction: f64) -> Self {
        BodyProps {
            texture: Texture::new_color(1., 1., 1.),
            material: Material::Dielectric(index_refraction),
        }
    }
    pub fn null() -> Self {
        BodyProps {
            texture: Texture::new_color(1., 1., 1.),
            material: Material::Ether,
        }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }

    pub fn color_emitted(&self, u: f64, v: f64, p: Point) -> Point {
        match self.material {
            Material::DiffuseLight => self.texture.color(u, v, p),
            _ => Point::default(),
        }
    }
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Point, Ray)> {
        match self.material {
            Material::Ether => None,
            Material::DiffuseLight => None,
            Material::Lambertian => {
                let mut scatter_direction = rec.normal + Point::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::new(rec.p, scatter_direction);
                Some((self.texture.color(rec.u, rec.v, rec.p), scattered))
            }
            Material::Metal(fuzziness) => {
                let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + fuzziness * Point::random_in_unit_sphere(),
                );
                Some((self.texture.color(rec.u, rec.v, rec.p), scattered))
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
                Some((self.texture.color(rec.u, rec.v, rec.p), scattered))
            }
        }
    }
}
