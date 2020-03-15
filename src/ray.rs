use crate::vec3::Vec3;

#[derive(Default, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub const fn origin(&self) -> Vec3 {
        self.origin
    }

    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    pub const fn time(&self) -> f32 {
        self.time
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
