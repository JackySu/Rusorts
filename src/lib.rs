extern crate rand;
extern crate introsort;
extern crate pdqsort;

pub mod util;
pub mod test;
pub mod qsort;
pub mod ty;

use ty::FloatOrd;
use qsort::{quick_sort, double_pivot_quicksort, triple_pivot_quicksort};


#[no_mangle]
pub unsafe extern "C" fn f32_std_sort(v: *mut f32, size: usize) {
	let v = unsafe { std::slice::from_raw_parts_mut(v, size) };
	v.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn f32_introsort(v: *mut f32, size: usize) {
	let v = std::slice::from_raw_parts_mut(v, size);
	let v: &mut [FloatOrd] = std::mem::transmute(v);
	introsort::sort(v);
}

#[no_mangle]
pub unsafe extern "C" fn f32_pdqsort(v: *mut f32, size: usize) {
	let v = std::slice::from_raw_parts_mut(v, size);
	let v: &mut [FloatOrd] = std::mem::transmute(v);
	pdqsort::sort(v);
}

#[no_mangle]
pub unsafe extern "C" fn f32_unstable_sort(v: *mut f32, size: usize) {
	let v = std::slice::from_raw_parts_mut(v, size);
	v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn f32_quicksort(v: *mut f32, size: usize) {
	let v = std::slice::from_raw_parts_mut(v, size);
	quick_sort(v);
}

#[no_mangle]
pub unsafe extern "C" fn f32_double_pivot_quicksort(v: *mut f32, size: usize) {
	let v = std::slice::from_raw_parts_mut(v, size);
	double_pivot_quicksort(v);
}

#[no_mangle]
pub unsafe extern "C" fn f32_triple_pivot_quicksort(v: *mut f32, size: usize) {
	let v = std::slice::from_raw_parts_mut(v, size);
	triple_pivot_quicksort(v);
}
