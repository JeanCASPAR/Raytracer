use crate::random::random;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    initial_time: f32,
    final_time: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        initial_time: f32,
        final_time: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        let lower_left_corner = origin - (u * half_width + v * half_height + w) * focus_dist;
        let horizontal = u * 2.0 * half_width * focus_dist;
        let vertical = v * 2.0 * half_height * focus_dist;
        println!(
            "{:?} {:?} {:?} {:?}",
            lower_left_corner, horizontal, vertical, origin
        );
        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
            initial_time,
            final_time,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rand_vec = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rand_vec.x() + self.v * rand_vec.y();
        let time = self.initial_time + random() * (self.final_time - self.initial_time);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
            time,
        )
    }
}
