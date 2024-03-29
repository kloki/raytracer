use crate::bodies::collision::{Body, HitRecord, AABB};
use crate::ordered_float::OrderedFloat;
use crate::point::Point;
use crate::raytracer::Ray;
use rand::Rng;
#[derive(Debug)]
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
                let mut rng = rand::thread_rng();
                match rng.gen_range(0..3) {
                    0 => bodies.sort_unstable_by_key(|b| OrderedFloat(b.bounding_box().min.x)),
                    1 => bodies.sort_unstable_by_key(|b| OrderedFloat(b.bounding_box().min.y)),
                    _ => bodies.sort_unstable_by_key(|b| OrderedFloat(b.bounding_box().min.z)),
                }

                let left = bodies.drain(0..(length / 2)).collect();
                BVH {
                    left: Box::new(BVH::new(left)),
                    right: Box::new(BVH::new(bodies)),
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

#[derive(Debug)]
struct NullBody;

impl Body for NullBody {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(Point::new(0., 0., 0.), Point::new(0., 0., 0.))
    }
}
