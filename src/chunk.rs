use std::sync::{Arc, Mutex};

use super::{HEIGHT, MAX_DEPTH, RAY_PER_PIXEL, WIDTH};

use crate::camera::Camera;
use crate::hittable::Scene;
use crate::random::random;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn color(ray: Ray, scene: &Scene, depth: usize) -> Vec3 {
    if let Some(rec) = scene.hit(&ray, 0.001, std::f32::MAX) {
        if let Some((attenuation, scattered)) = if depth < MAX_DEPTH {
            rec.material.scatter(&ray, &rec)
        } else {
            None
        } {
            attenuation * color(scattered, &scene, depth + 1)
        } else {
            Default::default()
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

pub fn from_rgb(r: f32, g: f32, b: f32) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[allow(dead_code)]
pub fn to_rgb(mut color: u32) -> (u8, u8, u8) {
    let r = color >> 16;
    color ^= r << 16;
    let g = color >> 8;
    color ^= g << 8;
    let b = color;
    (r as u8, g as u8, b as u8)
}

pub struct Chunk {
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub buffer: Arc<Mutex<Vec<u32>>>,
    camera: Arc<Camera>,
    scene: Arc<Scene>,
}

impl Chunk {
    pub fn new(
        width: usize,
        height: usize,
        offset_x: usize,
        offset_y: usize,
        buffer: Arc<Mutex<Vec<u32>>>,
        camera: Arc<Camera>,
        scene: Arc<Scene>,
    ) -> Self {
        Self {
            width,
            height,
            offset_x,
            offset_y,
            buffer,
            camera,
            scene,
        }
    }

    pub fn process(self) {
        for j in self.offset_y..(self.offset_y + self.height) {
            for i in self.offset_x..(self.offset_x + self.width) {
                let mut pixel_color = Vec3::default();

                for _ in 0..RAY_PER_PIXEL {
                    let u = (i as f32 + random()) / WIDTH as f32;
                    let v = ((HEIGHT - j) as f32 + random()) / HEIGHT as f32;

                    let ray = self.camera.get_ray(u, v);
                    pixel_color += color(ray, &*self.scene, 0);
                }
                pixel_color /= RAY_PER_PIXEL as f32;
                pixel_color = Vec3::new(
                    pixel_color.x().sqrt(),
                    pixel_color.y().sqrt(),
                    pixel_color.z().sqrt(),
                );

                let Vec3 { data: [r, g, b] } = pixel_color * 255.99;
                if let Some(index) = self.buffer.lock().unwrap().get_mut(i + j * WIDTH) {
                    *index = from_rgb(r, g, b);
                } else {
                    #[cfg(debug_assertions)]
                    eprintln!("{}, {}, {}, {}", i, j, self.offset_x, self.offset_y);
                }
            }
        }
        #[cfg(debug_assertions)]
        eprintln!("finished");
    }
}
