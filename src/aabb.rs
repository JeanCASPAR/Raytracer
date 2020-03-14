use ::std::mem::swap;

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_direction = ray.direction()[a].recip();
            let mut t0 = (self.min()[a] - ray.origin()[a]) * inv_direction;
            let mut t1 = (self.max()[a] - ray.origin()[a]) * inv_direction;
            if inv_direction < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t0.min(t_min);
            t_max = t1.max(t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vec3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let big = Vec3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );
        AABB::new(small, big)
    }
}
