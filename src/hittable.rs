use ::std::fmt::Debug;
use ::std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

#[derive(Default, Debug)]
pub struct Scene {
    pub hittables: Vec<Arc<dyn Hittable>>,
}

impl Scene {
    pub fn new(hittables: Vec<Arc<dyn Hittable>>) -> Self {
        Self { hittables }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for hittable in self.hittables.iter() {
            if let Some(rec) = hittable.hit(ray, t_min, closest_so_far) {
                hit_record = Some(rec);
                closest_so_far = hit_record.as_ref().map(|rec| rec.t).unwrap();
            }
        }
        hit_record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.hittables.is_empty() {
            return None;
        }

        let mut aabb = if let Some(aabb) = self.hittables[0].bounding_box(t0, t1) {
            aabb
        } else {
            return None;
        };

        for i in 1..self.hittables.len() {
            if let Some(temp_box) = self.hittables[i].bounding_box(t0, t1) {
                aabb = AABB::surrounding_box(aabb, temp_box);
            } else {
                return None;
            }
        }

        Some(aabb)
    }
}
