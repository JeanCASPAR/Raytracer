use ::std::fmt::Debug;
use ::std::sync::Arc;

use crate::vec3::Vec3;

pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Vec3
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