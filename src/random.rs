use ::std::{
    f32,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

/// Implement Xoshiro256+ algorithm
pub struct XorShift32 {
    state: [u64; 4],
}

impl XorShift32 {
    pub fn new(seed: Option<u64>) -> Self {
        let mut seed = seed.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        let mut state = [0; 4];

        let mut tmp = Self::split_mix_64(&mut seed);
        state[0] = tmp;
        state[1] = tmp >> 32;

        tmp = Self::split_mix_64(&mut seed);
        state[2] = tmp;
        state[3] = tmp >> 32;

        Self { state }
    }

    pub fn linear(&mut self) -> f32 {
        let result = self.state[0].wrapping_add(self.state[3]);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = Self::rol_64(self.state[3], 45);

        (result as u64 >> 3) as f32 / (2u64.pow(61) - 1) as f32
    }

    fn rol_64(x: u64, k: u64) -> u64 {
        (x << k) | (x >> (64 - k))
    }

    fn split_mix_64(s: &mut u64) -> u64 {
        let mut result = *s;

        *s = result.wrapping_div(0x9E37_79B9_7F4A_7C15);
        result = (result ^ (result >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        result = (result ^ (result >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        result ^ (result >> 31)
    }

    #[allow(dead_code)]
    pub fn normal(&mut self) -> f32 {
        let u1 = self.linear();
        let u2 = self.linear();

        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * f32::consts::PI * u2).cos();
        // let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * f32::consts::PI * u2).sin();

        z0
    }
}

pub(crate) static mut RANDOM: Option<Arc<Mutex<XorShift32>>> = None;

pub fn random() -> f32 {
    unsafe {
        RANDOM
            .as_ref()
            .expect("You have to init the global RANDOM object with init_rand!")
            .lock()
            .unwrap()
            .linear()
    }
}

#[macro_export]
macro_rules! init_rand {
    () => {{
        unsafe {
            $crate::random::RANDOM =
                Some(Arc::new(Mutex::new($crate::random::XorShift32::new(None))));
        }
    }};
    ($seed: expr) => {{
        unsafe {
            $crate::random::RANDOM = Some(Arc::new(Mutex::new($crate::random::XorShift32::new(
                Some($seed),
            ))));
        }
    }};
}
