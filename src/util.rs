use rand::{distributions::Standard, Rng, prelude::Distribution};
use std::time;
use std::mem::MaybeUninit;

pub fn default_vec<T>(n: usize) -> Vec<T>
where Standard: Distribution<T> {
    rand::thread_rng().sample_iter(Standard).take(n).collect()
}

pub fn is_sorted<T: PartialOrd>(v: &[T]) -> bool {
    (1..v.len()).all(|i| v[i - 1] <= v[i])
}

pub fn time_it(f: impl FnOnce()) -> u64 {
    let start = time::Instant::now();
    f();
    let dur = start.elapsed();
    let nanos = dur.subsec_nanos() as u64 + dur.as_secs() * 1_000_000_000u64;
    nanos
}

#[macro_export]
macro_rules! impl_rotate_n {
    ($func: ident, $n: expr) => {
        #[inline(always)]
        pub unsafe fn $func<T: Copy>(arr: &mut [T], idx: [usize; $n]) {
            let mut tmp: [MaybeUninit<T>; $n] = MaybeUninit::uninit().assume_init();
            
            // Initialize the temporary array with uninitialized memory
            for i in 0..$n {
                tmp[i] = MaybeUninit::new(arr[idx[i]]);
            }

            // Copy values back to the original array
            for i in 0..$n {
                arr[idx[i]] = tmp[(i + 1) % $n].assume_init();
            }
        }
    };
}

impl_rotate_n!(rotate3, 3);
impl_rotate_n!(rotate4, 4);
impl_rotate_n!(rotate5, 5);
impl_rotate_n!(rotate6, 6);