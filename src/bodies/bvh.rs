use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::raytracer::Ray;
pub struct BVH {
    left: Box<dyn Body>,
    right: Box<dyn Body>,
    aabb: AABB,
}

impl BVH {
    pub fn new(bodies: Vec<Box<dyn Body>>) -> Self {
        let aabb = AABB::from_bodies(&bodies).unwrap();
        let lenght = &bodies.len();
        match lenght {
            0 => panic!("got an empty world"),
            1 => Self {
                left: bodies[0],
                right: bodies[0],
                aabb,
            },
            2 => panic!("got an empty world"),
            _ => panic!("not implemented"),
        }
    }
}

impl Body for BVH {
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
