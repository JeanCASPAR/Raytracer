use ::std::ptr::swap;

use crate::random::random;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Perlin {
    random_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            random_vec: Self::perlin_generate(),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Default::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.random_vec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize]
                }
            }
        }

        Self::perlin_interpolation(&c, u, v, w)
    }

    fn perlin_generate() -> Vec<Vec3> {
        let mut p = Vec::with_capacity(256);
        for _ in 0..256 {
            p.push(
                Vec3::new(
                    2.0 * random() - 1.0,
                    2.0 * random() - 1.0,
                    2.0 * random() - 1.0,
                )
                .unit_vector(),
            );
        }
        p
    }

    fn permute(p: &mut [i32]) {
        for i in (0..p.len()).rev() {
            let target = (random() * (i as f32 + 1.0)) as usize;
            unsafe {
                swap(&mut p[i], &mut p[target]);
            }
        }
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i as i32);
        }
        Self::permute(&mut p);
        p
    }

    #[inline]
    #[allow(dead_code)]
    fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1.0 - i as f32) * (1.0 - u))
                        * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                        * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }

    #[inline]
    fn perlin_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_vector = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - vv))
                        * c[i][j][k].dot(&weight_vector);
                }
            }
        }

        accum
    }

    /// Default depth is 7
    pub fn turb(&self, p: &Vec3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp);
            weight *= 0.5;
            temp *= 2.0;
        }

        accum.abs()
    }
}
