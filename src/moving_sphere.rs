use ::std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct MovingSphere {
    initial_center: Vec3,
    final_center: Vec3,
    initial_time: f32,
    final_time: f32,
    radius: f32,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        initial_center: Vec3,
        final_center: Vec3,
        initial_time: f32,
        final_time: f32,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            initial_center,
            final_center,
            initial_time,
            final_time,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.initial_center
            + (self.final_center - self.initial_center)
                * ((time - self.initial_time) / (self.final_time - self.initial_time))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> std::option::Option<HitRecord> {
        let oc = *ray.origin() - self.center(ray.time());
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    material: self.material.clone(),
                };
                return Some(rec);
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    material: self.material.clone(),
                };
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let aabb = AABB::surrounding_box(box0, box1);
        Some(aabb)
    }
}
