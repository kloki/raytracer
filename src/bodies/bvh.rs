use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::raytracer::Ray;
pub struct BVH {
    left: Box<dyn Body>,
    right: Box<dyn Body>,
    aabb: AABB,
}

impl BVH {
    pub fn new(left: Box<dyn Body>, right: Box<dyn Body>) -> Self {
        let aabb = left.bounding_box().surrounding_box(right.bounding_box());
        Self { left, right, aabb }
    }
}

impl Body for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }

        if self.left.hit(ray, t_min, t_max, rec) {
            self.right.hit(ray, t_min, t_max, rec);
        }
        true
    }
    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
