use core::cmp::Ordering;
use core::cmp::Ord;
use std::ops::Deref;

use rand::distributions::Standard;
use rand::prelude::Distribution;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct FloatOrd(f32);

impl Deref for FloatOrd {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

// compiler defines
impl Eq for FloatOrd {}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        #[inline]
        fn f32_bits(a: f32) -> i32 { unsafe { std::mem::transmute(a) } }
        // ideally this would be replaced by a totalorder primitive when that's available
        let mut a = f32_bits(self.0);
        let mut b = f32_bits(other.0);
        if a < 0 { a ^= 0x7fffffff; }
        if b < 0 { b ^= 0x7fffffff; }
        a.cmp(&b)
    }
}

// for rng sampling to generate random floats
impl Distribution<FloatOrd> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> FloatOrd {
        FloatOrd(rng.gen())
    }
}
