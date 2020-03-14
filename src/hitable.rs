use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("HitRecord")
            .field("t", &self.t)
            .field("p", &self.p)
            .field("normal", &self.normal)
            .field("material", &"something")
            .finish()
    }
}

#[derive(Default)]
pub struct Scene {
    pub hitables: Vec<Arc<dyn Hitable>>,
}

impl Scene {
    pub fn new(hitables: Vec<Arc<dyn Hitable>>) -> Self {
        Self { hitables }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for hitable in self.hitables.iter() {
            if let Some(rec) = hitable.hit(ray, t_min, closest_so_far) {
                hit_record = Some(rec);
                closest_so_far = hit_record.as_ref().map(|rec| rec.t).unwrap();
            }
        }
        hit_record
    }
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("Scene")
            .field("hitables", &self.hitables.len())
            .finish()
    }
}
