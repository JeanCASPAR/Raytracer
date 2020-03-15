use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
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

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let aabb = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center - Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(aabb)
    }
}
