#![deny(missing_debug_implementations)]

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
#[macro_use]
mod random;
mod chunk;

use std::path::Path;
use std::sync::{Arc, Mutex, mpsc::{channel, TryRecvError}};
use std::time::{Duration, Instant};
use std::thread::sleep;

use image::{ImageBuffer, ImageFormat, Rgb};
use minifb::{Key, Window, WindowOptions};
use threadpool::Builder;

use camera::Camera;
use hitable::{Hitable, Scene};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use random::random;
use chunk::Chunk;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const RAY_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;
const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const CHUNK_WIDTH: usize = 50;
const CHUNK_HEIGHT: usize = 50;
const NB_WORKERS: usize = 10;

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

pub fn to_rgb(mut color: u32) -> (u8, u8, u8) {
    let r = color >> 16;
    color ^= r << 16;
    let g = color >> 8;
    color ^= g << 8;
    let b = color;
    (r as u8, g as u8, b as u8)
}

fn main() {
    init_rand!();
    let buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
    println!("{}", buffer.lock().unwrap()[199 + 1 * WIDTH]);

    let mut window = Window::new("Raytracer", WIDTH, HEIGHT, WindowOptions::default())
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    
    let scene = Arc::new(random_scene());
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        UP,
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        10.0,
    ));

    let time = Instant::now();

    let thread_pool = Builder::new()
        .num_threads(NB_WORKERS)
        .build();
    let (tx, rx) = channel();
    println!("{} {}", (WIDTH as f32 / CHUNK_WIDTH as f32).ceil() as usize, (HEIGHT as f32 / CHUNK_HEIGHT as f32).ceil() as usize);
    for j in 0..((HEIGHT as f32 / CHUNK_HEIGHT as f32).ceil() as usize) {
        for i in 0..((WIDTH as f32 / CHUNK_WIDTH as f32).ceil() as usize) {
            let chunk = Chunk::new(CHUNK_WIDTH, CHUNK_HEIGHT, i * CHUNK_WIDTH, j * CHUNK_HEIGHT, Arc::clone(&buffer), Arc::clone(&camera), Arc::clone(&scene));
            let tx = tx.clone();

            thread_pool.execute(move || {
                println!("begin {} {}", i, j);
                chunk.process();
                println!("end {} {}", i, j);
                tx.send((i, j)).unwrap();
            });
        }
    }

    drop(tx);

    let mut k = 0;

    loop {
        match rx.try_recv() {
            Ok((i, j)) => {
                k += 1;
                println!("pass number {}", k);
                println!("begin render {} {}", i, j);
                window.update_with_buffer(&*buffer.lock().unwrap(), WIDTH, HEIGHT).unwrap();
                println!("end render {} {}", i, j);
                continue
            },
            Err(TryRecvError::Disconnected) => break,
            _ => {
                window.update_with_buffer(&*buffer.lock().unwrap(), WIDTH, HEIGHT).unwrap();
                sleep(Duration::from_millis(100));
                if !window.is_open() {
                    panic!("Window closed!");
                }
                continue
            },
        };
    }

    println!("number of passes: {}", k);

    thread_pool.join();

    println!("It took {:?}", time.elapsed());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&*buffer.lock().unwrap(), WIDTH, HEIGHT).unwrap();
        if window.is_key_down(Key::S) {
            // Save
            let bytes_buffer = buffer
                .lock().unwrap()
                .iter()
                .flat_map(|color| {
                    let (r, g, b) = to_rgb(*color);
                    vec![r, g, b]
                })
                .collect::<Vec<u8>>();
            let img_buffer =
                ImageBuffer::<Rgb<u8>, _>::from_vec(WIDTH as u32, HEIGHT as u32, bytes_buffer)
                    .unwrap();
            img_buffer
                .save_with_format(&Path::new("./image.png"), ImageFormat::Png)
                .unwrap();

            println!("Image saved!")
        }
    }
}

#[allow(dead_code)]
fn random_scene() -> Scene {
    let n = 500;
    let mut list: Vec<Arc<dyn Hitable>> = Vec::with_capacity(n + 1);
    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(
                a as f32 + 0.9 * random(),
                0.2,
                b as f32 + 0.9 * random(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    list.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Vec3::new(
                            random() * random(),
                            random() * random(),
                            random() * random(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    list.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + random()),
                                0.5 * (1.0 + random()),
                                0.5 * (1.0 + random()),
                            ),
                            0.5 * (1.0 + random()),
                        )),
                    )));
                } else {
                    // Glass
                    list.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    list.push(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.push(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    Scene::new(list)
}