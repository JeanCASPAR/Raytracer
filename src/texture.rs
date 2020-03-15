use ::std::fmt::Debug;
use ::std::sync::Arc;

use crate::perlin::Perlin;
use crate::vec3::Vec3;

pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    #[inline]
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    #[inline]
    pub fn color(&self) -> Vec3 {
        self.color
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color()
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sin = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sin < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 50.0 * self.noise.turb(p, 7)).sin())
    }
}
