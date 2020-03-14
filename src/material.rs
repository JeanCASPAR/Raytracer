use ::std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::random::random;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>; // Attenuation, scattered
}

/// Diffuse
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub const fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p, target - rec.p, ray.time());
        Some((attenuation, scattered))
    }
}

/// Reflect
#[derive(Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Self {
        let fuzz = f.min(1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().unit_vector().reflect(&rec.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            ray.time(),
        );

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

/// Refract
#[derive(Debug)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub const fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }

    fn schlick(&self, cosine: f32) -> f32 {
        let mut r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().reflect(&rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&rec.normal) > 0.0 {
            (
                -rec.normal,
                self.ref_idx,
                ray.direction().dot(&rec.normal) * self.ref_idx / ray.direction().length(),
            )
        } else {
            (
                rec.normal,
                self.ref_idx.recip(),
                -ray.direction().dot(&rec.normal) / ray.direction().length(),
            )
        };
        if let Some(refracted) = ray.direction().refract(&outward_normal, ni_over_nt) {
            if random() < self.schlick(cosine) {
                let scattered = Ray::new(rec.p, reflected, ray.time());
                Some((attenuation, scattered))
            } else {
                let scattered = Ray::new(rec.p, refracted, ray.time());
                Some((attenuation, scattered))
            }
        } else {
            Some((attenuation, Default::default()))
        }
    }
}
