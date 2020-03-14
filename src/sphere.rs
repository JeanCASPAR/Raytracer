use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::Arc;

use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                };
                return Some(rec);
            }
            temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                };
                return Some(rec);
            }
        }
        None
    }
}

impl Debug for Sphere {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("Sphere")
            .field("center", &self.center)
            .field("radius", &self.radius)
            .finish()
    }
}
