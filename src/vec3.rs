use std::ops;

use crate::random::random;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    pub data: [f32; 3],
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }

    #[inline]
    pub const fn x(&self) -> f32 {
        self.data[0]
    }

    #[inline]
    pub const fn y(&self) -> f32 {
        self.data[1]
    }

    #[inline]
    pub const fn z(&self) -> f32 {
        self.data[2]
    }

    #[inline]
    pub const fn r(&self) -> f32 {
        self.data[0]
    }

    #[inline]
    pub const fn g(&self) -> f32 {
        self.data[1]
    }

    #[inline]
    pub const fn b(&self) -> f32 {
        self.data[2]
    }

    #[inline]
    pub fn squared_length(&self) -> f32 {
        self.dot(&self)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn make_unit_vector(&mut self) {
        *self = self.unit_vector();
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.data[0] * rhs.data[0] + self.data[1] * rhs.data[1] + self.data[2] * rhs.data[2]
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        )
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let vec =
                Self::new(random(), random(), 0.0) * 2.0 - Self::new(1.0, 1.0, 0.0);
            if vec.squared_length() < 1.0 {
                break vec;
            }
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vec = Self::new(random(), random(), random()) * 2.0
                - Self::new(1.0, 1.0, 1.0);
            if vec.squared_length() < 1.0 {
                break vec;
            }
        }
    }

    pub fn reflect(&self, other: &Self) -> Self {
        return *self - *other * 2.0 * self.dot(other);
    }

    pub fn refract(&self, other: &Self, ni_overt_nt: f32) -> Option<Self> {
        let uv = self.unit_vector();
        let dt = uv.dot(other);
        let discriminant = 1.0 - ni_overt_nt * ni_overt_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some((uv - *other * dt) * ni_overt_nt - *other * discriminant.sqrt())
        } else {
            None
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.data[0], -self.data[1], -self.data[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
    }
}

impl ops::MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
        self.data[2] *= rhs.data[2];
    }
}

impl ops::DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
        self.data[2] /= rhs.data[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        let invert_rhs = rhs.recip(); // Save compute time, as division being expansive

        *self *= invert_rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2],
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.data[0] * rhs.data[0],
            self.data[1] * rhs.data[1],
            self.data[2] * rhs.data[2],
        )
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.data[0] / rhs.data[0],
            self.data[1] / rhs.data[1],
            self.data[2] / rhs.data[2],
        )
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let invert_rhs = rhs.recip(); // Save compute time, as division being expansive

        self * invert_rhs
    }
}
