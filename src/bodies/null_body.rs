use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::point::Point;
use crate::raytracer::Ray;
pub struct NullBody;
impl Body for NullBody {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(Point::new(0., 0., 0.), Point::new(0., 0., 0.))
    }
}
