use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::raytracer::Ray;
pub struct BVH<'a> {
    left: &'a dyn Body,
    right: &'a dyn Body,
    aabb: AABB,
}

impl<'a> BVH<'a> {
    pub fn new(bodies: Vec<&'a dyn Body>) -> Self {
        let aabb = AABB::from_bodies(&bodies).unwrap();
        let lenght = &bodies.len();
        match lenght {
            0 => panic!("got an empty world"),
            1 => BVH {
                left: bodies[0],
                right: bodies[0],
                aabb,
            },
            2 => BVH {
                left: bodies[0],
                right: bodies[1],
                aabb,
            },
            _ => panic!("not implemented"),
        }
    }
}

impl<'a> Body for BVH<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self.right.hit(ray, t_min, t_max, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
