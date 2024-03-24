use rand::{distributions::Standard, Rng, prelude::Distribution};
use std::time;

use pyo3::{prelude::*, types::PyList};
use core::ffi::c_char;

use crate::ty::FloatOrd;

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

pub unsafe fn get_type_str(v: &PyAny) -> &str {
    let typ = (*(*v.as_ptr()).ob_type).tp_name as *const c_char;
    core::ffi::CStr::from_ptr(typ).to_str().unwrap()
}

pub fn pylist_to_ord_vec(v: &PyList) -> PyResult<Vec<i32>> {
    // get object from first element or return an empty Vec<i32>
    let item = v.get_item(0);
    let obj = match item {
        Ok(obj) => obj,
        Err(_) => return Ok(Vec::new())
    };
    let typ = unsafe { get_type_str(obj) };
    match typ {
        "int" => v.extract::<Vec<i32>>(),
        "float" => v.extract::<Vec<f32>>()
            .map(|v| v
                .iter()
                .map(|x| {
                    let f = unsafe { core::mem::transmute::<f32, i32>(*x) };
                    if f < 0 { f ^ 0x7fffffff } else { f }
                })
                .collect()
            ),
        _ => Err(pyo3::exceptions::PyTypeError::new_err("unsupported type"))
    }
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
                std::ptr::copy_nonoverlapping(&arr[idx[i]], &mut arr[idx[i - 1]], 1);
            }
            std::ptr::copy_nonoverlapping(&tmp, &mut arr[idx[$n - 1]], 1);
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