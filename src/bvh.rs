use ::std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::random::random;
use crate::ray::Ray;

#[derive(Debug)]
struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

impl BVHNode {
    pub fn new(hittables: &mut [Arc<dyn Hittable>], initial_time: f32, final_time: f32) -> Self {
        let axis = (3.0 * random()) as i32;

        let getter = |hittable: &Arc<dyn Hittable>| {
            hittable
                .bounding_box(0.0, 0.0)
                .expect("Non bounding box in BVHNode constructor")
                .min()
        };

        match axis {
            0 => hittables.sort_unstable_by(|hit1, hit2| {
                getter(hit1).x().partial_cmp(&getter(hit2).x()).unwrap()
            }),
            1 => hittables.sort_unstable_by(|hit1, hit2| {
                getter(hit1).y().partial_cmp(&getter(hit2).y()).unwrap()
            }),
            _ => hittables.sort_unstable_by(|hit1, hit2| {
                getter(hit1).z().partial_cmp(&getter(hit2).z()).unwrap()
            }),
        }

        let len = hittables.len();

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match len {
            1 => (hittables[0].clone(), hittables[0].clone()),
            2 => (hittables[0].clone(), hittables[1].clone()),
            _ => (
                Arc::new(Self::new(
                    &mut hittables[..(len / 2)],
                    initial_time,
                    final_time,
                )),
                Arc::new(Self::new(
                    &mut hittables[(len / 2)..],
                    initial_time,
                    final_time,
                )),
            ),
        };

        let box_left = left.bounding_box(initial_time, final_time);
        let box_right = right.bounding_box(initial_time, final_time);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in BVHNode constructor");
        }

        let aabb = AABB::surrounding_box(box_left.unwrap(), box_right.unwrap());

        Self { left, right, aabb }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            let hit_left = self.left.hit(ray, t_min, t_max);
            let hit_right = self.right.hit(ray, t_min, t_max);

            if hit_left.is_some() && hit_right.is_some() {
                let rec_left = hit_left.unwrap();
                let rec_right = hit_right.unwrap();
                let rec = if rec_left.t < rec_right.t {
                    rec_left
                } else {
                    rec_right
                };
                Some(rec)
            } else if hit_left.is_some() {
                hit_left
            } else if hit_right.is_some() {
                hit_right
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}
