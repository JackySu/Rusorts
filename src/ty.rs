use core::cmp::Ordering;
use core::ops::Deref;

use rand::distributions::Standard;
use rand::prelude::Distribution;

// This type is for internal use only within the crate
// not meant to be exposed to the pyo3 bindings
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Default)]
#[repr(transparent)]  // guarantees same layout as a single f32
pub struct FloatOrd(pub f32);

impl Deref for FloatOrd {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

macro_rules! impl_float_ord {
    ($($t:ty),*) => {
        $(
            impl From<$t> for FloatOrd {
                fn from(f: $t) -> Self {
                    FloatOrd(f as f32)
                }
            }

            impl From<FloatOrd> for $t {
                fn from(f: FloatOrd) -> Self {
                    f.0 as $t
                }
            }
        )*
    };
}

impl_float_ord!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64);

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
