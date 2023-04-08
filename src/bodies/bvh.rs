use crate::bodies::collision::{Body, HitRecord, AABB};

use crate::point::Point;
use crate::raytracer::Ray;
pub struct BVH {
    left: Box<dyn Body>,
    right: Box<dyn Body>,
    aabb: AABB,
}

impl BVH {
    pub fn new(mut bodies: Vec<Box<dyn Body>>) -> Self {
        let aabb = AABB::from_bodies(&bodies).unwrap();
        let length = &bodies.len();
        match length {
            0 => panic!("got an empty world"),
            1 => BVH {
                left: bodies.pop().unwrap(),
                right: Box::new(NullBody {}),
                aabb,
            },
            2 => BVH {
                left: bodies.pop().unwrap(),
                right: bodies.pop().unwrap(),
                aabb,
            },
            _ => {
                let (left, right) = bodies.split_at_mut(length / 2);
                BVH {
                    left: BVH::new(left.to_vec()),
                    right: BVH::new(right.to_vec()),
                    aabb,
                }
            }
        }
    }
}

impl Body for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let new_t_max = match hit_left {
            true => rec.t,
            false => t_max,
        };
        let hit_right = self.right.hit(ray, t_min, new_t_max, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}

struct NullBody;

impl Body for NullBody {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(Point::new(0., 0., 0.), Point::new(0., 0., 0.))
    }
}
