// enable portable simd feature to use simd in nightly build
#![feature(portable_simd)]
#![feature(slice_swap_unchecked)]

extern crate rand;
extern crate introsort;
extern crate pdqsort;

pub mod util;
pub mod test;
pub mod qsort;
pub mod ty;

use ty::FloatOrd;
use qsort::*;
use util::*;


#[no_mangle]
pub unsafe extern "C" fn f32_std_sort(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	let v: &mut [FloatOrd] = std::mem::transmute(v);
	time_it(|| v.sort())
}

#[no_mangle]
pub unsafe extern "C" fn f32_introsort(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	let v: &mut [FloatOrd] = std::mem::transmute(v);
	time_it(|| introsort::sort(v))
}

#[no_mangle]
pub unsafe extern "C" fn f32_pdqsort(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	let v: &mut [FloatOrd] = std::mem::transmute(v);
	time_it(|| pdqsort::sort(v))
}

#[no_mangle]
pub unsafe extern "C" fn f32_unstable_sort(v: *mut f32, size: usize) -> u64 {
	// std lib's unstable sort uses PDQSort under the hood, but slower than the community version
	let v = std::slice::from_raw_parts_mut(v, size);
	time_it(|| v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()))
}

#[no_mangle]
pub unsafe extern "C" fn f32_quicksort_hoare(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	time_it(|| quick_sort_hoare_partition(v))
}

#[no_mangle]
pub unsafe extern "C" fn f32_quicksort_lomuto(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	time_it(|| quick_sort_lomuto_partition(v))
}

#[no_mangle]
pub unsafe extern "C" fn f32_double_pivot_quicksort(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	time_it(|| double_pivot_quicksort(v))
}

#[no_mangle]
pub unsafe extern "C" fn f32_triple_pivot_quicksort(v: *mut f32, size: usize) -> u64 {
	let v = std::slice::from_raw_parts_mut(v, size);
	time_it(|| triple_pivot_quicksort(v))
}
