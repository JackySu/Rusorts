use rand::{distributions::Standard, Rng, prelude::Distribution};
use std::time;

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
macro_rules! sort_ordnum_f {
    (trait, $v: expr, $func: ident, $py: ident) => {
        match $v {
            OrdNum::Int(mut v) => {
                let t = time_it(|| (&mut v).$func());
                let v = PyList::new($py, v);
                Ok((v, t))
            }
            OrdNum::Float(mut v) => {
                let t = time_it(|| (&mut v).$func());
                let v = PyList::new($py, v);
                Ok((v, t))
            }
        }
    };
    (slice, $v: expr, $func: ident, $py: ident) => {
        match $v {
            OrdNum::Int(mut v) => {
                let t = time_it(|| $func(&mut v));
                let v = PyList::new($py, v);
                Ok((v, t))
            }
            OrdNum::Float(mut v) => {
                let t = time_it(|| $func(&mut v));
                let v = PyList::new($py, v);
                Ok((v, t))
            }
        }
    };
}

/* 
 * the generic rotate_left function can be used in place of this macro
 * but when it comes to circumstances that the idx array isn't ascending
 * like when putting pivots back to their original positions (marked in the quicksort algorithm)
 * it could lead to a panic
*/
#[macro_export]
macro_rules! impl_rotate_n {
    ($func: ident, $n: expr) => {
        #[inline(always)]
        pub unsafe fn $func<T>(arr: &mut [T], idx: [usize; $n]) {
            // cycle the elements in the idx array
            let tmp = std::ptr::read(&arr[idx[0]]);
            for i in 1..$n {
                std::ptr::copy_nonoverlapping(arr.get_unchecked(idx[i]), arr.get_unchecked_mut(idx[i - 1]), 1);
            }
            std::ptr::copy_nonoverlapping(&tmp, arr.get_unchecked_mut(idx[$n - 1]), 1);
        }
    };
}

impl_rotate_n!(rotate3, 3);
impl_rotate_n!(rotate4, 4);
impl_rotate_n!(rotate5, 5);
impl_rotate_n!(rotate6, 6);

/*
 * if you would like to try out the generic rotate_left function
 * you can use this macro, but be aware of the part that could lead to a panic
 * which is marked in the quicksort algorithm
 */
#[macro_export]
macro_rules! rotate_n {
    ($array:expr, [$($indices:expr),*]) => {
        {
            let mut temp = [$($array[$indices]),*];
            temp.rotate_left(1);
            $(
                $array[$indices] = temp[$indices];
            )*
        }
    };
}